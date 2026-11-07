use winit::window::Window;
use crate::ui::icon::Icon;
use crate::shapes::rect::Rect;
use crate::general::config::{ SIDE_BAR_WIDTH };

pub struct Empty {}

impl Empty {
    pub fn new() -> Empty {
        Empty {}
    }
}

pub enum Item {
    Empty(Empty),
    Icon(Icon)
}

pub struct Component {
    pub element: Item,
    pub rect: Rect
}

impl Component {
    pub fn new(y: i32, color: u32) -> Component {
        let element = Item::Empty(Empty::new());
        let rect = Rect::new(10, y, SIDE_BAR_WIDTH-20, SIDE_BAR_WIDTH-20, color);
        Component { element, rect }
    }

    pub fn frame(
        &self,
        buffer: &mut softbuffer::Buffer<&Window, &Window>,
        window_width: i32,
        window_height: i32
    ) {
        self.rect.draw(buffer, window_height, window_width);
    }
}