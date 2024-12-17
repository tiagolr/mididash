use crate::devices::delay::Delay;
use crate::devices::script::Script;
use crate::devices::trigger::Trigger;
use crate::{app, utils};
use std::fs;
use crate::devices::{device::Device, input::Input, mapper::Mapper, monitor::Monitor, output::Output, splitter::Splitter};
use app::Project;
use tauri_plugin_store::StoreExt;
use crate::globals::EVT_PROJECT_NEW;
use crate::hub::{Connector, Hub};
use serde_json::{json, Value};
use crate::Settings;
use crate::SETTINGS_FILE;
#[cfg(not(windows))]
use crate::devices::virtual_c::VirtualC;

#[tauri::command]
pub fn new_devices_project() -> Result<(), String> {
    app::new_devices_project()
        .and_then(|_| {
            let hub_instance = Hub::get_instance();
            let hub = hub_instance.lock().unwrap();
            let devices = hub.serialize_devices()?;
            let connectors = hub.serialize_connectors()?;
            let mut project = Project::default();
            project.devices = devices;
            project.connectors = connectors.into_iter()
                .map(|c| serde_json::to_value(c).expect("Connector serialization failed"))
                .collect();
            app::save_current_project(project)?;
            app::set_project_path("")?;
            app::emit(EVT_PROJECT_NEW, json!(null));
            Ok(())
        })
        .map_err(|err| {
            app::emit_error(&format!("Failed to create project {}", err));
            format!("Failed to set project path: {}", err)
        })?;
    Ok(())
}

#[tauri::command]
pub fn new_blank_project() -> Result<(), String> {
    app::new_empty_project()
        .and_then(|_| {
            app::save_current_project(Project::default())?;
            app::set_project_path("")?;
            app::emit(EVT_PROJECT_NEW, json!(null));
            Ok(())
        })
        .map_err(|err| {
            app::emit_error(&format!("Failed to create blank project {}", err));
            format!("Failed to create blank project: {}", err)
        })?;
    Ok(())
}

#[tauri::command]
pub fn get_settings() -> Settings {
    let settings = app::get_settings();
    return settings;
}

#[tauri::command]
pub fn set_settings(settings: Settings) -> Result<(), String> {
    app::set_settings(settings).or_else(|_| Err("Failed to set settings".to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn connect(from: String, to: String, from_port: String, to_port: String) -> Result<Connector, String> {
    let hub_instance = Hub::get_instance();
    let mut hub = hub_instance.lock().unwrap();
    let connector = hub.connect(&from, &to, &from_port, &to_port);
    if let Some(connector) = connector {
        Ok(connector)
    } else {
        Err("Duplicate connector".to_string())
    }
}

#[tauri::command]
pub fn set_hub_paused(paused: bool) -> Result<(), String> {
    app::set_hub_paused(paused).map_err(|err| format!("{}", err))?;
    Ok(())
}

#[tauri::command]
pub fn disconnect(from: String, to: String, from_port: String, to_port: String) -> Result<(), String> {
    let hub_instance = Hub::get_instance();
    let mut hub = hub_instance.lock().unwrap();
    if !hub.disconnect(&from, &to, &from_port, &to_port) {
        Err("Connector not found".to_string())
    } else {
        Ok(())
    }
}

#[tauri::command]
pub fn save_current_project(project: Project) -> Result<Value, String> {
    app::save_current_project(project).or_else(|e| Err(format!("Failed to save current project {:?}", e)))?;
    Ok(json!(true))
}

#[tauri::command]
pub fn save_project_file() -> Result<Value, String> {
    app::save_project_file().or_else(|e| Err(format!("Failed to save current project {:?}", e)))?;
    Ok(json!(true))
}

#[tauri::command]
pub fn set_project_path(path: String) -> Result<Value, String> {
    app::set_project_path(&path).map_err(|e| format!("Failed to set project path: {}", e))?;
    Ok(json!(true))
}

#[tauri::command]
pub fn open_project(path: String) -> Result<Value, String> {
    let file_content = fs::read_to_string(path.clone()).or_else(|_| Err("Failed to open file".to_string()))?;
    let project:Project = serde_json::from_str(&file_content).or_else(|_| Err("Failed to parse JSON"))?;
    app::load_project(project.clone()).map_err(|e| format!("Failed to load project: {}", e))?;
    app::save_current_project(project.clone()).map_err(|e| format!("Failed to save current project {}", e))?;
    app::set_project_path(&path).or_else(|_| Err("Failed to set project path".to_string()))?;
    app::emit(EVT_PROJECT_NEW, json!(null));
    Ok(json!(true))
}

#[tauri::command]
pub fn get_project() -> Result<Value, String> {
    let app = app::get_app().unwrap();
    let store = app.store(SETTINGS_FILE).unwrap();
    let project = store.get("project");
    if let Some(project) = project {
        Ok(project)
    } else {
        Err("Failed to get latest project".to_string())?
    }
}

#[tauri::command]
pub fn get_midi_ports() -> Result<Value, String> {
    let devices = utils::get_valid_midi_ports().or_else(|e| Err(format!("Failed to fetch ports {:?}", e)))?;
    let res = serde_json::to_value(devices).or_else(|_| Err("Failed to serialize midi ports".to_string()))?;
    Ok(res)
}

#[tauri::command]
pub fn add_device(id: String, class: String) -> Result<Value, String> {
    let mut device: Option<Box<dyn Device>> = None;
    match class.as_str() {
        "virtual" => {
            #[cfg(windows)]
            {
                Err("Virtual devices are not supported on windows".to_string())?;
            }
            #[cfg(not(windows))]
            {
                device = Some(Box::new(VirtualC::new(&id)));
            }
        },
        "input" => {
            device = Some(Box::new(Input::new(&id)));
        },
        "output" => {
            device = Some(Box::new(Output::new(&id)));
        },
        "monitor" => {
            device = Some(Box::new(Monitor::new(&id)));
        },
        "map" => {
            device = Some(Box::new(Mapper::new(&id)));
        },
        "split" => {
            device = Some(Box::new(Splitter::new(&id)));
        },
        "delay" => {
            device = Some(Box::new(Delay::new(&id)));
        },
        "trigger" => {
            device = Some(Box::new(Trigger::new(&id)));
        },
        "script" => {
            device = Some(Box::new(Script::new(&id)));
        }
        _ => Err(format!("Unknown device type {}", class))?
    };
    let mut device = device.unwrap();
    device.init()
        .or_else(|e| Err(format!("{:?}", e)))?;
    let res = device.serialize()
        .map_err(|e| format!("failed to serialize device {:?} ", e))?;
    let hub_instance = Hub::get_instance();
    let mut hub = hub_instance
        .lock()
        .map_err(|e| format!("Failed to acquire hub lock: {:?}", e))?;
    if !hub.add_device(device) {
        Err(format!("Device already exists {}", id))?;
    }
    Ok(res)
}

#[tauri::command]
pub fn set_device_data(id: String, key: String, data: Value) -> Result<(), String> {
    let hub_instance = Hub::get_instance();
    let mut hub = hub_instance
        .lock()
        .map_err(|e| format!("Failed to acquire hub lock: {:?}", e))?;

    hub.set_device_data(id, key, data)
}

#[tauri::command]
pub fn hub_process(ts: u64, bytes: Vec<u8>, from: String, to: String, from_port: String, to_port: String) -> Result<(), String> {
    let hub_instance = Hub::get_instance();
    let mut hub = hub_instance
        .lock()
        .map_err(|e| format!("Failed to acquire hub lock: {:?}", e))?;

    hub.process(ts, &bytes, &from, &to, &from_port, &to_port);
    Ok(())
}

#[tauri::command]
pub fn get_device_data(id: String, key: String) -> Result<Option<Value>, String> {
    let hub_instance = Hub::get_instance();
    let mut hub = hub_instance
        .lock()
        .map_err(|e| format!("Failed to acquire hub lock: {:?}", e))?;

    hub.get_device_data(id, key)
}

#[tauri::command]
pub fn get_device(id: String) -> Result<Value, String> {
    let hub_instance = Hub::get_instance();
    let hub = hub_instance
        .lock()
        .map_err(|e| format!("Failed to acquire hub lock: {:?}", e))?;

    if let Some(device) = hub.serialize_device(&id) {
        Ok(device)
    } else {
        Err(format!("Device not found {}", id))
    }
}

#[tauri::command]
pub fn reconnect_device(id: String) -> Result<Value, String> {
    let hub_instance = Hub::get_instance();
    let mut hub = hub_instance.lock().map_err(|e| format!("Failed to acquire hub lock: {:?}", e))?;
    hub.reconnect_device(&id).map_err(|e| format!("Failed to reconnect device {:?}", e))?;
    Ok(json!(true))
}

#[tauri::command]
pub fn remove_device(id: String) -> Result<Value, String> {
    let hub_instance = Hub::get_instance();
    let mut hub = hub_instance
        .lock()
        .map_err(|e| format!("Failed to acquire hub lock: {:?}", e))?;
    if !hub.remove_device(&id) {
        Err(format!("Failed to remove device {}", id))?;
    }
    Ok(json!(true))
}

#[tauri::command]
pub fn exit() -> Result<(), String> {
    let app = app::get_app().unwrap();
    app.exit(0);
    Ok(())
}