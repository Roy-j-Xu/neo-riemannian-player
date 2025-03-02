use iced::Point;
use net_components::{NetOperation, Transform};

mod net_components;
mod tonnetz;

pub trait MusicalNet {
    fn supported_operations() -> Vec<String>;
    fn apply_operation(operation: NetOperation) -> Result<(), String>;
    fn set_transform(transform: Transform);
    fn press(point: Point);
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