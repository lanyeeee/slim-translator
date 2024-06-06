use std::sync::Arc;

use rust_i18n::t;
use tauri::{
    async_runtime::Mutex,
    menu::{CheckMenuItemBuilder, Menu, MenuEvent, MenuItem, Submenu},
    tray::TrayIconBuilder,
    AppHandle, Manager, Wry,
};

pub fn init(app_handle: &AppHandle) -> anyhow::Result<()> {
    let sys_locale = sys_locale::get_locale().unwrap_or_else(|| String::from("en-US"));
    rust_i18n::set_locale(&sys_locale);

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

fn create_from_submenu(app_handle: &AppHandle) -> anyhow::Result<Submenu<Wry>> {
    let auto_i18n = t!("tray.auto_lan");
    let en_i18n = t!("tray.en_lan");
    let zh_i18n = t!("tray.zh_lan");
    let jp_i18n = t!("tray.jp_lan");
    let kor_i18n = t!("tray.kor_lan");
    let from_i18n = t!("tray.from");
    let config = app_handle.state::<Arc<Mutex<crate::config::Config>>>();

    let config = config.blocking_lock();

    let auto = CheckMenuItemBuilder::with_id("from.auto", auto_i18n).build(app_handle)?;
    auto.set_checked(config.from == "auto")?;
    let en = CheckMenuItemBuilder::with_id("from.english", en_i18n).build(app_handle)?;
    en.set_checked(config.from == "en")?;
    let zh = CheckMenuItemBuilder::with_id("from.chinese", zh_i18n).build(app_handle)?;
    zh.set_checked(config.from == "zh")?;
    let jp = CheckMenuItemBuilder::with_id("from.japanese", jp_i18n).build(app_handle)?;
    jp.set_checked(config.from == "jp")?;
    let kor = CheckMenuItemBuilder::with_id("from.korean", kor_i18n).build(app_handle)?;
    kor.set_checked(config.from == "kor")?;

    let from_submenu =
        Submenu::with_items(app_handle, from_i18n, true, &[&auto, &en, &zh, &jp, &kor])?;

    Ok(from_submenu)
}

fn create_to_submenu(app_handle: &AppHandle) -> anyhow::Result<Submenu<Wry>> {
    let en_i18n = t!("tray.en_lan");
    let zh_i18n = t!("tray.zh_lan");
    let jp_i18n = t!("tray.jp_lan");
    let kor_i18n = t!("tray.kor_lan");
    let to_i18n = t!("tray.to");

    let config = app_handle.state::<Arc<Mutex<crate::config::Config>>>();
    let config = config.blocking_lock();

    let en = CheckMenuItemBuilder::with_id("to.english", en_i18n).build(app_handle)?;
    en.set_checked(config.to == "en")?;
    let zh = CheckMenuItemBuilder::with_id("to.chinese", zh_i18n).build(app_handle)?;
    zh.set_checked(config.to == "zh")?;
    let jp = CheckMenuItemBuilder::with_id("to.japanese", jp_i18n).build(app_handle)?;
    jp.set_checked(config.to == "jp")?;
    let kor = CheckMenuItemBuilder::with_id("to.korean", kor_i18n).build(app_handle)?;
    kor.set_checked(config.to == "kor")?;

    let to_submenu = Submenu::with_items(app_handle, to_i18n, true, &[&en, &zh, &jp, &kor])?;

    Ok(to_submenu)
}

fn create_about_submenu(handle: &AppHandle) -> anyhow::Result<Submenu<Wry>> {
    let github = MenuItem::with_id(handle, "about.github", "GitHub", true, None::<&str>)?;
    let version = MenuItem::with_id(handle, "about.version", "0.0.0", false, None::<&str>)?;

    let about = Submenu::with_items(handle, t!("tray.about"), true, &[&github, &version])?;

    Ok(about)
}

fn handler(app_handle: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        "from.auto" => {
            crate::config::Config::set_from(app_handle, "auto");
        }
        "from.english" => {
            crate::config::Config::set_from(app_handle, "en");
        }
        "from.chinese" => {
            crate::config::Config::set_from(app_handle, "zh");
        }
        "from.japanese" => {
            crate::config::Config::set_from(app_handle, "jp");
        }
        "from.korean" => {
            crate::config::Config::set_from(app_handle, "kor");
        }
        "to.english" => {
            crate::config::Config::set_to(app_handle, "en");
        }
        "to.chinese" => {
            crate::config::Config::set_to(app_handle, "zh");
        }
        "to.japanese" => {
            crate::config::Config::set_to(app_handle, "jp");
        }
        "to.korean" => {
            crate::config::Config::set_to(app_handle, "kor");
        }
        "about.github" => {
            let _ = open::that("https://github.com/lanyeeee/slim-translator");
        }
        "exit" => {
            let panel = app_handle.get_webview_window("panel").unwrap();
            let _ = panel.hide();
            app_handle.exit(0)
        }
        _ => {}
    }
    fresh(app_handle);
}

fn fresh(app_handler: &AppHandle) {
    let _ = app_handler
        .tray_by_id("menu")
        .unwrap()
        .set_menu(Some(create_menu(app_handler).unwrap()));
}
