use iced::Point;



pub struct VisualState
{
    layout: Vec<Point<f32>>,
    transform: Transform,
}

/// Used in musical net for 
#[derive(Clone, Debug)]
pub struct Transform {
    pub ratio: Point<f32>,
    pub shift: Point<f32>,
}

impl Transform {
    pub fn apply(&self, point: &Point<f32>) -> Point<f32> {
        Point::<f32>::new(self.ratio.x * point.x + self.shift.x, self.ratio.y * point.y + self.shift.y)
    }

    pub fn reverse(&self, point: &Point<f32>) -> Point<f32> {
        Point::<f32>::new((point.x - self.shift.x) / self.ratio.x, (point.y - self.shift.y) / self.ratio.y)
    }
}