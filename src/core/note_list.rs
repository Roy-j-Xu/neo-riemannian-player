use std::{collections::VecDeque, sync::{Arc, Mutex, MutexGuard}};

use crate::configs;

type Notes = Vec<bool>;

#[derive(Debug)]
pub struct NoteList(Arc<Mutex<Notes>>);

impl Default for NoteList {
    fn default() -> Self {
        NoteList::of_size(configs::NOTE_RANGE)
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

    pub fn of_size(size: usize) -> Self {
        NoteList(Arc::new(Mutex::new(vec![false; size])))
    }

    pub fn size(&self) -> usize {
        self.inner().len()
    }


    pub fn press(&self, note: usize) -> Result<bool, String> {
        if note >= self.size() {
            return Err(format!("Note \"{}\" exceeds the total number of notes", note));
        }

        let mut note_list = self.inner();

        note_list[note] = !note_list[note];

        Ok(note_list[note])
    }

    pub fn release_all(&self) {
        let mut note_list = self.inner();
        for note in note_list.iter_mut() {
            *note = false;
        }
    }

    pub fn press_all(&self, notes: Vec<usize>) -> Result<(), String> {
        let result: Result<Vec<bool>, String> = 
            notes.into_iter()
            .map(|note| self.press(note))
            .collect();

        match result {
            Ok(_) => Ok(()),
            Err(str) => Err(str)
        }
    }


    // --------------
    // Operations
    // --------------

    pub fn parallel_move(&self, intervel: i32) -> Result<(), String> {
        let size = self.inner().len() as i32;
        let mut note_list = self.inner();

        let mut moved_notes = VecDeque::<usize>::new();
        for (index, pressed) in note_list.iter_mut().enumerate() {
            if *pressed {
                let mut moved_note_i32 = (index as i32 + (intervel % size)) % size;
                if moved_note_i32 < 0 { moved_note_i32 += size; }
                moved_notes.push_back(moved_note_i32.try_into().unwrap());
                *pressed = false;
            }
        }

        while let Some(note) = moved_notes.pop_front() {
            note_list[note] = true;
        }

        Ok(())
    }

    pub fn inner(&self) -> MutexGuard<Notes> {
        self.0.lock().unwrap()
    }

}



#[cfg(test)]
mod note_list_tests {
    use std::thread;

    use super::*;

    #[test]
    fn threads_press() {
        let size = 10;
        let note_list = NoteList::of_size(size);
        println!("{:?}", note_list);

        let mut handle = vec![];

        for i in 0..10 {
            let thread_note_list = note_list.clone();
            let thread = thread::spawn(move || {
                let list = thread_note_list;
                let _ = list.press(i);
                println!("Pressed {i}");
            });
            handle.push(thread);
        }

        handle.into_iter().for_each(|thread| { thread.join().unwrap(); });
        print!("{:?}", note_list);
        
        assert_eq!(*note_list.inner(), vec![true; 10]);
    }

    #[test]
    fn threads_parallel_move() {
        let size = 6;
        let note_list = NoteList::of_size(size);
        note_list.press_all(vec![0, 3]).unwrap();
        println!("{:?}", note_list);

        let mut handle = vec![];

        for i in 0..5 {
            let thread_note_list = note_list.clone();
            let thread = thread::spawn(move || {
                let list = thread_note_list;
                list.parallel_move(1).unwrap();
                println!("Thread {}: {:?}", i, list.inner());
            });
            handle.push(thread);
        }

        handle.into_iter().for_each(|thread| { thread.join().unwrap(); });

        assert_eq!(*note_list.inner(), vec![false, false, true, false, false, true]);
    }


}