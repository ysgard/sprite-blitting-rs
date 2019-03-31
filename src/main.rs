extern crate sdl2;

mod util;
mod window;

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::surface::Surface;
use sdl2::image::{LoadSurface, InitFlag};

const WINDOW_WIDTH: u32 = 1200;
const WINDOW_HEIGHT: u32 = 800;

pub fn main() -> Result<(), String> {

    let mut window = window::Window::new("Sprite Blitting", WINDOW_WIDTH, WINDOW_HEIGHT)?;

    // Load the spritesheet
    let _image_context = sdl2::image::init(InitFlag::PNG)?;
    let mut sprite_surface = Surface::from_file("assets/BrogueFont5.png")
        .map_err(|err| format!("failed to load spritesheet!"))?;



    window.set_color(Color::RGB(0, 0, 0));
    window.draw();

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
