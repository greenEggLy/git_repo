// https://exercism.org/tracks/rust/exercises/circular-buffer

#[derive(Debug)]
pub struct CircularBuffer<T> {
    buffer: Vec<T>,
    read: usize,
    write: usize,
}

impl CircularBuffer<String> {
    pub fn new(size: usize) -> Self {
        CircularBuffer {
            buffer: vec![String::from("\0"); size],
            read: 0,
            write: 0,
        }
    }

    pub fn write(&mut self, _element: String) -> Result<(), &str> {
        if self.buffer[self.write] != "\0" {
            return Err("buffer full");
        }
        self.buffer[self.write] = _element;
        self.write = (self.write + 1) % self.buffer.len();
        Ok(())
    }

    pub fn force_write(&mut self, _element: String) {
        self.buffer[self.write] = _element;
        self.write = (self.write + 1) % self.buffer.len();
        self.read = self.write;
    }

    pub fn read(&mut self) -> Result<String, &str> {
        if self.buffer[self.read] == "\0" {
            return Err("buffer empty");
        }
        let result = self.buffer[self.read].clone();
        self.buffer[self.read] = String::from("\0");
        self.read = (self.read + 1) % self.buffer.len();
        Ok(result)
    }
}
