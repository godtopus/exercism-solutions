use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
pub enum Error {
    EmptyBuffer,
    FullBuffer
}

pub struct CircularBuffer<T: Clone> {
    capacity: usize,
    data: VecDeque<T>
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            capacity: capacity,
            data: VecDeque::with_capacity(capacity)
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.data.is_empty() {
            return Err(Error::EmptyBuffer)
        }

        Ok(self.data.pop_front().unwrap())
    }

    pub fn write(&mut self, entry: T) -> Result<usize, Error> {
        if self.data.len() == self.capacity {
            return Err(Error::FullBuffer)
        }

        self.data.push_back(entry);
        Ok(self.data.len())
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn overwrite(&mut self, entry: T) {
        if self.data.len() == self.capacity {
            self.data.pop_front().unwrap();
        }

        self.write(entry).unwrap();
    }
}