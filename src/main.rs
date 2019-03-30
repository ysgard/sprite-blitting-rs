extern crate sdl2;

mod window;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

const WINDOW_WIDTH: u32 = 1200;
const WINDOW_HEIGHT: u32 = 800;

pub fn main() -> Result<(), String> {

    let mut window = window::Window::new("Sprite Blitting", WINDOW_WIDTH, WINDOW_HEIGHT)?;

    // Initialize SDL2
    //let sdl_context = sdl2::init()?;
    //let video_subsystem = sdl_context.video()?;

    // Create a window
    //let window = video_subsystem.window("Sprite Blitting", WINDOW_WIDTH, WINDOW_HEIGHT)
    //    .position_centered()
    //    .opengl()
    //    .build()
    //    .map_err(|e| e.to_string())?;

    //let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    window.canvas.set_draw_color(Color::RGB(255, 0, 0));
    window.canvas.clear();
    window.canvas.present();

    // Main event loop
    'running: loop {
        for event in window.event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
