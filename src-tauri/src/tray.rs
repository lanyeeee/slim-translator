use rust_i18n::t;
use tauri::{
    AppHandle,
    Manager,
    menu::{CheckMenuItemBuilder, Menu, MenuEvent, MenuItem, Submenu}, tray::TrayIconBuilder, Wry,
};

use crate::config::Config;

pub fn init(app_handle: &AppHandle) -> anyhow::Result<()> {
    TrayIconBuilder::with_id("menu")
        .tooltip("slim-translator")
        .icon(app_handle.default_window_icon().unwrap().clone())
        .menu(&create_menu(app_handle)?)
        .on_menu_event(handler)
        .build(app_handle)?;
    Ok(())
}

fn create_menu(handle: &AppHandle) -> anyhow::Result<Menu<Wry>> {
    let from_submenu = create_from_submenu(handle)?;
    let to_submenu = create_to_submenu(handle)?;
    let about_submenu = create_about_submenu(handle)?;
    let exit = MenuItem::with_id(handle, "exit", t!("tray.exit"), true, None::<&str>)?;

    Menu::with_items(handle, &[&from_submenu, &to_submenu, &about_submenu, &exit])
        .map_err(|e| anyhow::anyhow!("Failed to create menu, error: {}", e))
}

fn create_from_submenu(app: &AppHandle) -> anyhow::Result<Submenu<Wry>> {
    let auto_i18n = t!("tray.auto_lan");
    let en_i18n = t!("tray.en_lan");
    let zh_i18n = t!("tray.zh_lan");
    let from_i18n = t!("tray.from");

    let config = Config::load(app)?;

    let auto = CheckMenuItemBuilder::with_id("from.auto", auto_i18n).build(app)?;
    auto.set_checked(config.from == "auto")?;
    let en = CheckMenuItemBuilder::with_id("from.english", en_i18n).build(app)?;
    en.set_checked(config.from == "en")?;
    let zh = CheckMenuItemBuilder::with_id("from.chinese", zh_i18n).build(app)?;
    zh.set_checked(config.from == "zh")?;

    let from_submenu = Submenu::with_items(app, from_i18n, true, &[&auto, &en, &zh])?;

    Ok(from_submenu)
}

fn create_to_submenu(app: &AppHandle) -> anyhow::Result<Submenu<Wry>> {
    let en_i18n = t!("tray.en_lan");
    let zh_i18n = t!("tray.zh_lan");
    let to_i18n = t!("tray.to");

    let config = Config::load(app)?;

    let en = CheckMenuItemBuilder::with_id("to.english", en_i18n).build(app)?;
    en.set_checked(config.to == "en")?;
    let zh = CheckMenuItemBuilder::with_id("to.chinese", zh_i18n).build(app)?;
    zh.set_checked(config.to == "zh")?;

    let to_submenu = Submenu::with_items(app, to_i18n, true, &[&en, &zh])?;

    Ok(to_submenu)
}

fn create_about_submenu(handle: &AppHandle) -> anyhow::Result<Submenu<Wry>> {
    let github = MenuItem::with_id(handle, "about.github", "GitHub", true, None::<&str>)?;
    let version = MenuItem::with_id(handle, "about.version", "0.0.0", false, None::<&str>)?;

    let about = Submenu::with_items(handle, t!("tray.about"), true, &[&github, &version])?;

    Ok(about)
}

fn handler(app: &AppHandle, event: MenuEvent) {
    let mut config = Config::load(app).unwrap();
    match event.id.as_ref() {
        "from.auto" => {
            config.from = "auto".to_owned();
        }
        "from.english" => {
            config.from = "en".to_owned();
        }
        "from.chinese" => {
            config.from = "zh".to_owned();
        }
        "to.english" => {
            config.to = "en".to_owned();
        }
        "to.chinese" => {
            config.to = "zh".to_owned();
        }
        "about.github" => {
            let _ = open::that("https://github.com/lanyeeee/slim-translator");
        }
        "exit" => {
            let panel = app.get_webview_window("panel").unwrap();
            let _ = panel.hide();
            app.exit(0);
        }
        _ => {}
    };
    config.save(app).unwrap();
    fresh(app);
}

fn fresh(app_handler: &AppHandle) {
    let _ = app_handler
        .tray_by_id("menu")
        .unwrap()
        .set_menu(Some(create_menu(app_handler).unwrap()));
}
