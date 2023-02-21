use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};
use rand::random;


#[derive(Debug)]
pub struct Tape {
    // The tape is a vector of cells
    cells: HashMap<usize, u8>,
    // The current position of the tape head
    pointer: usize,
}

impl Tape {
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
            pointer: 0,
        }
    }

    pub fn get(&self) -> u8 {
        *self.cells.get(&self.pointer).unwrap_or(&0)
    }

    pub fn set(&mut self, value: u8) {
        self.cells.insert(self.pointer, value);
    }

    pub fn increment(&mut self) {
        self.cells.insert(self.pointer, self.get().checked_add(1).unwrap_or(0));
    }

    pub fn decrement(&mut self) {
        self.cells.insert(self.pointer, self.get().checked_sub(1).unwrap_or(255));
    }

    pub fn move_right(&mut self) {
        self.pointer += 1;
    }

    pub fn move_left(&mut self) {
        self.pointer = self.pointer.checked_sub(1).unwrap_or(0);
    }

    pub fn set_random(&mut self) {
        self.set(random::<u8>());
    }
}

impl Display for Tape {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let value = self.get() as char;
        write!(f, "{}", value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_initial_state() {
        let tape = Tape::new();
        assert_eq!(tape.get(), 0);
    }

    #[test]
    fn increment() {
        let mut tape = Tape::new();
        tape.increment();
        assert_eq!(tape.get(), 1);
    }

    #[test]
    fn decrement() {
        let mut tape = Tape::new();
        tape.decrement();
        assert_eq!(tape.get(), 255);
    }

    #[test]
    fn move_pointer() {
        let mut tape = Tape::new();
        tape.move_right();
        assert_eq!(tape.get(), 0);
        assert_eq!(tape.pointer, 1);
        tape.move_left();
        assert_eq!(tape.get(), 0);
        assert_eq!(tape.pointer, 0);
    }

    #[test]
    fn set_random() {
        let mut tape = Tape::new();
        tape.set_random();
        assert_ne!(tape.get(), 0);
    }

    #[test]
    fn display() {
        let mut tape = Tape::new();
        tape.set(65);
        assert_eq!(tape.to_string(), "A");
    }

    #[test]
    fn set() {
        let mut tape = Tape::new();
        tape.set(65);
        assert_eq!(tape.get(), 65);
        tape.set(66);
        assert_eq!(tape.get(), 66);
        tape.set(67);
        assert_eq!(tape.get(), 67);
    }
}
