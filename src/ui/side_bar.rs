use crate::shapes::rect::Rect;

pub struct SideBar {
    rect: Rect
}

impl SideBar {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> SideBar {
        let rect = Rect::new(x, y, w, h, 0x1f1f1f);
        SideBar {
            rect
        }
    }

    pub fn frame(
        &self,
        window: &mut softbuffer::Surface<&winit::window::Window, &winit::window::Window>,
        window_width: i32,
        window_height: i32
    ) {
        self.rect.draw(window, window_width, window_height)
    }
}