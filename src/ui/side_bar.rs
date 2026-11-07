use winit::window::Window;
use crate::general::config::SIDE_BAR_WIDTH;
use crate::shapes::rect::Rect;
use crate::ui::component::{ Component, Item };

pub struct SideBar {
    pub rect: Rect,
    pub components: [Component; 10],
    pub current_component: i32
}

impl SideBar {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> SideBar {
        let rect = Rect::new(x, y, w, h, 0x3f3f3f);
        let components: [Component; 10] = [
            Component::new(10, 0x0000ff),
            Component::new(10 + SIDE_BAR_WIDTH, 0x0000ff),
            Component::new(10 + 2 * SIDE_BAR_WIDTH, 0x0000ff),
            Component::new(10 + 3 * SIDE_BAR_WIDTH, 0x0000ff),
            Component::new(10 + 4 * SIDE_BAR_WIDTH, 0x0000ff),

            Component::new(10 + 5 * SIDE_BAR_WIDTH, 0x0000ff),
            Component::new(10 + 6 * SIDE_BAR_WIDTH, 0x0000ff),
            Component::new(10 + 7 * SIDE_BAR_WIDTH, 0x0000ff),
            Component::new(10 + 8 * SIDE_BAR_WIDTH, 0x0000ff),
            Component::new(10 + 9 * SIDE_BAR_WIDTH, 0x0000ff)
        ];
        let current_component = 0;
        SideBar {
            rect,
            components,
            current_component
        }
    }

    pub fn frame(
        &mut self,
        buffer: &mut softbuffer::Buffer<&Window, &Window>,
        window_width: i32,
        window_height: i32
    ) {
        self.rect.h = window_height;
        self.rect.draw(buffer, window_width, window_height);
        for comp in self.components.iter() {
            comp.frame(buffer, window_height, window_width);
        }
    }

    pub fn add_component(&mut self, element: Item) {
        self.components[self.current_component as usize].element = element;
        self.current_component += 1;
    }
}