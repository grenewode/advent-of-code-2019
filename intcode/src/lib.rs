use std::convert::TryFrom;
mod error;

pub use error::Error;

pub mod opcodes {
    pub const ADD: i32 = 1;
    pub const MUL: i32 = 2;
    pub const HALT: i32 = 99;
}

pub struct VM {
    memory: Vec<i32>,
    program_counter: i32,
    print_info: bool,
}

impl VM {
    pub fn new(memory: &[i32], print_info: bool) -> Self {
        Self {
            memory: memory.to_vec(),
            program_counter: 0,
            print_info,
        }
    }

    pub fn new_debug(memory: &[i32]) -> Self {
        Self::new(memory, true)
    }

    pub fn new_silent(memory: &[i32]) -> Self {
        Self::new(memory, false)
    }

    pub fn load(&self, address: i32) -> Result<i32, Error> {
        let idx = usize::try_from(address).map_err(|_| Error::InvalidLoad(address))?;
        self.memory
            .get(idx)
            .copied()
            .ok_or_else(|| Error::InvalidLoad(address))
    }

    pub fn store(&mut self, address: i32, value: i32) -> Result<i32, Error> {
        let idx = usize::try_from(address).map_err(|_| Error::InvalidStore(address))?;
        let dest = self
            .memory
            .get_mut(idx)
            .ok_or_else(|| Error::InvalidLoad(address))?;

        Ok(std::mem::replace(dest, value))
    }

    pub fn read(&mut self) -> Result<i32, Error> {
        let value = self.load(self.program_counter)?;
        self.program_counter += 1;
        Ok(value)
    }

    pub fn load_at_read(&mut self) -> Result<(i32, i32), Error> {
        let addr = self.read()?;
        self.load(addr).map(|value| (addr, value))
    }

    pub fn store_to_read(&mut self, value: i32) -> Result<(i32, i32), Error> {
        let addr = self.read()?;
        self.store(addr, value).map(|old_value| (addr, old_value))
    }

    fn op2(&mut self, name: &str, op: impl FnOnce(i32, i32) -> i32) -> Result<(), Error> {
        let (a_addr, a) = self.load_at_read()?;
        let (b_addr, b) = self.load_at_read()?;
        let result = op(a, b);
        let (dest_addr, _) = self.store_to_read(result)?;
        if self.print_info {
            println!(
                "{} @{}:{} @{}:{} -> @{}:{}",
                name, a_addr, a, b_addr, b, dest_addr, result
            );
        }
        Ok(())
    }

    pub fn step(&mut self) -> Result<bool, Error> {
        let opcode = self.read()?;
        match opcode {
            opcodes::ADD => {
                self.op2("add", |a, b| a + b)?;
                Ok(true)
            }
            opcodes::MUL => {
                self.op2("mul", |a, b| a * b)?;
                Ok(true)
            }
            opcodes::HALT => Ok(false),
            _ => Err(Error::InvalidOpcode(opcode)),
        }
    }

    pub fn execute(&mut self) -> Result<i32, Error> {
        while self.step()? {}

        self.load(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_program() {
        let mut vm = VM::new_silent(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);

        assert_eq!(vm.execute().unwrap(), 3500);
    }
}
