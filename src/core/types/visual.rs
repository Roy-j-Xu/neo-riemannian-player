use iced::Point;

#[derive(Clone, Debug)]
pub struct Transform {
    pub ratio: Point<f32>,
    pub shift: Point<f32>,
}

impl Default for Transform {
    fn default() -> Self {
        Self { ratio: Point::new(1.0, 1.0), shift: Point::new(0.0, 0.0) }
    }
}

impl Transform {

    pub fn apply(&self, point: &Point<f32>) -> Point<f32> {
        Point::<f32>::new(self.ratio.x * point.x + self.shift.x, self.ratio.y * point.y + self.shift.y)
    }
    
    pub fn reverse(&self, point: &Point<f32>) -> Point<f32> {
        Point::<f32>::new((point.x - self.shift.x) / self.ratio.x, (point.y - self.shift.y) / self.ratio.y)
    }
}


pub type Blueprint = Vec<Point<f32>>;