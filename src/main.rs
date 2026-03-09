#![cfg_attr(windows, windows_subsystem = "windows")]

mod config;
mod ui;

use eframe::egui;

fn main() {
    let md_path = find_keys_file();
    let text = std::fs::read_to_string(&md_path).unwrap_or_else(|e| {
        eprintln!("Could not read {}: {}", md_path.display(), e);
        std::process::exit(1);
    });

    let sections = config::parse(&text);

    #[cfg(windows)]
    close_existing_keyviewer();

    let (x, y, w, h) = primary_monitor_rect();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("KeyViewer")
            .with_decorations(false)
            .with_resizable(false)
            .with_always_on_top()
            .with_inner_size([w, h])
            .with_position([x, y]),
        centered: false,
        ..Default::default()
    };

    eframe::run_native(
        "KeyViewer",
        options,
        Box::new(move |cc| {
            apply_theme(&cc.egui_ctx);
            Ok(Box::new(ui::KeyViewerApp::new(sections)))
        }),
    )
    .expect("eframe failed");
}

fn find_keys_file() -> std::path::PathBuf {
    if let Ok(mut p) = std::env::current_exe() {
        p.pop();
        p.push("keys.md");
        if p.exists() { return p; }
    }
    std::path::PathBuf::from("keys.md")
}

fn primary_monitor_rect() -> (f32, f32, f32, f32) {
    #[cfg(windows)]
    {
        use windows_sys::Win32::{
            Foundation::{POINT, RECT},
            Graphics::Gdi::{
                GetMonitorInfoW, MONITORINFO, MONITOR_DEFAULTTOPRIMARY, MonitorFromPoint,
            },
        };
        unsafe {
            let monitor = MonitorFromPoint(POINT { x: 0, y: 0 }, MONITOR_DEFAULTTOPRIMARY);
            let mut info = MONITORINFO {
                cbSize:    std::mem::size_of::<MONITORINFO>() as u32,
                rcMonitor: RECT { left: 0, top: 0, right: 0, bottom: 0 },
                rcWork:    RECT { left: 0, top: 0, right: 0, bottom: 0 },
                dwFlags:   0,
            };
            GetMonitorInfoW(monitor, &mut info);
            let work  = info.rcWork;
            let mon_w = (work.right  - work.left) as f32;
            let mon_h = (work.bottom - work.top)  as f32;
            let win_w = mon_w * 0.95;
            let win_h = mon_h * 0.95;
            let x = work.left as f32 + (mon_w - win_w) / 2.0;
            let y = work.top  as f32 + (mon_h - win_h) / 2.0;
            return (x, y, win_w, win_h);
        }
    }
    #[allow(unreachable_code)]
    (72.0, 54.0, 1296.0, 972.0)
}

fn apply_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.visuals = egui::Visuals::dark();
    style.visuals.window_corner_radius = egui::CornerRadius::same(8);
    ctx.set_style(style);
}

#[cfg(windows)]
fn close_existing_keyviewer() {
    use windows_sys::Win32::UI::WindowsAndMessaging::{FindWindowW, PostMessageW, WM_CLOSE};
    unsafe {
        let title: Vec<u16> = "KeyViewer\0".encode_utf16().collect();
        let hwnd = FindWindowW(std::ptr::null(), title.as_ptr());
        if !hwnd.is_null() {
            PostMessageW(hwnd, WM_CLOSE, 0, 0);
            std::thread::sleep(std::time::Duration::from_millis(300));
        }
    }
}
