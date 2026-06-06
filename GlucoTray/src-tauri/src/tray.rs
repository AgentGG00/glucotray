use tauri::{
    AppHandle, Manager,
    image::Image,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};
use image::{ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;
use ab_glyph::{FontRef, PxScale};
use crate::db::mgdl_to_mmol;
use crate::state::AppState;

const ICON_SIZE: u32 = 32;
const VERY_HIGH_MGDL: i32 = 250;
const CRITICAL_LOW_MGDL: i32 = 54;
const COLOR_NO_DATA: &str = "#6B7280";

pub struct TrayState {
    pub update_available: bool,
}

pub struct ColorScheme {
    pub critical_low: String,
    pub low:          String,
    pub normal:       String,
    pub high:         String,
    pub very_high:    String,
}

pub fn resolve_color(value_mgdl: i32, trend: &str, threshold_low: i32, threshold_high: i32, colors: &ColorScheme) -> String {
    if value_mgdl <= 0 {
        return COLOR_NO_DATA.to_string();
    }
    if trend == "Low" || value_mgdl < CRITICAL_LOW_MGDL {
        return colors.critical_low.clone();
    }
    if value_mgdl < threshold_low {
        return colors.low.clone();
    }
    if value_mgdl > VERY_HIGH_MGDL {
        return colors.very_high.clone();
    }
    if value_mgdl > threshold_high {
        return colors.high.clone();
    }
    colors.normal.clone()
}

fn trend_symbol(trend: &str) -> &str {
    match trend {
        "DoubleUp"      => "⇈",
        "SingleUp"      => "↑",
        "FortyFiveUp"   => "↗",
        "Flat"          => "→",
        "FortyFiveDown" => "↘",
        "SingleDown"    => "↓",
        "DoubleDown"    => "⇊",
        "Low"           => "",
        _               => "?",
    }
}

fn hex_to_rgba(hex: &str) -> Rgba<u8> {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(128);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(128);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(128);
    Rgba([r, g, b, 255])
}

pub fn render_icon(value_mgdl: i32, trend: &str, bg_hex: &str, unit: &str, update_available: bool) -> Vec<u8> {
    let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_pixel(ICON_SIZE, ICON_SIZE, Rgba([0, 0, 0, 0]));

    let bg_color = hex_to_rgba(bg_hex);
    for pixel in img.pixels_mut() {
        *pixel = bg_color;
    }

    let font_data = include_bytes!("../assets/fonts/NotoSans-Bold.ttf");
    let font = FontRef::try_from_slice(font_data).unwrap();
    let scale = PxScale::from(11.0);
    let text_color = Rgba([255, 255, 255, 255]);

    if value_mgdl <= 0 {
        draw_text_mut(&mut img, text_color, 4, 10, scale, &font, "N/A");
    } else {
        let label = if unit == "mmol" {
            format!("{:.1}", mgdl_to_mmol(value_mgdl))
        } else {
            format!("{}", value_mgdl)
        };
        draw_text_mut(&mut img, text_color, 2, 2, scale, &font, &label);

        let sym = trend_symbol(trend);
        if !sym.is_empty() {
            draw_text_mut(&mut img, text_color, 2, 16, scale, &font, sym);
        }
    }

    if update_available {
        for x in 24..32u32 {
            for y in 0..8u32 {
                img.put_pixel(x, y, Rgba([220, 38, 38, 255]));
            }
        }
    }

    let mut png_bytes: Vec<u8> = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut png_bytes),
        image::ImageFormat::Png,
    ).unwrap();

    png_bytes
}

pub fn build_menu(app: &AppHandle, update_available: bool) -> tauri::Result<Menu<tauri::Wry>> {
    let title = MenuItem::with_id(app, "title", "GlucoTray", false, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let update_label = if update_available { "Update 🔴" } else { "Update check" };
    let update = MenuItem::with_id(app, "update", update_label, true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let restart = MenuItem::with_id(app, "restart", "Restart", true, None::<&str>)?;

    Menu::with_items(app, &[&title, &separator, &update, &separator, &quit, &restart])
}

pub fn setup_tray(app: &AppHandle) -> tauri::Result<()> {
    let png_bytes = render_icon(0, "Flat", COLOR_NO_DATA, "mgdl", false);
    let icon = Image::from_bytes(&png_bytes)?;
    let menu = build_menu(app, false)?;

    let _tray = TrayIconBuilder::with_id("main")
        .icon(icon)
        .menu(&menu)
        .tooltip("GlucoTray")
        .on_menu_event({
            let app = app.clone();
            move |_tray, event| {
                match event.id.as_ref() {
                    "quit"    => { app.exit(0); }
                    "restart" => { app.restart(); }
                    "update"  => {
                        let state = app.state::<std::sync::Mutex<TrayState>>();
                        let update_available = state.lock().unwrap().update_available;
                        if update_available {
                            let _ = open::that("https://github.com/AgentGG00/glucotray/releases/latest");
                        }
                    }
                    _ => {}
                }
            }
        })
        .on_tray_icon_event({
            let app = app.clone();
            move |_tray, event| {
                if let TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } = event
                {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        })
        .build(app)?;

    Ok(())
}

pub fn update_tray(app: &AppHandle, value_mgdl: i32, trend: &str, bg_hex: &str) {
    let update_available = {
        let state = app.state::<std::sync::Mutex<TrayState>>();
        let x = state.lock().unwrap().update_available;
        x
    };

    let unit = {
        let app_state = app.state::<std::sync::Mutex<AppState>>();
        let x = app_state.lock().unwrap().unit.clone();
        x
    };

    let png_bytes = render_icon(value_mgdl, trend, bg_hex, &unit, update_available);

    if let Ok(icon) = Image::from_bytes(&png_bytes) {
        if let Some(tray) = app.tray_by_id("main") {
            let _ = tray.set_icon(Some(icon));
        }
    }

    let tooltip = if value_mgdl <= 0 {
        "GlucoTray – N/A".to_string()
    } else {
        let mmol = mgdl_to_mmol(value_mgdl);
        format!("GlucoTray – {} mg/dL / {:.1} mmol/L {}", value_mgdl, mmol, trend_symbol(trend))
    };

    if let Some(tray) = app.tray_by_id("main") {
        let _ = tray.set_tooltip(Some(&tooltip));
    }
}