use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder},
    App,
};

pub(crate) fn system_tray(app: &mut App) -> Result<TrayIcon, Box<dyn std::error::Error>> {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let hide_i = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
    // let separator = MenuItem::Separator;
    // let separator = MenuItem::separator(app)?;
    let menu = Menu::with_items(app, &[&quit_i, &hide_i])?;

    let tray = TrayIconBuilder::new()
        .menu(&menu)
        .menu_on_left_click(true)
        .on_menu_event(move |app, event| match event.id().as_ref() {
            "quit" => {
                println!("quit clicked");
            }
            _ => (),
        })
        .build(app)?;

    Ok(tray)
}
