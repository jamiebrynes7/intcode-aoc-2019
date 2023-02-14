use std::{
    convert::{TryFrom, TryInto},
    error::Error,
    str::FromStr,
};

use io::{Input, Output};
use memory::Memory;
use opcode::{OpCode, ParameterMode};

pub mod io; // Expose this module to allow users to write custom Input/Output implementations.
mod memory;
mod opcode;

pub struct Machine {
    memory: Memory,
    ip: usize,
    relative_offset: i64,
}

impl Machine {
    pub fn new(program: &str) -> Result<Self, Box<dyn Error>> {
        let memory = program
            .split(',')
            .map(|s| i64::from_str(s))
            .collect::<Result<Vec<i64>, _>>()?;

        Ok(Machine {
            memory: Memory::new(memory),
            ip: 0,
            relative_offset: 0,
        })
    }

    pub fn run<I: Input, O: Output>(
        mut self,
        input: &mut I,
        output: &mut O,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            if self.step(input, output)? {
                return Ok(());
            }
        }
    }

    fn step<I: Input, O: Output>(
        &mut self,
        input: &mut I,
        output: &mut O,
    ) -> Result<bool, Box<dyn Error>> {
        let op = OpCode::try_from(self.memory.read(self.ip))?;

        match op {
            OpCode::Add => {
                let result = self.read_param(1)? + self.read_param(2)?;
                let addr = self.load_param_addr(3)?;

                self.memory.write(addr, result);
                self.ip += 4;
            }
            OpCode::Mul => {
                let result = self.read_param(1)? * self.read_param(2)?;
                let addr = self.load_param_addr(3)?;
                self.memory.write(addr, result);
                self.ip += 4;
            }
            OpCode::Input => {
                let result = input.get();
                let addr = self.load_param_addr(1)?;
                self.memory.write(addr, result);
                self.ip += 2;
            }
            OpCode::Output => {
                let result = self.read_param(1)?;
                output.put(result);
                self.ip += 2;
            }
            OpCode::Jnz => {
                let compare_target = self.read_param(1)?;
                let jump_addr = if compare_target != 0 {
                    self.read_param(2)?.try_into()?
                } else {
                    self.ip + 3
                };

                self.ip = jump_addr;
            }
            OpCode::Jz => {
                let compare_target = self.read_param(1)?;
                let jump_addr = if compare_target == 0 {
                    self.read_param(2)?.try_into()?
                } else {
                    self.ip + 3
                };

                self.ip = jump_addr;
            }
            OpCode::Lt => {
                let result = if self.read_param(1)? < self.read_param(2)? {
                    1
                } else {
                    0
                };

                let addr = self.load_param_addr(3)?.try_into()?;
                self.memory.write(addr, result);
                self.ip += 4;
            }
            OpCode::Eq => {
                let result = if self.read_param(1)? == self.read_param(2)? {
                    1
                } else {
                    0
                };

                let addr = self.load_param_addr(3)?;
                self.memory.write(addr, result);
                self.ip += 4;
            }
            OpCode::Rel => {
                self.relative_offset += self.read_param(1)?;
                self.ip += 2;
            }
            OpCode::Halt => {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn read_param(&self, offset: usize) -> Result<i64, Box<dyn Error>> {
        let addr = self.load_param_addr(offset)?;
        Ok(self.memory.read(addr))
    }

    fn load_param_addr(&self, offset: usize) -> Result<usize, Box<dyn Error>> {
        let value = self.memory.read(self.ip);
        let digit = value / (10i64).pow(offset as u32 + 1) % 10;
        let mode = ParameterMode::try_from(digit)?;

        let addr = match mode {
            ParameterMode::Position => self.memory.read(self.ip + offset),
            ParameterMode::Immediate => (self.ip + offset) as i64,
            ParameterMode::Relative => self.memory.read(self.ip + offset) + self.relative_offset,
        };

        Ok(addr.try_into()?)
    }
}
