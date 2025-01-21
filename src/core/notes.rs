use crate::NUM_OF_NOTES;


#[derive(Debug)]
pub struct Notes(Vec<bool>);

impl Default for Notes {
    fn default() -> Self {
        Notes(vec![false; NUM_OF_NOTES])
    }
}


impl Notes {
    pub fn new() -> Self {
        Notes::default()
    }

    pub fn press(&mut self, note: usize) -> Result<bool, String> {
        if note < NUM_OF_NOTES {
            let target = !self.0[note];
            self.0[note] = target;
            Ok(target)
        } else {
            Err(format!("Note \"{}\" exceeds the total number of notes", note))
        }
    }
}