use iced::Point;

use crate::core::NoteList;

mod tonnetz;


/// --- First component: net state --

pub trait NetState { }



/// --- Second component: net operator ---

pub enum NetOperation {
    ParallelMove(i32),
    Rotate{ target: i32, center: i32 },
}

pub trait NetOperator<T> where T: NetState {
    fn supported_operations() -> Vec<String>;
    
    fn apply(net: T, operation: NetOperation) -> Result<(), String>;
}



/// --- Third component: visual state ---

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


type Blueprint = Vec<Point<f32>>;

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
            blueprint: blueprint,
            transform: Transform::default(),
            layout: layout,
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




#[cfg(test)]
mod musical_net_tests {
    use super::*;

    // helper function to print out a net on terminal
    fn print_net(layout: Vec<Point<f32>>) {
        let width = layout.iter().fold(0, |acc, point| {acc.max(point.x as usize)});
        let height = layout.iter().fold(0, |acc, point| {acc.max(point.y as usize)});
        let mut screen = vec![vec![" ".to_string(); width + 1]; height + 1];

        for (index, point) in layout.iter().enumerate() {
            let x = point.x as usize;
            let y = point.y as usize;
            screen[y][x] = index.to_string();
        }

        for char_vec in screen.into_iter() {
            let line: String = char_vec.into_iter().collect();
            println!("{}", line);
        }
    }

    #[test]
    fn layout_tonnetz() {
        let note_list = NoteList::of_size(40);
        let tonnetz = Tonnetz::new(&note_list);

        let layout = tonnetz.layout();

        print_net(layout);
    }

}