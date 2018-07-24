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

    pub fn center(&self) -> (i32, i32) {
        (
            (self.tl.0 + self.lr.0) / 2,
            (self.tl.1 + self.lr.1) / 2,
        )
    }

    pub fn intersect(&self, other : &Rect) -> bool {
        self.tl.0 <= other.lr.0 && self.lr.0 >= other.tl.0 &&
        self.tl.1 <= other.lr.1 && self.lr.1 >= other.tl.1
    }

}
