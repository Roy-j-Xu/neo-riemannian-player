use iced::widget::canvas::path::lyon_path::geom::euclid::Point2D;

use crate::core::nets::MusicalNet;
use crate::core::NoteList;


/// Unlike duel-Tonnetz, Tonnetz does not need another layer of state
pub struct Tonnetz {
    note_list: NoteList,
}


impl Tonnetz {
    pub fn new(note_list: &NoteList) -> Tonnetz {
        Tonnetz{ note_list: note_list.clone() }
    }
}


impl MusicalNet for Tonnetz {

    fn layout(&self) -> Vec<Point2D<f32, f32>> {
        let note_range = self.note_list.size();
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
    
    fn press(&self, location: f32) -> Result<bool, String> {
        todo!()
    }
    
    fn parallel_move(&self, intervel: usize) {
        todo!()
    }
    
    fn rotate(&self, center_note: usize, conter_clockwise: bool) {
        todo!()
    }
    
}