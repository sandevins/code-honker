mod vscode;
mod goose;

use vscode::find_vscode_window;
use goose::GooseWindow;

fn main() {
    println!("🦆 Code Honker is launching...");

    if let Some(hwnd) = find_vscode_window() {
        println!("✅ VS Code window found: {:?}", hwnd);

        let (x, y) = vscode::get_window_position(hwnd);
        println!("📍 VS Code window position: ({}, {})", x, y);

        // Launch the goose window (asynchronously)
        pollster::block_on(GooseWindow::run_at(x, y));
    } else {
        eprintln!("❌ VS Code window not found!");
    }
}