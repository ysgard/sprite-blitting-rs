extern crate sdl2;
extern crate rand;

mod window;

use rand::Rng;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::surface::Surface;
use sdl2::image::{LoadSurface, InitFlag};
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, TextureAccess};

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
    let mut sprite_sheet = texture_creator.create_texture_from_surface(sprite_surface)
        .map_err(|err| format!("Failed to create spritesheet texture: {}", err.to_string()))?;
    // Set the blend mode to ADD
    sprite_sheet.set_blend_mode(BlendMode::Add);

    // Create a 'punch' for getting the sprites out of the spritesheet
    let sprite_clip = Rect::new(0, 0, 18, 28);

    // Get the size of the window and the spritesheet
    let window_size = Rect::new(0, 0, window.width, window.height);

    // Create a texture to store our changes in, so we can control when
    // we blit to the screen
    let mut buffer_tex = texture_creator.create_texture(
        None, TextureAccess::Target,
        window_size.w as u32, window_size.h as u32)
        .map_err(|err| format!("Failed to create texture: {}", err.to_string()))?;


    window.set_color(Color::RGB(0, 0, 0));
    window.draw();

    let mut old_seconds = 0;
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
        // window.canvas.clear();
        // window.canvas.copy(&sprite_sheet, None, Some(clip_rect))?;
        // window.canvas.present();

        let seconds = window.timer.ticks() / 1000;

        // Set the render target to be buffer_tex
        if seconds > old_seconds {
            window.canvas.with_texture_canvas(&mut buffer_tex, |tex| {
                tex.set_draw_color(Color::RGBA(0, 0, 0, 255));
                tex.clear();
                for i in 0 .. (window_size.w / sprite_clip.w) + 1 {
                    for j in 0 .. (window_size.h / sprite_clip.h) + 1 {
                        // Generate a random glyph. NB: gen_range is [n, m)
                        let glyph_x = rand::thread_rng().gen_range(0, 16);
                        let glyph_y = rand::thread_rng().gen_range(3, 16);
                        // Random color
                        let foreground_color = Color::RGBA(
                            rand::thread_rng().gen_range(0, 255),
                            rand::thread_rng().gen_range(0, 255),
                            rand::thread_rng().gen_range(0, 255),
                            0);
                        let background_color = Color::RGBA(
                            rand::thread_rng().gen_range(0, 255),
                            rand::thread_rng().gen_range(0, 255),
                            rand::thread_rng().gen_range(0, 255),
                            0);
                        // Generate the source and the destination clipping rects
                        let dest_rect = Rect::new(i * 18, j * 28, 18, 28);
                        let src_rect = Rect::new(glyph_x * 18, glyph_y * 28, 18, 28);
                        // Random color for sprite background
                        tex.set_draw_color(background_color);
                        tex.fill_rect(dest_rect).unwrap();
                        // Blit the sprite
                        sprite_sheet.set_color_mod(foreground_color.r,
                                                   foreground_color.g,
                                                   foreground_color.b);
                        tex.copy(&sprite_sheet, src_rect, dest_rect).unwrap();
                    }
                }
            }).map_err(|err| format!("Error blitting to buffer_tex: {}", err.to_string()))?;
        }
        old_seconds = seconds;
        window.canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        window.canvas.clear();
        window.canvas.copy(&buffer_tex, None, None)?;
        window.canvas.present();
    }
    Ok(())
}
