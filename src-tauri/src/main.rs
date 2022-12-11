#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod admin;
mod menu_diy;
mod system_tray;
mod utils;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

fn has_arg(args: &[String], arg: &str) -> bool {
    args.contains(&arg.to_string())
}

fn main() {
    let context = tauri::generate_context!();
    let app_name = &context.package_info().name;
    tauri::Builder::default()
        .setup(|app|{
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
            Ok(())
        })
        // .system_tray(system_tray::system_tray())
        .menu(menu_diy::init(&context))
        // .system_tray(menu_diy::tray_menu())
        .on_menu_event(menu_diy::menu_handler)
        .on_system_tray_event(menu_diy::tray_handler)
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "Home" => {
                    println!("nice")
                }
                _ => {}
            }
        })
        .run(context)
        .expect("error while running tauri application");
}
