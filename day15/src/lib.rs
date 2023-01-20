use std::cmp::{min, max};

pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point {x, y}
    }

    pub fn manhattan_distance(&self, other: Point) -> i32 {
        (max(self.x, other.x) - min(self.x, other.x)) + 
        (max(self.y, other.y) - min(self.y, other.y))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manhattan() {
        let p1 = Point::new(5, -2);
        let p2 = Point::new(2, 4);
        assert_eq!(p1.manhattan_distance(p2), 9);
    }

}
