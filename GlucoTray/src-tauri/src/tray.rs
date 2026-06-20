use tauri::{
    AppHandle, Manager,
    image::Image,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
};
use image::{ImageBuffer, Rgba};
use imageproc::drawing::{draw_text_mut, text_size};
use ab_glyph::{FontRef, PxScale};
use crate::db::mgdl_to_mmol;
use crate::state::AppState;

const ICON_SIZE: u32 = 16;
const VERY_HIGH_MGDL: i32 = 250;
const CRITICAL_LOW_MGDL: i32 = 54;
const COLOR_NO_DATA: &str = "#6B7280";

pub struct TrayState {
    pub update_available: bool,
    pub last_value_mgdl: i32,
    pub last_trend: String,
    pub last_color: String,
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

fn na_text_color(app: &AppHandle) -> Rgba<u8> {
    let is_dark = app
        .get_webview_window("main")
        .and_then(|w| w.theme().ok())
        .map(|theme| matches!(theme, tauri::Theme::Dark))
        .unwrap_or(true);

    if is_dark {
        Rgba([255, 255, 255, 255])
    } else {
        Rgba([0, 0, 0, 255])
    }
}

pub fn render_icon(
    value_mgdl: i32,
    trend: &str,
    color_hex: &str,
    na_color: Rgba<u8>,
    unit: &str,
    update_available: bool,
) -> Vec<u8> {
    let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_pixel(ICON_SIZE, ICON_SIZE, Rgba([0, 0, 0, 0]));

    let font_data = include_bytes!("../assets/fonts/NotoSans-Bold.ttf");
    let font = FontRef::try_from_slice(font_data).unwrap();

    let text_scale = PxScale::from(20.0);
    let na_scale = PxScale::from(16.0);

    if value_mgdl <= 0 {
        let (na_w, na_h) = text_size(na_scale, &font, "N/A");
        let na_x = ((ICON_SIZE as i32 - na_w as i32) / 2).max(0);
        let na_y = ((ICON_SIZE as i32 - na_h as i32) / 2).max(0);
        draw_text_mut(&mut img, na_color, na_x, na_y, na_scale, &font, "N/A");
    } else {
        let text_color = hex_to_rgba(color_hex);

        let label = if unit == "mmol" {
            format!("{:.1}", mgdl_to_mmol(value_mgdl))
        } else {
            format!("{}", value_mgdl)
        };

        let sym = trend_symbol(trend);
        let combined = if sym.is_empty() {
            label.clone()
        } else {
            format!("{} {}", label, sym)
        };

        let (text_w, text_h) = text_size(text_scale, &font, &combined);
        let text_x = ((ICON_SIZE as i32 - text_w as i32) / 2).max(0);
        let text_y = ((ICON_SIZE as i32 - text_h as i32) / 2).max(0);

        draw_text_mut(&mut img, text_color, text_x, text_y, text_scale, &font, &combined);
    }

    if update_available {
        let badge_start = (ICON_SIZE as f32 * 0.75).round() as u32;
        let badge_end = (ICON_SIZE as f32 * 0.25).round() as u32;
        for x in badge_start..ICON_SIZE {
            for y in 0..badge_end {
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
    let title = MenuItem::with_id(app, "open_window", "GlucoTray", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let update_label = if update_available { "Update 🔴" } else { "Update check" };
    let update = MenuItem::with_id(app, "update", update_label, true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let restart = MenuItem::with_id(app, "restart", "Restart", true, None::<&str>)?;

    Menu::with_items(app, &[&title, &separator, &update, &separator, &quit, &restart])
}

pub fn setup_tray(app: &AppHandle) -> tauri::Result<()> {
    let na_color = na_text_color(app);
    let png_bytes = render_icon(0, "Flat", COLOR_NO_DATA, na_color, "mgdl", false);
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
                    "open_window" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
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
        .build(app)?;

    Ok(())
}

pub fn update_tray(app: &AppHandle, value_mgdl: i32, trend: &str, color_hex: &str) {
    let update_available = {
        let state = app.state::<std::sync::Mutex<TrayState>>();
        let mut s = state.lock().unwrap();
        s.last_value_mgdl = value_mgdl;
        s.last_trend = trend.to_string();
        s.last_color = color_hex.to_string();
        s.update_available
    };

    let unit = {
        let app_state = app.state::<std::sync::Mutex<AppState>>();
        let s = app_state.lock().unwrap();
        s.unit.clone()
    };

    let na_color = na_text_color(app);
    let png_bytes = render_icon(value_mgdl, trend, color_hex, na_color, &unit, update_available);

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