// A struct to represent the memory of the intcode machine, the memory is elastic and grows as required. The default value is 0.
pub struct Memory {
    memory: Vec<i64>,
}

impl Memory {
    pub fn new(seed: Vec<i64>) -> Self {
        Memory { memory: seed }
    }

    pub fn read(&self, addr: usize) -> i64 {
        *self.memory.get(addr).unwrap_or(&0)
    }

    pub fn write(&mut self, addr: usize, value: i64) {
        if addr >= self.memory.len() {
            self.memory.resize(addr + 1, 0);
        }

        self.memory[addr] = value;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_value_is_0() {
        let memory = Memory::new(vec![]);
        assert_eq!(memory.read(100), 0);
    }

    #[test]
    fn writes_out_of_bounds_expands_memory() {
        let mut memory = Memory::new(vec![1, 2, 3]);
        memory.write(10, 10);

        // Ensure we copy existing memory
        assert_eq!(memory.read(0), 1);
        // And then validate that we've expanded the memory.
        assert_eq!(memory.read(10), 10);
    }
}
