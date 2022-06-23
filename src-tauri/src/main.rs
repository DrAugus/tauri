#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

// fn update() {
//     // Initialize updater and check if a new version is available
//     // Event: tauri://update
//     window.emit("tauri://update".to_string(), None);
//     // Listen New Update Available
//     // Event: tauri://update-available
//     window.listen("tauri://update-available".to_string(), move |msg| {
//         println!("New version available: {:?}", msg);
//     });
//     // Emit Install and Download
//     // Event: tauri://update-install
//     window.emit("tauri://update-install".to_string(), None);
//     // Listen Install Progress
//     // Event: tauri://update-status
//     window.listen("tauri://update-status".to_string(), move |msg| {
//         println!("New status: {:?}", msg);
//     })
// }

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("error while running tauri application");
}
