pub trait Input {
    fn get(&mut self) -> i64;
}

pub trait Output {
    fn put(&mut self, value: i64);
}

// Basic output types, read/write from/to an i64.
impl Input for i64 {
    fn get(&mut self) -> i64 {
        *self
    }
}

impl Output for i64 {
    fn put(&mut self, val: i64) {
        *self = val;
    }
}

// Also allow sinking from a vector for multiple input values
impl Output for Vec<i64> {
    fn put(&mut self, val: i64) {
        self.push(val);
    }
}
