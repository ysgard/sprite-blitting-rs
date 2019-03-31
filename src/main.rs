extern crate sdl2;

mod util;
mod window;

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::surface::Surface;
use sdl2::image::{LoadSurface, InitFlag};
use sdl2::rect::Rect;
use sdl2::render::TextureAccess;

const WINDOW_WIDTH: u32 = 1200;
const WINDOW_HEIGHT: u32 = 800;

pub fn main() -> Result<(), String> {

    let mut window = window::Window::new("Sprite Blitting", WINDOW_WIDTH, WINDOW_HEIGHT)?;

    // Load the spritesheet
    let _image_context = sdl2::image::init(InitFlag::PNG)?;
    let mut sprite_surface = Surface::from_file("assets/BrogueFont5.png")
        .map_err(|err| format!("failed to load spritesheet surface: {}", err.to_string()))?;
    // Convert black pixels to be transparent
    sprite_surface.set_color_key(true, Color::RGB(0, 0, 0))?;
    // Now convert it to a texture
    let texture_creator = window.canvas.texture_creator();
    let sprite_sheet = texture_creator.create_texture_from_surface(sprite_surface)
        .map_err(|err| format!("Failed to create spritesheet texture: {}", err.to_string()))?;

    // Create a 'punch' for getting the sprites out of the spritesheet
    let sprite_clip = Rect::new(0, 0, 18, 28);

    // Get the size of the window and the spritesheet
    let window_size = Rect::new(0, 0, window.width, window.height);
    let sq = sprite_sheet.query();
    let sheet_size = Rect::new(0, 0, sq.width, sq.height);

    // Create the clip rectangle
    let clip_rect = Rect::new(
        (window_size.w - sheet_size.w) / 2,
        (window_size.h - sheet_size.h) / 2,
        sheet_size.w as u32, sheet_size.h as u32);

    // Create a texture to store our changes in, so we can control when
    // we blit to the screen
    let mut buffer_tex = texture_creator.create_texture(
        None, TextureAccess::Target,
        window_size.w as u32, window_size.h as u32)
        .map_err(|err| format!("Failed to create texture: {}", err.to_string()))?;


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

        // Blit the entire spritesheet to the main window
        window.canvas.clear();
        window.canvas.copy(&sprite_sheet, None, Some(clip_rect))?;
        window.canvas.present();

        // Wait one second
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
