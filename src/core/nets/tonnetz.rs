use iced::widget::canvas::path::lyon_path::geom::euclid::Point2D;

use crate::core::nets::MusicalNet;
use crate::core::NoteList;


pub struct Tonnetz;

impl MusicalNet for Tonnetz {

    fn layout(&self, note_list: &NoteList) -> Vec<Point2D<f32, f32>> {
        let note_range = note_list.size();
        let mut results = vec![Point2D::new(0.0, 0.0); note_range];

        for (index, point) in results.iter_mut().enumerate() {
            let note = index as i32;
            let x;
			let y= 6 - (3 + 2 * (note % 7)) % 7;
			if note % 7 == 6 || note % 7 == 0 || note % 7 == 1 {
				x = ((note + 1)/7) * 2;
			} else {
				x = (note/7) * 2 + 1;
			}

            point.x = x as f32;
            point.y = y as f32;
		}
        
        results

    }

    fn press(&self, note_list: &NoteList, location: f32) -> Result<bool, String> {
        todo!()
    }

    fn parallel_move(&self, note_list: &NoteList, intervel: usize) {
        todo!()
    }

    fn rotate(&self, note_list: &NoteList, center_note: usize, conter_clockwise: bool) {
        todo!()
    }
}