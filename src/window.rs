pub struct Window {
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub event_pump: sdl2::EventPump,
    pub width: u32,
    pub height: u32,
}

impl Window {
    pub fn new(title: &str,
               width: u32,
               height: u32
    ) -> Result<Window, String> {
        let context = sdl2::init()?;
        let video = context.video()?;
        let window = video.window(title, width, height)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let event_pump = context.event_pump()?;
        Ok(Window {
            canvas: canvas,
            event_pump: event_pump,
            width: width,
            height: height,
        })
    }
}