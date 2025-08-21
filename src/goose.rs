use pixels::{Pixels, SurfaceTexture};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use rand::Rng;
use std::time::{Duration, Instant};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

pub struct GooseWindow;

impl GooseWindow {
    pub async fn run_at(offset_x: i32, offset_y: i32) {
        let event_loop = EventLoop::new().unwrap();
        let width = 200;
        let height = 200;

        let window = WindowBuilder::new()
            .with_title("Code Honker Goose")
            .with_inner_size(LogicalSize::new(width, height))
            .with_resizable(false)
            .with_position(winit::dpi::PhysicalPosition::new(offset_x + 100, offset_y + 100))
            .with_transparent(true)
            .build(&event_loop)
            .unwrap();

        // Make the window click-through (WS_EX_LAYERED | WS_EX_TRANSPARENT)
        #[cfg(target_os = "windows")]
        {
            use windows::Win32::Foundation::HWND;
            use windows::Win32::UI::WindowsAndMessaging::{GetWindowLongW, SetWindowLongW, GWL_EXSTYLE, WS_EX_LAYERED, WS_EX_TRANSPARENT};

            if let Ok(handle) = window.window_handle() {
                match handle.as_raw() {
                    RawWindowHandle::Win32(h) => {
                        let hwnd = HWND(h.hwnd.get() as isize);
                        unsafe {
                            let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
                            SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style | WS_EX_LAYERED.0 as i32 | WS_EX_TRANSPARENT.0 as i32);
                        }
                    }
                    _ => {}
                }
            }
        }

        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
        let mut pixels = Pixels::new(size.width, size.height, surface_texture).unwrap();

        let mut x = 100.0;
        let mut y = 100.0;
        let mut dx = 1.0;
        let mut dy = 1.0;
        let mut rng = rand::thread_rng();
        let mut last_move = Instant::now();

        event_loop.run(move |event, event_loop_window_target| {
            match &event {
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    event_loop_window_target.exit();
                }
                Event::AboutToWait => {
                    if last_move.elapsed() > Duration::from_millis(16) {
                        x += dx;
                        y += dy;

                        if rng.gen_bool(0.05) {
                            dx = (rng.gen_range(-1.0..1.0) * 2.0f32).clamp(-2.0, 2.0);
                            dy = (rng.gen_range(-1.0..1.0) * 2.0f32).clamp(-2.0, 2.0);
                        }

                        if x < 10.0 || x > size.width as f32 - 10.0 {
                            dx = -dx;
                        }
                        if y < 10.0 || y > size.height as f32 - 10.0 {
                            dy = -dy;
                        }

                        window.request_redraw();
                        last_move = Instant::now();
                    }
                }
                Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                    let frame = pixels.frame_mut();
                    // Fill with transparent pixels
                    for px in frame.chunks_exact_mut(4) {
                        px[0] = 0;
                        px[1] = 0;
                        px[2] = 0;
                        px[3] = 0;
                    }
                    draw_circle(frame, size.width as usize, size.height as usize, x as i32, y as i32, 10);
                    pixels.render().unwrap();
                }
                _ => {}
            }
        }).unwrap();
    }
}

fn draw_circle(frame: &mut [u8], width: usize, _height: usize, cx: i32, cy: i32, radius: i32) {
    for y in -radius..=radius {
        for x in -radius..=radius {
            if x * x + y * y <= radius * radius {
                let px = cx + x;
                let py = cy + y;
                if px >= 0 && py >= 0 && (px as usize) < width && (py as usize) < width {
                    let idx = ((py as usize) * width + (px as usize)) * 4;
                    frame[idx] = 255;     // R
                    frame[idx + 1] = 255; // G
                    frame[idx + 2] = 0;   // B
                    frame[idx + 3] = 255; // A
                }
            }
        }
    }
}
