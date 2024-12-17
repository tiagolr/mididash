use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::to_writer_pretty;
use serde_json::Value;
use tauri::AppHandle;
use tauri::Emitter;
use tauri::Manager;
use tauri_plugin_store::StoreExt;
use tokio::runtime::Runtime;
use std::fs::File;
use std::io;
use std::sync::Mutex;
use std::sync::OnceLock;
use lazy_static::lazy_static;

use crate::devices::delay::Delay;
use crate::devices::device::Device;
use crate::devices::input::Input;
use crate::devices::mapper::Mapper;
use crate::devices::monitor::Monitor;
use crate::devices::output::Output;
use crate::devices::script::Script;
use crate::devices::splitter::Splitter;
use crate::devices::trigger::Trigger;
#[cfg(not(windows))]
use crate::devices::virtual_c::VirtualC;
use crate::globals::EVT_ERROR;
use crate::globals::EVT_SETTINGS_CHANGE;
use crate::hub::Connector;
use crate::hub::Hub;
use crate::utils;
use crate::Settings;
use crate::State;
use crate::SETTINGS_FILE;

pub static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();
pub fn get_app<'a>() -> Option<&'a AppHandle> {
    APP_HANDLE.get()
}

// Initialize the Tokio runtime statically, using lazy_static
lazy_static! {
    pub static ref TOKIO_RUNTIME: Mutex<Runtime> = Mutex::new(Runtime::new().unwrap());
}

// Project file frontend settings
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Project {
    pub version: Value,
    pub preferences: Value,
    pub devices: Vec<Value>,
    pub connectors: Vec<Value>,
}

pub fn emit<T: Serialize + Clone>(event: &str, payload: T) {
    if let Some(app) = get_app() {
        app.emit(event, payload).unwrap_or_else(|e|
            eprintln!("failed to emit {} {:?}", event, e)
        );
    }
}

pub fn emit_error(error: &str) {
    eprintln!("ERROR: {}", error);
    emit(EVT_ERROR, json!(error));
}

pub fn get_settings() -> Settings {
    if let Some(app) = get_app() {
        let store = app.store(SETTINGS_FILE).unwrap();
        let setts = store.get("settings").unwrap();
        serde_json::from_value(setts).expect("Unexpected settings file")
    } else {
        Settings::default()
    }
}

pub fn get_current_project() -> Project {
    let app = get_app().unwrap();
    let store = app.store(SETTINGS_FILE).unwrap();
    let project = store.get("project").unwrap();
    serde_json::from_value(project).expect("Unexpecting project file")
}

pub fn save_project_file() -> Result<(), Box<dyn std::error::Error>> {
    let settings = get_settings();
    if settings.project_path.is_empty() {
        return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "No project path found")));
    }
    let project = get_current_project();
    let file = File::create(&settings.project_path)?;
    to_writer_pretty(file, &project).map_err(|e| format!("Failed to save file: {}", e))?;
    Ok(())
}

pub fn set_settings(settings: Settings) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(app) = get_app() {
        let store = app.store(SETTINGS_FILE).unwrap();
        store.set(String::from("settings"), serde_json::to_value(settings)?);
        emit(EVT_SETTINGS_CHANGE, json!(null))
    }
    Ok(())
}

pub fn get_state() -> State {
    if let Some(app) = get_app() {
        let state = app.state::<Mutex<State>>();
        let state = state.lock().unwrap();
        state.clone()
    } else {
        State::default()
    }
}

pub fn set_project_path(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut settings = get_settings();
    settings.project_path = path.to_string();
    set_settings(settings)?;
    Ok(())
}

/**
 */
pub fn save_current_project(project: Project) -> Result<(), Box<dyn std::error::Error>> {
    let app = get_app().unwrap();
    let store = app.store(SETTINGS_FILE).unwrap();
    store.set("project", serde_json::to_value(project)?);
    Ok(())
}

pub fn new_empty_project() -> Result<(), Box<dyn std::error::Error>> {
    let hub_instance = Hub::get_instance();
    let mut hub = hub_instance.lock().unwrap();
    hub.destroy();

    Ok(())
}

pub fn set_hub_paused(paused: bool) -> Result<(), Box<dyn std::error::Error>> {
    let hub_instance = Hub::get_instance();
    let mut hub = hub_instance.lock().unwrap();
    hub.set_paused(paused);
    let mut settings = get_settings();
    settings.hub_paused = paused;
    set_settings(settings)?;
    Ok(())
}

pub fn new_devices_project() -> Result<(), Box<dyn std::error::Error>> {
    let hub_instance = Hub::get_instance();
    let mut hub = hub_instance.lock().unwrap();
    hub.destroy();

    let ports = utils::get_valid_midi_ports()?;

    for name in ports.inputs {
        let mut input = Input::new(&name);
        input.init()?;
        hub.add_device(Box::new(input));
    }

    for name in ports.outputs {
        let mut output = Output::new(&name);
        output.init()?;
        hub.add_device(Box::new(output));
    }

    Ok(())
}
/**
 * Creates devices and connectors from project file
 */
pub fn load_project(project: Project) -> Result<(), Box<dyn std::error::Error>> {
    let hub_instance = Hub::get_instance();
    let mut hub = hub_instance.lock().unwrap();
    hub.destroy();

    let mut devices = project.devices.clone();
    // sort devices such that virtual devices are added first
    devices.sort_by(|a, b| {
        let a_is_virtual = a.get("class").and_then(Value::as_str) == Some("virtual");
        let b_is_virtual = b.get("class").and_then(Value::as_str) == Some("virtual");
        b_is_virtual.cmp(&a_is_virtual)
    });

    for d in devices {
        let id = d.get("id").and_then(Value::as_str).unwrap_or_default();
        let class = d.get("class").and_then(Value::as_str).unwrap_or_default();
        match class {
            "input" => {
                let mut input = Input::new(id);
                input.init().or_else(|_| -> Result<(), Box<dyn std::error::Error>> {
                    input.destroy();
                    Ok(())
                })?;
                hub.add_device(Box::new(input));
            },
            "output" => {
                let mut output = Output::new(id);
                output.init().or_else(|_| -> Result<(), Box<dyn std::error::Error>> {
                    output.destroy();
                    Ok(())
                })?;
                hub.add_device(Box::new(output));
            },
            #[cfg(not(windows))]
            "virtual" => {
                let mut virtual_c = VirtualC::new(id);
                virtual_c.init().and_then(|_| {
                    hub.add_device(Box::new(virtual_c));
                    Ok(())
                })?;
            },
            "monitor" => {
                let monitor = Monitor::new(id);
                hub.add_device(Box::new(monitor));
            },
            "split" => {
                let splitter = Splitter::new(id);
                hub.add_device(Box::new(splitter));
            },
            "map" => {
                let mut mapper = Mapper::new(id);
                mapper.set_data("rule".to_string(), d.get("rule").unwrap().clone())?;
                hub.add_device(Box::new(mapper));
            },
            "delay" => {
                let mut delay = Delay::new(id);
                delay.set_data("delay".to_string(), d.get("delay").unwrap().clone())?;
                hub.add_device(Box::new(delay));
            },
            "trigger" => {
                let trigger = Trigger::new(id);
                hub.add_device(Box::new(trigger));
            },
            "note" => {},
            "script" => {
                let mut script = Script::new(id);
                script.init()?;
                let empty = json!("");
                let code = d.get("script").unwrap_or(&empty);
                script.set_data("script".to_string(), code.clone())?;
                hub.add_device(Box::new(script));
            }
            _ => {
                eprintln!("Load file invalid device class {}", class);
            }
        }
    }

    for connector in project.connectors {
        if let Ok(c) = serde_json::from_value::<Connector>(connector.clone()) {
            hub.connect(&c.from, &c.to, &c.from_port, &c.to_port);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use crate::utils;

    use super::*;

    #[test]
    #[serial]
    fn new_empty_project () {
        super::new_empty_project().expect("");
        let hub_instance = Hub::get_instance();
        let hub = hub_instance.lock().unwrap();
        let devices = hub.serialize_devices().expect("");
        let connectors = hub.serialize_connectors().expect("");

        assert_eq!(devices.len(), 0);
        assert_eq!(connectors.len(), 0);
    }

    #[test]
    #[serial]
    fn new_devices_project () {
        let ports = utils::get_valid_midi_ports().expect("");
        super::new_devices_project().expect("");
        let hub_instance = Hub::get_instance();
        let hub = hub_instance.lock().unwrap();
        let devices = hub.serialize_devices().expect("");
        assert_eq!(devices.len(), ports.inputs.len() + ports.outputs.len());
    }
}