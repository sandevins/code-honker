use windows::{
    Win32::{
        Foundation::{HWND, LPARAM, RECT},
        UI::WindowsAndMessaging::{
            EnumWindows, GetWindowTextA, GetWindowRect,
            IsWindowVisible,
        },
    },
};

pub fn find_vscode_window() -> Option<HWND> {
    let mut result = None;

    unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> windows::Win32::Foundation::BOOL {
        let mut buffer = [0u8; 256];
        if IsWindowVisible(hwnd).as_bool() {
            let len = GetWindowTextA(hwnd, &mut buffer);
            if len > 0 {
                let title = String::from_utf8_lossy(&buffer[..len as usize]);
                if title.contains("Visual Studio Code") || title.contains("Code") {
                    let result = lparam.0 as *mut Option<HWND>;
                    unsafe { *result = Some(hwnd) };
                    return windows::Win32::Foundation::BOOL(0); // Stop enum
                }
            }
        }
        windows::Win32::Foundation::BOOL(1) // Continue enum
    }

    unsafe {
        let _ = EnumWindows(Some(enum_proc), LPARAM(&mut result as *mut _ as isize));
    }

    result
}

pub fn get_window_position(hwnd: HWND) -> (i32, i32) {
    let mut rect = RECT::default();
    unsafe {
        let _ = GetWindowRect(hwnd, &mut rect);
    }

    (rect.left, rect.top)
}
