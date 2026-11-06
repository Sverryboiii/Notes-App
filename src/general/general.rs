
pub fn in_screen(x: i32, y: i32, window_width: i32, window_height: i32) -> bool {
    return y >= window_height || y <= 0 || x >= window_width || x <= 0;
}