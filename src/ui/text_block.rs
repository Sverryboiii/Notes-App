// use crate::shapes::rect::Rect;
// 
// pub struct TextBlock {
//     pub rect: Rect,
//     pub text: String
// }
// 
// impl TextBlock {
//     pub fn new(x: i32, y: i32, w: i32, h: i32, color: u32, text: String) -> TextBlock {
//         let rect = Rect::new(x, y, w, h, color);
//         TextBlock { rect, text }
//     }
// 
//     pub fn frame(
//         &self,
//         window: &mut softbuffer::Surface<&winit::window::Window, &winit::window::Window>,
//         window_width: i32,
//         window_height: i32
//     ) {
//         self.rect.draw(window, window_width, window_height)
//     }
// }