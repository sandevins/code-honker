use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{HWND, LPARAM},
        UI::WindowsAndMessaging::{
            EnumWindows, GetWindowTextA, GetWindowThreadProcessId, IsWindowVisible,
        },
    },
};
use std::{ffi::CString, ptr, process::Command};
use enigo::{Enigo, MouseControllable};

fn main() {
    println!("🦆 Goose Gremlin Starting...");

    let code_hwnd = find_vscode_window();
    if let Some(hwnd) = code_hwnd {
        println!("✅ VS Code window found: {:?}", hwnd);

        let (x, y) = get_window_position(hwnd);
        println!("🧭 Window position: ({}, {})", x, y);

        // Example goose movement (moves the mouse near VS Code)
        let mut enigo = Enigo::new();
        enigo.mouse_move_to(x + 100, y + 100);

        // Spawn goose window or animation here (stubbed)
        println!("🦆 Goose is looking at your code...");
    } else {
        println!("❌ Could not find VS Code window!");
    }
}

/// Find the main VS Code window using window title and visibility.
fn find_vscode_window() -> Option<HWND> {
    let mut result = None;

    use windows::Win32::Foundation::BOOL;
    unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let mut buffer = [0u8; 256];
        if IsWindowVisible(hwnd).as_bool() {
            let len = GetWindowTextA(hwnd, &mut buffer);
            if len > 0 {
                let title = String::from_utf8_lossy(&buffer[..len as usize]);
                if title.contains("Visual Studio Code") || title.contains("Code") {
                    println!("🔍 Matched window: {}", title);
                    let result = lparam.0 as *mut Option<HWND>;
                    unsafe { *result = Some(hwnd) };
                    return BOOL(0); // Stop enumeration
                }
            }
        }
        BOOL(1) // Continue enumeration
    }

    unsafe {
        EnumWindows(Some(enum_proc), LPARAM(&mut result as *mut _ as isize))
            .expect("Failed to enumerate windows");
    }

    result
}

/// Get the top-left position of a window.
fn get_window_position(hwnd: HWND) -> (i32, i32) {
    use windows::Win32::Foundation::RECT;
    use windows::Win32::UI::WindowsAndMessaging::GetWindowRect;

    let mut rect = RECT::default();
    unsafe {
        GetWindowRect(hwnd, &mut rect);
    }

    (rect.left, rect.top)
}
