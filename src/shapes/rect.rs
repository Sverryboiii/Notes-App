use winit::window::Window;
use crate::general::general::in_screen;

pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub color: u32
}
impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32, color: u32) -> Rect {
        Rect { x, y, w, h, color }
    }

    pub fn draw(
        &self,
        buffer: &mut softbuffer::Buffer<&Window, &Window>,
        display_width: i32,
        display_height: i32
    ) {
        for y in self.y..(self.y + self.h) {
            for x in self.x..(self.x + self.w) {
                let index = y * display_width + x;
                if in_screen(x, y, display_width, display_height) {
                    continue
                }
                buffer[index as usize] = self.color;
            }
        }
    }
}