use iced::Point;
use crate::core::types::{
    Blueprint,
    Transform
};



// --- First component: net state --
pub trait NetState { }


// --- Second component: net operator ---

pub enum NetOperation {
    ParallelMove(i32),
    Rotate{ target: i32, center: i32 },
}

pub trait NetOperator<T> where T: NetState {
    fn supported_operations() -> Vec<String>;
    fn apply(net: T, operation: NetOperation) -> Result<(), String>;
}



// --- Third component: visual state ---



pub struct VisualState
{
    blueprint: Blueprint,
    transform: Transform,
    layout: Blueprint,
}

impl VisualState {
    pub fn new(blueprint: Blueprint) -> Self {
        let layout = blueprint.clone();
        VisualState {
            blueprint,
            transform: Transform::default(),
            layout,
        }
    }

    fn update_layout(&mut self) {
        for (index, point) in self.blueprint.iter().enumerate() {
            self.layout[index] = self.transform.apply(point);
        }
    }

    pub fn update_ratio(&mut self, x: f32, y: f32) {
        self.transform.ratio.x = x;
        self.transform.ratio.y = y;
        self.update_layout();
    }

    pub fn update_shift(&mut self, x: f32, y: f32) {
        self.transform.shift.x = x;
        self.transform.shift.y = y;
        self.update_layout();
    }
}