use winit::{event_loop::EventLoop, window::WindowBuilder};
use winit::event::{Event, WindowEvent};
use std::num::NonZeroU32;

pub struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    color: u32
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32, color: u32) -> Rect {
        Rect { x, y, w, h, color }
    }

    pub fn draw(
        &self,
        surface: &mut softbuffer::Surface<&winit::window::Window, &winit::window::Window>,
        display_width: i32,
        display_height: i32
    ) {
        let mut buffer = surface.buffer_mut().unwrap();
        for y in self.y..(self.y + self.h) {
            for x in self.x..(self.x + self.w) {
                let index = y * display_width + x;
                if x < display_width && y < display_height {
                    buffer[index as usize] = self.color;
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut display_width: u32 = 500;
    let mut display_height: u32 = 500;
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let context = softbuffer::Context::new(&window)?;
    let mut surface = softbuffer::Surface::new(&context, &window)?;
    surface.resize(NonZeroU32::new(display_width).unwrap(), NonZeroU32::new(display_height).unwrap())?;

    let r = Rect::new(3, 500, 1000, 60, 0xff00ff);

    event_loop.run(move |event, elwt| {
        match event {Event::WindowEvent {event, ..} => match event {
            WindowEvent::CloseRequested => elwt.exit(),
            WindowEvent::RedrawRequested => {
                surface.buffer_mut().unwrap().fill(0x0f0f0f);
                r.draw(&mut surface, display_width as i32, display_height as i32);

                surface.buffer_mut().unwrap().present().expect("TODO: panic message");
            }
            WindowEvent::Resized(physical_size) => {
                display_height = physical_size.height;
                display_width = physical_size.width;
                surface.resize(NonZeroU32::new(display_width).unwrap(), NonZeroU32::new(physical_size.height).unwrap()).unwrap();
            } _ => {}
        } _ => {}}
    })?;

    Ok(())
}