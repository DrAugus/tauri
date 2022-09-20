use tauri::{Menu, MenuItem, Submenu, CustomMenuItem};

// configure the menu
// refactor os_default menu
pub(crate) fn menu_info(#[allow(unused)] app_name: &str) -> Menu {
    let mut menu = Menu::new();
    #[cfg(target_os = "macos")]
    {
        menu = menu.add_submenu(Submenu::new(
            app_name,
            Menu::new()
                .add_native_item(MenuItem::About(
                    app_name.to_string(),
                    AboutMetadata::default(),
                ))
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Services)
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Hide)
                .add_native_item(MenuItem::HideOthers)
                .add_native_item(MenuItem::ShowAll)
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Quit),
        ));
    }

    #[cfg(not(target_os = "linux"))]
        let mut edit_menu = Menu::new();
    #[cfg(target_os = "macos")]
    {
        edit_menu = edit_menu.add_native_item(MenuItem::Undo);
        edit_menu = edit_menu.add_native_item(MenuItem::Redo);
        edit_menu = edit_menu.add_native_item(MenuItem::Separator);
    }
    #[cfg(not(target_os = "linux"))]
    {
        edit_menu = edit_menu.add_native_item(MenuItem::Cut);
        edit_menu = edit_menu.add_native_item(MenuItem::Copy);
        edit_menu = edit_menu.add_native_item(MenuItem::Paste);
    }
    #[cfg(target_os = "macos")]
    {
        edit_menu = edit_menu.add_native_item(MenuItem::SelectAll);
    }
    #[cfg(not(target_os = "linux"))]
    {
        menu = menu.add_submenu(Submenu::new("Edit", edit_menu));
    }
    #[cfg(target_os = "macos")]
    {
        menu = menu.add_submenu(Submenu::new(
            "View",
            Menu::new().add_native_item(MenuItem::EnterFullScreen),
        ));
    }

    let mut window_menu = Menu::new();
    window_menu = window_menu.add_native_item(MenuItem::Minimize);
    #[cfg(target_os = "macos")]
    {
        window_menu = window_menu.add_native_item(MenuItem::Zoom);
        window_menu = window_menu.add_native_item(MenuItem::Separator);
    }
    window_menu = window_menu.add_native_item(MenuItem::Separator);
    window_menu = window_menu.add_native_item(MenuItem::CloseWindow);
    #[cfg(not(target_os = "macos"))]
    {
        window_menu = window_menu.add_native_item(MenuItem::Quit);
    }
    menu = menu.add_submenu(Submenu::new("Window", window_menu));

    let mut custom_menu = Menu::new();
    custom_menu = custom_menu.add_item(
        CustomMenuItem::new("home".to_string(), "Home")
    );

    menu = menu.add_submenu(Submenu::new("Op", custom_menu));

    // menu = menu.add_submenu(Submenu::new("About", Menu::new().add_native_item(MenuItem::About())));

    return menu;
}
