//! The tray icon: the only way back to a window that close or minimize hid.

use tauri::menu::{CheckMenuItem, Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Runtime};

use crate::autostart;
use crate::window::show_main_window;

const SHOW_ID: &str = "tray-show";
const AUTOSTART_ID: &str = "tray-autostart";
const QUIT_ID: &str = "tray-quit";

pub fn install<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let show = MenuItem::with_id(app, SHOW_ID, "Open OpenPencil", true, None::<&str>)?;
    let start_with_windows = CheckMenuItem::with_id(
        app,
        AUTOSTART_ID,
        "Start with Windows",
        true,
        autostart::is_enabled(),
        None::<&str>,
    )?;
    let quit = MenuItem::with_id(app, QUIT_ID, "Quit OpenPencil", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show, &start_with_windows, &quit])?;

    let check = start_with_windows.clone();
    let mut builder = TrayIconBuilder::with_id("openpencil-tray")
        .tooltip("OpenPencil")
        .menu(&menu)
        // The menu opens on right click; a left click is the shortcut back to the window.
        .show_menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id().as_ref() {
            SHOW_ID => show_main_window(app),
            AUTOSTART_ID => {
                let enabled = !autostart::is_enabled();
                match autostart::set_enabled(enabled) {
                    Ok(()) => {
                        let _ = check.set_checked(enabled);
                    }
                    Err(error) => eprintln!("[tray] could not change startup entry: {error}"),
                }
            }
            // Goes through the frontend so unsaved documents still get their prompt; bringing
            // the window back first keeps that prompt visible if one appears.
            QUIT_ID => {
                show_main_window(app);
                let _ = app.emit("menu-event", "quit");
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_main_window(tray.app_handle());
            }
        });

    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }

    builder.build(app)?;
    Ok(())
}
