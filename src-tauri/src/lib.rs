use std::{error::Error, sync::Mutex};

use app::{Project, APP_HANDLE};
use globals::{EVT_FILE_OPEN, EVT_FILE_SAVE, EVT_FILE_SAVE_AS, EVT_SHOW_ABOUT, EVT_WINDOW_SHOW};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tauri::{tray::{TrayIconBuilder, TrayIconEvent}, Builder, Manager};
#[cfg(target_os = "linux")]
use tauri::menu::{MenuBuilder, MenuItemBuilder};
#[cfg(target_os = "macos")]
use tauri::menu::{ CheckMenuItem, Menu, SubmenuBuilder };
use tauri_plugin_store::StoreExt;

pub mod globals;
pub mod hub;
pub mod app;
pub mod utils;
pub mod commands;
pub mod devices {
    pub mod input;
    pub mod output;
    pub mod device;
    #[cfg(not(windows))]
    pub mod virtual_c;
    pub mod monitor;
    pub mod splitter;
    pub mod mapper;
    pub mod delay;
    pub mod script;
    pub mod trigger;
}

/**
 * App settings config file
 */
pub static SETTINGS_FILE: &str = "mdash.config.json";

/**
 * App internal state
 */
#[derive(Default, Clone, Serialize)]
pub struct State {}

/**
 * App persistent settings
 */
#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub project_path: String,
    pub minimize_to_tray: bool,
    pub sidebar_width: u64,
    pub script_templates: Vec<Value>,
    pub script_show_line_numbers: bool,
    pub monitor_in: Value,
    pub monitor_out: Value,
    pub hub_paused: bool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // Global app pointer
            APP_HANDLE.set(app.handle().to_owned()).unwrap();

            // Setup state
            app.manage(Mutex::new(State {}));

            // Setup persistent store
            let store = app.store(SETTINGS_FILE)?;
            if let None = store.get("settings") {
                app::set_settings(Settings::default())?;
            }

            let settings = app::get_settings();
            if settings.hub_paused {
                app::set_hub_paused(true)?;
            }

            // load previous session even if there is a project path
            // session will be allowed to save to project file path
            let last_project: Option<Project> = store
                .get("project")
                .and_then(|p| serde_json::from_value(p).ok());

            if let Some(project) = last_project {
                // load last session project
                app::load_project(project.clone())?;
            } else {
                commands::new_devices_project()?;
            }

            // macOS window menu
            #[cfg(target_os = "macos")]
            {
                let menu = build_menu(app)?;
                app.set_menu(menu)?;
            }

            app.on_menu_event(|app, event| {
                match event.id().as_ref() {
                    "new" => {
                        let _ = commands::new_devices_project();
                    },
                    "new_blank" => {
                        let _ = commands::new_blank_project();
                    },
                    "save" => {
                        let settings = app::get_settings();
                        if settings.project_path.is_empty() {
                            app::emit(EVT_FILE_SAVE_AS, json!(null));
                        } else {
                            app::emit(EVT_FILE_SAVE, json!(null));
                        }
                    },
                    "save_as" => {
                        app::emit(EVT_FILE_SAVE_AS, json!(null));
                    },
                    "open" => {
                        app::emit(EVT_FILE_OPEN, json!(null));
                    },
                    "quit" => app.exit(0),
                    "show" => app::emit(EVT_WINDOW_SHOW, json!(null)),
                    "minimize_to_tray" => {
                        let mut settings = app::get_settings();
                        settings.minimize_to_tray = !settings.minimize_to_tray;
                        let _ = app::set_settings(settings);
                    }
                    "about" => app::emit(EVT_SHOW_ABOUT, json!(null)),
                    _ => {}
                }
            });

            // Tray
            build_tray(app)?;
            app.on_tray_icon_event(|_, event| {
                match event {
                    TrayIconEvent::Click { .. } => app::emit(EVT_WINDOW_SHOW, json!(null)),
                    _ => ()
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::new_devices_project,
            commands::new_blank_project,
            commands::get_settings,
            commands::set_settings,
            commands::get_project,
            commands::get_midi_ports,
            commands::set_hub_paused,
            commands::connect,
            commands::disconnect,
            commands::add_device,
            commands::set_device_data,
            commands::get_device_data,
            commands::get_device,
            commands::reconnect_device,
            commands::hub_process,
            commands::remove_device,
            commands::save_current_project,
            commands::set_project_path,
            commands::save_project_file,
            commands::open_project,
            commands::exit
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/* #[cfg(not(target_os = "macos"))]
fn build_menu(app: &mut tauri::App) -> Result<Menu<tauri::Wry>, Box<dyn Error>> {
    let file_menu = SubmenuBuilder::new(app.handle(), "&File")
        .item(&MenuItemBuilder::with_id("new", "New").build(app)?)
        .item(&MenuItemBuilder::with_id("new_blank", "New Blank").build(app)?)
        .separator()
        .item(&MenuItemBuilder::with_id("open", "Open").build(app)?)
        .item(&MenuItemBuilder::with_id("save", "Save").build(app)?)
        .item(&MenuItemBuilder::with_id("save_as", "Save As").build(app)?)
        .separator()
        .item(&MenuItemBuilder::with_id("quit", "Quit").build(app)?)
        .build()?;

    let settings = app::get_settings();
    let window_menu = SubmenuBuilder::new(app.handle(), "Window")
        .item(&CheckMenuItem::with_id(app, "minimize_to_tray", "Close to tray", true, settings.minimize_to_tray, None::<String>)?)
        .build()?;

    let menu = MenuBuilder::new(app.handle())
        .item(&file_menu)
        .item(&window_menu)
        .item(&MenuItemBuilder::with_id("about", "About").build(app)?)
        .build()?;

    Ok(menu)
} */

#[cfg(target_os = "macos")]
fn build_menu(app: &mut tauri::App) -> Result<Menu<tauri::Wry>, Box<dyn Error>> {
    let file_menu = SubmenuBuilder::new(app.handle(), "&File")
        .item(&MenuItemBuilder::with_id("new", "New").build(app)?)
        .item(&MenuItemBuilder::with_id("new_blank", "New Blank").build(app)?)
        .separator()
        .item(&MenuItemBuilder::with_id("open", "Open").build(app)?)
        .item(&MenuItemBuilder::with_id("save", "Save").build(app)?)
        .item(&MenuItemBuilder::with_id("save_as", "Save As").build(app)?)
        .separator()
        .item(&MenuItemBuilder::with_id("about", "About").build(app)?)
        .item(&MenuItemBuilder::with_id("quit", "Quit").build(app)?)
        .build()?;

    let menu = MenuBuilder::new(app.handle())
        .item(&file_menu)
        .build()?;

    Ok(menu)
}


fn build_tray(app: &mut tauri::App) -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "linux")]
    let menu = MenuBuilder::new(app.handle()) // Fix linux not showing tray without a menu
        .items(&[
            &MenuItemBuilder::with_id("show", "Show").build(app)?,
            &MenuItemBuilder::with_id("quit", "Quit").build(app)?,
        ])
        .build()?;

    #[cfg(not(windows))]
    let icon = tauri::image::Image::from_bytes(include_bytes!("../icons/128x128.png"))
        .expect("Failed to load icon");
    #[cfg(windows)]
    let icon = tauri::image::Image::from_bytes(include_bytes!("../icons/icon.ico"))
        .expect("Failed to load icon");

    #[cfg(target_os = "linux")]
    TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .menu_on_left_click(false)
        .build(app)?;

    #[cfg(not(target_os = "linux"))]
    TrayIconBuilder::new()
        .icon(icon)
        .menu_on_left_click(false)
        .build(app)?;

    Ok(())
}