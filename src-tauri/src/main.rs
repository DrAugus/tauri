// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// mod admin;
// mod menu_diy;
mod system_tray;
mod utils;

// the payload type must implement `Serialize` and `Clone`.
// #[derive(Clone, serde::Serialize)]
// struct Payload {
//     message: String,
// }

// fn has_arg(args: &[String], arg: &str) -> bool {
//     args.contains(&arg.to_string())
// }

fn main() {
    augus_tauri_lib::run();
    let context = tauri::generate_context!();
    // let app_name = &context.package_info().name;
    tauri::Builder::default()
        .setup(|_app| {
            // let docs_window = tauri::WindowBuilder::new(
            //     app,
            //     "external", /* the unique window label */
            //     tauri::WindowUrl::External("https://draugus.github.io/game/genshin/timeline".parse().unwrap())
            // ).build()?;

            // let local_window = tauri::WindowBuilder::new(
            //     app,
            //     "local",
            //     tauri::WindowUrl::App("index.html".into())
            // ).build()?;

            // let tray = system_tray::system_tray(app).unwrap() ;

            // let tray_handler = menu_diy::tray_handler;

            Ok(())
        })
        // .menu(menu_diy::init(&context))
        // .system_tray(menu_diy::tray_menu())
        // .on_menu_event(menu_diy::menu_handler)
        .run(context)
        .expect("error while running tauri application");
}
