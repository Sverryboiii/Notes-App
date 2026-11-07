mod button;
mod general;
mod shapes;
mod ui;

use ui::side_bar::SideBar;
use ui::icon::Icon;
use ui::component::Item;
use general::general::{ MOUSE_POSITION, MOUSE_PRESSED, draw_text };
use general::config::{ SIDE_BAR_WIDTH };

use std::num::NonZeroU32;
use std::time::Instant;
use winit::event::{ElementState, Event, KeyEvent, WindowEvent, MouseButton};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::{event_loop::EventLoop, window::WindowBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut display_width: u32 = 500;
    let mut display_height: u32 = 500;

    let mut time = Instant::now();

    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new().build(&event_loop)?;
    let context = softbuffer::Context::new(&window)?;
    let mut surface = softbuffer::Surface::new(&context, &window)?;
    surface.resize(NonZeroU32::new(display_width).unwrap(), NonZeroU32::new(display_height).unwrap())?;

    window.focus_window();

    let win = &window;

    let mut control_pressed: bool = false;
    let mut sidebar = SideBar::new(0, 0, SIDE_BAR_WIDTH, display_width as i32);

    sidebar.add_component(Item::Icon(Icon::new()));

    event_loop.run(move |event, elwt| {
        let _delta_time = time.elapsed().as_secs_f64();
        time = Instant::now();

        match event {Event::WindowEvent {event, ..} => match event {
            WindowEvent::CloseRequested => elwt.exit(),
            WindowEvent::RedrawRequested => {
                let mut buffer = surface.buffer_mut().unwrap();
                buffer.fill(0x0f0f0f);

                sidebar.frame(&mut buffer, display_width as i32, display_height as i32);
                draw_text(
                    String::from("Your notes:").into_boxed_str(), 20.0, (SIDE_BAR_WIDTH+10) as f32, 20.0, &mut buffer
                );

                buffer.present().expect("TODO: panic message");
            },
            WindowEvent::Resized(physical_size) => {
                display_height = physical_size.height;
                display_width = physical_size.width;
                surface.resize(NonZeroU32::new(display_width).unwrap(), NonZeroU32::new(physical_size.height).unwrap()).unwrap();
            },
            WindowEvent::CursorMoved {
                position, ..
            } => unsafe {
                MOUSE_POSITION = <[i32; 2]>::from(position);
            }
            WindowEvent::MouseInput {
                state, button, ..
            } => unsafe {
                match state {
                    ElementState::Pressed => {
                        match button {
                            MouseButton::Left => {
                                MOUSE_PRESSED = true
                            },
                            _ => {}
                        }
                    },
                    ElementState::Released => {
                        match button {
                            MouseButton::Left => {
                                MOUSE_PRESSED = false
                            },
                            _ => {}
                        }
                    }
                }
            },
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    physical_key,
                    state,
                    ..
                }, device_id: _device_id, is_synthetic: _is_synthetic
            } => {
                if let PhysicalKey::Code(keycode) = physical_key {
                    match (keycode, state) {
                        (KeyCode::ControlLeft, ElementState::Pressed) => {
                            control_pressed = true;
                        }
                        (KeyCode::Escape, ElementState::Pressed) => {
                            if control_pressed {
                                elwt.exit()
                            }
                        }
                        (KeyCode::KeyA, ElementState::Pressed) => {
                        }
                        _ => {}
                    }
                };
            }
            _ => {}
        } _ => {}}
        win.request_redraw();
    })?;

    Ok(())
}