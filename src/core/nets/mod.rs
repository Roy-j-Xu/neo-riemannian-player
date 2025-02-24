use iced::widget::canvas::path::lyon_path::geom::euclid::Point2D;

use crate::core::NoteList;

mod tonnetz;


pub use tonnetz::Tonnetz;


/// A musical net is a series of strategies to interact with `NoteList`
pub trait MusicalNet {
    
    /// Provide data for visualization, using unit length. This data requires further
    /// mapping in order to be presented by iced
    fn layout(&self) -> Vec<Point2D<f32, f32>>;

    /// Press the given location of the net
    fn press(&self, location: f32) -> Result<bool, String>;
    
    fn parallel_move(&self, intervel: usize);

    fn rotate(&self, center_note: usize, conter_clockwise: bool);


}


#[cfg(test)]
mod musical_net_tests {
    use super::*;

    // helper function to print out a net on terminal
    fn print_net(layout: Vec<Point2D<f32, f32>>) {
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