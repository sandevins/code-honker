//! Minimal GDI transparent, click-through, always-on-top overlay with a moving circle
//! Windows only

use std::ptr::null_mut;
use std::thread;
use std::time::{Duration, Instant};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM, HINSTANCE};
use windows::core::PCWSTR;
use std::ffi::c_void;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Foundation::COLORREF;

const CLASS_NAME: &str = "GDIOverlayWindow";
const CIRCLE_RADIUS: i32 = 50;

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);
            // Transparent background
            let rect = {
                let mut r = RECT::default();
            let _ = GetClientRect(hwnd, &mut r);
                r
            };
            FillRect(
                hdc,
                &rect,
                HBRUSH(GetStockObject(GET_STOCK_OBJECT_FLAGS(5)).0), // 5 = NULL_BRUSH
            );
            // Draw moving circle
            let now = Instant::now();
            let t = now.elapsed().as_secs_f32();
            let x = (rect.right as f32 / 2.0 + (rect.right as f32 / 3.0) * (t * 0.5).cos()) as i32;
            let y = (rect.bottom as f32 / 2.0 + (rect.bottom as f32 / 3.0) * (t * 0.7).sin()) as i32;
            let yellow = COLORREF(0x00FFFF); // 0x00BBGGRR
            let brush = CreateSolidBrush(yellow);
            let old_brush = SelectObject(hdc, brush);
            let _ = Ellipse(hdc, x - CIRCLE_RADIUS, y - CIRCLE_RADIUS, x + CIRCLE_RADIUS, y + CIRCLE_RADIUS);
            SelectObject(hdc, old_brush);
            let _ = DeleteObject(brush);
            let _ = EndPaint(hwnd, &ps);
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

pub fn run_gdi_overlay() {
    unsafe {
        let hinstance = GetModuleHandleW(None).unwrap();
        let class_name: Vec<u16> = CLASS_NAME.encode_utf16().chain(Some(0)).collect();
        let wnd_class = WNDCLASSW {
            lpfnWndProc: Some(wnd_proc),
            hInstance: HINSTANCE(hinstance.0),
            lpszClassName: PCWSTR(class_name.as_ptr()),
            style: CS_HREDRAW | CS_VREDRAW,
            ..Default::default()
        };
        RegisterClassW(&wnd_class);
        let screen_w = GetSystemMetrics(SM_CXSCREEN);
        let screen_h = GetSystemMetrics(SM_CYSCREEN);
        let hwnd = CreateWindowExW(
            WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST,
            PCWSTR(class_name.as_ptr()),
            PCWSTR(class_name.as_ptr()),
            WS_POPUP,
            0,
            0,
            screen_w,
            screen_h,
            HWND(0),
            None,
            HINSTANCE(hinstance.0),
            None,
        );
        // Set per-pixel alpha (255 = fully opaque, 0 = fully transparent)
        let _ = SetLayeredWindowAttributes(hwnd, COLORREF(0), 255, LWA_ALPHA);
        let _ = ShowWindow(hwnd, SW_SHOW);
        let _ = UpdateWindow(hwnd);
        // Animation loop
        let mut msg = MSG::default();
        loop {
            while PeekMessageW(&mut msg, HWND(0), 0, 0, PM_REMOVE).as_bool() {
                if msg.message == WM_QUIT {
                    return;
                }
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
            let _ = InvalidateRect(hwnd, None, true);
            thread::sleep(Duration::from_millis(16));
        }
    }
}
