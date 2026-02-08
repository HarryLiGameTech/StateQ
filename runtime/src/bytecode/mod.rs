use crate::bytecode::instruction::Instruction;

pub mod instruction;

#[cfg(test)]
mod test;

pub struct ByteCode(Vec<u8>);

impl ByteCode {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }
}

impl From<Vec<Instruction>> for ByteCode {
    fn from(instructions: Vec<Instruction>) -> Self {
        ByteCode(
            instructions.into_iter().flat_map(Vec::<u8>::from).collect::<Vec<u8>>()
        )
    }
}
