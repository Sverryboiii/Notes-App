use crate::general::general::in_screen;

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub rad: f64,
    pub color: u32
}
impl Circle {
    pub fn new(x: f64, y: f64, rad: f64, color: u32) -> Circle {
        Circle { x, y, rad, color }
    }

    pub fn collide_point(&self, x: f64, y: f64) -> bool {
        (x - self.x) * (x - self.x) + (y - self.y) * (y - self.y) >= self.rad * self.rad
    }

    pub fn draw(
        &self,
        surface: &mut softbuffer::Surface<&winit::window::Window, &winit::window::Window>,
        display_width: i32,
        display_height: i32
    ) {
        let mut buffer = surface.buffer_mut().unwrap();
        for y in (self.y - self.rad) as i32..(self.y + self.rad + 1.0) as i32 {
            for x in (self.x - self.rad) as i32..(self.x + self.rad + 1.0) as i32 {
                if in_screen(x, y, display_width, display_height) {
                    continue
                }
                if self.collide_point(x as f64, y as f64) {
                    continue
                }
                let index = y * display_width + x;
                buffer[index as usize] = self.color;
            }
        }
    }
}