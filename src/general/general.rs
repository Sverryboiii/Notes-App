use rusttype::{point, Font, PositionedGlyph, Scale};
use winit::window::Window;

pub fn in_screen(x: i32, y: i32, window_width: i32, window_height: i32) -> bool {
    y >= window_height || y <= 0 || x >= window_width || x <= 0
}

pub fn get_font() -> Font<'static> {
    Font::try_from_vec(
        std::fs::read("src/fonts/pixelated.ttf").unwrap()
    ).expect("Could not load font data!")
}

pub fn get_text_gylphs(text: Box<str>, scale: f32, x: f32, y: f32) -> Vec<PositionedGlyph<'static>> {
    let font = get_font();
    let scale = Scale::uniform(scale);
    let point_ = point(x, y);
    font.layout(&text, scale, point_).collect()
}

pub fn draw_text(
    text: Box<str>,
    scale: f32,
    x: f32,
    y: f32,
    buffer: &mut softbuffer::Buffer<&Window, &Window>
) {
    let width = buffer.width().get() as i32;
    let height = buffer.height().get() as i32;
    let gylphs = get_text_gylphs(text, scale, x, y);

    for glyph in &gylphs {
        let Some(bounds) = glyph.pixel_bounding_box() else {
            continue
        };

        glyph.draw(| gx, gy, coverage | {
            let x = bounds.min.x + gx as i32;
            let y = bounds.min.y + gy as i32;

            if in_screen(x, y, width, height) {
                return
            }

            let index = y * width + x;

            let background_px = buffer[index as usize];
            let bg_r = ((background_px >> 16) & 0xff) as f32;
            let bg_g = ((background_px >> 8) & 0xff) as f32;
            let bg_b = (background_px & 0xff) as f32;

            let r = (255.0 * coverage + bg_r * (1.0 - coverage)) as u32;
            let g = (255.0 * coverage + bg_g * (1.0 - coverage)) as u32;
            let b = (255.0 * coverage + bg_b * (1.0 - coverage)) as u32;

            buffer[index as usize] = (r << 16) | (g << 8) | b;
        })
    }
}

pub static mut MOUSE_POSITION: [i32; 2] = [0, 0];
pub static mut MOUSE_PRESSED: bool = false;