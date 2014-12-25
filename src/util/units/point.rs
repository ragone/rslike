use util::units::{AsTuple, Direction};

#[deriving(Show, Copy)]
pub struct Point {
    pub x: int,
    pub y: int
}

impl Point {

    pub fn new(x: int, y: int) -> Point {
        Point { x: x, y: y }
    }

    pub fn zero() -> Point {
        Point { x: 0, y: 0 }
    }

    pub fn move_dir(&self, dir: Direction) -> Point {
        *self + dir.as_tuple()
    }

}

impl Add<Point, Point> for Point {
    fn add(self, other: Point) -> Point {
        let (ox, oy) = other.as_tuple();
        Point { x: self.x + ox, y: self.y + oy }
    }
}

impl Add<(int, int), Point> for Point {
    fn add(self, other: (int, int)) -> Point {
        let (ox, oy) = other;
        Point { x: self.x + ox, y: self.y + oy }
    }
}

impl AsTuple<int> for Point {
    fn as_tuple(&self) -> (int, int) {
        (self.x, self.y)
    }
}