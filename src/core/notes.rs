use std::sync::{Arc, Mutex};

use crate::NUM_OF_NOTES;

type Notes = Vec<bool>;

#[derive(Debug)]
pub struct NoteList(Arc<Mutex<Notes>>);

impl Default for NoteList {
    fn default() -> Self {
        NoteList(Arc::new(Mutex::new(vec![false; NUM_OF_NOTES])))
    }
}

impl Clone for NoteList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}


impl NoteList {
    pub fn new() -> Self {
        NoteList::default()
    }

    pub fn press(&self, note: usize) -> Result<bool, String> {
        if note >= NUM_OF_NOTES {
            return Err(format!("Note \"{}\" exceeds the total number of notes", note));
        }

        let mut note_list;

        match self.0.lock() {
            Ok(notes) => note_list = notes,
            Err(_) => return Err("Thread poisoned".to_string())
        }

        note_list[note] = !note_list[note];

        Ok(note_list[note])
    }

}


#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn threads() {
        let note_list = NoteList::new();
        print!("{:?}", note_list);

        let mut handle = vec![];

        for i in 0..10 {
            let thread_note_list = note_list.clone();
            let thread = thread::spawn(move || {
                let list = thread_note_list;
                let _ = list.press(i);
                print!("Pressed {i}\n");
            });
            handle.push(thread);
        }

        handle.into_iter().for_each(|thread| { thread.join().unwrap(); });
        print!("{:?}", note_list);
    }

}