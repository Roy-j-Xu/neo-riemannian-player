use iced::Point;

use crate::core::NoteList;

use super::{NetOperation, NetOperator, NetState};


/// Unlike duel-Tonnetz, Tonnetz does not need another layer of state
pub struct TonnetzState {
    note_list: NoteList,
    rotational_center: i32,
}

impl NetState for TonnetzState { }

impl TonnetzState {
    pub fn new(note_list: &NoteList) -> TonnetzState {
        TonnetzState { 
            note_list: note_list.clone(),
            rotational_center: -1,
        }
    }
    
    fn initialize_layout(note_range: usize) -> Vec<Point<f32>> {
        let mut results = vec![Point::new(0.0, 0.0); note_range];
        
        for (index, point) in results.iter_mut().enumerate() {
            let note = index as i32;
            let x;
			let y = 6 - (3 + 2 * (note % 7)) % 7;
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
}


pub struct TonnetzOperator;

impl NetOperator<TonnetzState> for TonnetzOperator {
    fn supported_operations() -> Vec<String> {
        vec!["ParallelMove".to_string(), "Rotate".to_string()]
    }

    fn apply(net: TonnetzState, operation: NetOperation) -> Result<(), String> {
        match operation {
            NetOperation::ParallelMove(intervel) => net.note_list.parallel_move(intervel),
            NetOperation::Rotate { target, center } => {
                todo!()
            },
            _ => Err("Unsupported operation".to_string())
        }
    } 
}