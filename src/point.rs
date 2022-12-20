#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn x_distance(&self, point_b: &Point) -> u32 {
        self.x.abs_diff(point_b.x)
    }

    pub fn y_distance(&self, point_b: &Point) -> u32 {
        self.y.abs_diff(point_b.y)
    }

    pub fn manhattan_distance(&self, point_b: &Point) -> u32 {
        self.x_distance(point_b) + self.y_distance(point_b)
    }

    pub fn distance(&self, point_b: &Point) -> f32 {
        let x_distance = self.x_distance(point_b);
        let y_distance = self.y_distance(point_b);
        ((x_distance as f32).powi(2) + (y_distance as f32).powi(2)).sqrt()
    }
}
