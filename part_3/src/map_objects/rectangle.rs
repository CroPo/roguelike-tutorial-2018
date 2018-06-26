pub struct Rect {
    pub tl: (i32, i32),
    pub lr: (i32, i32),
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            tl: (x, y),
            lr: (x + w, y + h),
        }
    }
}
