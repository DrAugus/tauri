#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use tauri::{Manager, Window};

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// init a background process on the command, and emit periodic events only to the window that used the command
#[tauri::command]
fn init_process(window: Window) {
    std::thread::spawn(move || {
        loop {
            window.emit("tauri://update", Payload { message: "Initialize updater and check if a new version is available".into() }).unwrap();
            window.emit("tauri://update-install", Payload { message: "Emit Install and Download".into() }).unwrap();
        }
    });
}

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .setup(|app| {

            // Global events [https://tauri.app/v1/guides/features/events#global-events-1]
            // listen to the `event-name` (emitted on any window)
            // Listen New Update Available
            let id_update_avail = app.listen_global("tauri://update-available", move |msg| {
                println!("New version available: {:?}", msg);
            });
            // Listen Install Progress
            let id_install_progress = app.listen_global("tauri://update-status", move |msg| {
                println!("New status: {:?}", msg);
            });
            // unlisten to the event using the `id` returned on the `listen_global` function
            // an `once_global` API is also exposed on the `App` struct
            app.unlisten(id_update_avail);
            app.unlisten(id_install_progress);

            // emit the `event-name` event to all webview windows on the frontend
            app.emit_all("tauri://update-available", Payload { message: "new version".into() }).unwrap();
            app.emit_all("tauri://update-status", Payload { message: "update status".into() }).unwrap();


            // // Window-specific events [https://tauri.app/v1/guides/features/events#window-specific-events-1]
            // // `main` here is the window label; it is defined on the window creation or under `tauri.conf.json`
            // // the default value is `main`. note that it must be unique
            // let main_window = app.get_window("main").unwrap();
            //
            // // listen to the `event-name` (emitted on the `main` window)
            // let id_m = main_window.listen("event-name", |event| {
            //     println!("got window event-name with payload {:?}", event.payload());
            // });
            // // unlisten to the event using the `id` returned on the `listen` function
            // // an `once` API is also exposed on the `Window` struct
            // main_window.unlisten(id_m);
            //
            // // emit the `event-name` event to the `main` window
            // main_window.emit("event-name", Payload { message: "Tauri is awesome!".into() }).unwrap();

            Ok(())
        })
        .run(context)
        .expect("error while running tauri application");
}
