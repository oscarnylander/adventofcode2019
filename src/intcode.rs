use std::convert::TryFrom;

// struct Memory(Vec<i32>);
// 
// impl Memory {
//     fn load_register(&self, address_or_value: i32, mode: &OperationMode) -> Result<i32, ExecutionError> {
//         match mode {
//             OperationMode::Position => {
//                 Ok(self.0[usize::try_from(address_or_value)?])
//             }
//             OperationMode::Immediate => Ok(address_or_value),
//         }
//     }
// }

enum OperationMode {
    Position,
    Immediate
}

enum IntCode {
    Add(OperationMode, OperationMode),
    Multiply(OperationMode, OperationMode),
    StoreInput,
    LoadOutput(OperationMode),
    JumpIfTrue(OperationMode, OperationMode),
    JumpIfFalse(OperationMode, OperationMode),
    LessThan(OperationMode, OperationMode),
    Equals(OperationMode, OperationMode),
    Halt,
}

impl IntCode {
    fn instruction_width(&self) -> usize {
        match self {
            Self::Add(_, _) => 4,
            Self::Multiply(_, _) => 4,
            Self::StoreInput => 2,
            Self::LoadOutput(_) => 2,
            Self::JumpIfTrue(_, _) => 3,
            Self::JumpIfFalse(_, _) => 3,
            Self::LessThan(_, _) => 4,
            Self::Equals(_, _) => 4,
            Self::Halt => 1
        }
    }
}

impl TryFrom<i32> for IntCode {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(IntCode::Add(OperationMode::Position, OperationMode::Position)),
            101 => Ok(IntCode::Add(OperationMode::Immediate, OperationMode::Position)),
            1001 => Ok(IntCode::Add(OperationMode::Position, OperationMode::Immediate)),
            1101 => Ok(IntCode::Add(OperationMode::Immediate, OperationMode::Immediate)),

            2 => Ok(IntCode::Multiply(OperationMode::Position, OperationMode::Position)),
            102 => Ok(IntCode::Multiply(OperationMode::Immediate, OperationMode::Position)),
            1002 => Ok(IntCode::Multiply(OperationMode::Position, OperationMode::Immediate)),
            1102 => Ok(IntCode::Multiply(OperationMode::Immediate, OperationMode::Immediate)),

            3 => Ok(IntCode::StoreInput),

            4 => Ok(IntCode::LoadOutput(OperationMode::Position)),
            104 => Ok(IntCode::LoadOutput(OperationMode::Immediate)),

            5 => Ok(IntCode::JumpIfTrue(OperationMode::Position, OperationMode::Position)),
            105 => Ok(IntCode::JumpIfTrue(OperationMode::Immediate, OperationMode::Position)),
            1005 => Ok(IntCode::JumpIfTrue(OperationMode::Position, OperationMode::Immediate)),
            1105 => Ok(IntCode::JumpIfTrue(OperationMode::Immediate, OperationMode::Immediate)),

            6 => Ok(IntCode::JumpIfFalse(OperationMode::Position, OperationMode::Position)),
            106 => Ok(IntCode::JumpIfFalse(OperationMode::Immediate, OperationMode::Position)),
            1006 => Ok(IntCode::JumpIfFalse(OperationMode::Position, OperationMode::Immediate)),
            1106 => Ok(IntCode::JumpIfFalse(OperationMode::Immediate, OperationMode::Immediate)),

            7 => Ok(IntCode::LessThan(OperationMode::Position, OperationMode::Position)),
            107 => Ok(IntCode::LessThan(OperationMode::Immediate, OperationMode::Position)),
            1007 => Ok(IntCode::LessThan(OperationMode::Position, OperationMode::Immediate)),
            1107 => Ok(IntCode::LessThan(OperationMode::Immediate, OperationMode::Immediate)),

            8 => Ok(IntCode::Equals(OperationMode::Position, OperationMode::Position)),
            108 => Ok(IntCode::Equals(OperationMode::Immediate, OperationMode::Position)),
            1008 => Ok(IntCode::Equals(OperationMode::Position, OperationMode::Immediate)),
            1108 => Ok(IntCode::Equals(OperationMode::Immediate, OperationMode::Immediate)),

            99 => Ok(IntCode::Halt),
            _ => Err(format!("Invalid IntCode {}", value)),
        }
    }
}

#[derive(Debug)]
pub enum ExecutionError {
    Conversion,
    OutOfBounds,
    MalfunctioningInstruction
}

impl From<std::num::TryFromIntError> for ExecutionError {
    fn from(_err: std::num::TryFromIntError) -> ExecutionError {
        ExecutionError::Conversion
    }
}

fn load_register(input: &[i32], address_or_value: i32, mode: &OperationMode) -> Result<i32, ExecutionError> {
    match mode {
        OperationMode::Position => {
            Ok(input[usize::try_from(address_or_value)?])
        }
        OperationMode::Immediate => Ok(address_or_value),
    }
}

pub fn execute(memory: &mut Vec<i32>, input: i32) -> Result<i32, ExecutionError> {
    let mut idx = 0;

    let mut output = 0;

    loop {
        let current_instruction = IntCode::try_from(memory[idx]).unwrap();

        match &current_instruction {
            IntCode::Add(a_mode, b_mode) => {
                if output != 0 {
                    return Err(ExecutionError::MalfunctioningInstruction);
                }

                let a = load_register(&memory, *memory.get(idx + 1).ok_or(ExecutionError::OutOfBounds)?, a_mode)?;
                let b = load_register(&memory, *memory.get(idx + 2).ok_or(ExecutionError::OutOfBounds)?, b_mode)?;
                let c_loc = usize::try_from(*memory.get(idx + 3).ok_or(ExecutionError::OutOfBounds)?)?;

                memory[c_loc] = a + b;
                idx += current_instruction.instruction_width();
            }

            IntCode::Multiply(a_mode, b_mode) => {
                if output != 0 {
                    return Err(ExecutionError::MalfunctioningInstruction);
                }

                let a = load_register(&memory, *memory.get(idx + 1).ok_or(ExecutionError::OutOfBounds)?, a_mode)?;
                let b = load_register(&memory, *memory.get(idx + 2).ok_or(ExecutionError::OutOfBounds)?, b_mode)?;
                let c_loc = usize::try_from(*memory.get(idx + 3).ok_or(ExecutionError::OutOfBounds)?)?;

                memory[c_loc] = a * b;
                idx += current_instruction.instruction_width();
            }

            IntCode::StoreInput => {
                if output != 0 {
                    return Err(ExecutionError::MalfunctioningInstruction);
                }

                let a_loc = usize::try_from(*memory.get(idx + 1).ok_or(ExecutionError::OutOfBounds)?)?;
                memory[a_loc] = input;
                idx += current_instruction.instruction_width();
            }

            IntCode::LoadOutput(a_mode) => {
                if output != 0 {
                    return Err(ExecutionError::MalfunctioningInstruction);
                }

                output = load_register(&memory, *memory.get(idx + 1).ok_or(ExecutionError::OutOfBounds)?, a_mode)?;
                idx += current_instruction.instruction_width();
            }

            IntCode::JumpIfTrue(a_mode, b_mode) => {
                if output != 0 {
                    return Err(ExecutionError::MalfunctioningInstruction);
                }

                let a = load_register(&memory, *memory.get(idx + 1).ok_or(ExecutionError::OutOfBounds)?, a_mode)?;
                if a != 0 {
                    let b = usize::try_from(load_register(&memory, *memory.get(idx + 2).ok_or(ExecutionError::OutOfBounds)?, b_mode)?)?;
                    idx = b;
                } else {
                    idx += current_instruction.instruction_width();
                }
            }
            
            IntCode::JumpIfFalse(a_mode, b_mode) => {
                if output != 0 {
                    return Err(ExecutionError::MalfunctioningInstruction);
                }

                let a = load_register(&memory, *memory.get(idx + 1).ok_or(ExecutionError::OutOfBounds)?, a_mode)?;
                if a == 0 {
                    let b = usize::try_from(load_register(&memory, *memory.get(idx + 2).ok_or(ExecutionError::OutOfBounds)?, b_mode)?)?;
                    idx = b;
                } else {
                    idx += current_instruction.instruction_width();
                }
            }

            IntCode::LessThan(a_mode, b_mode) => {
                if output != 0 {
                    return Err(ExecutionError::MalfunctioningInstruction);
                }

                let a = load_register(&memory, *memory.get(idx + 1).ok_or(ExecutionError::OutOfBounds)?, a_mode)?;
                let b = load_register(&memory, *memory.get(idx + 2).ok_or(ExecutionError::OutOfBounds)?, b_mode)?;
                let c_loc = usize::try_from(*memory.get(idx + 3).ok_or(ExecutionError::OutOfBounds)?)?;

                memory[c_loc] = if a < b {
                    1
                }  else {
                    0
                };

                idx += current_instruction.instruction_width();
            }

            IntCode::Equals(a_mode, b_mode) => {
                if output != 0 {
                    return Err(ExecutionError::MalfunctioningInstruction);
                }

                let a = load_register(&memory, *memory.get(idx + 1).ok_or(ExecutionError::OutOfBounds)?, a_mode)?;
                let b = load_register(&memory, *memory.get(idx + 2).ok_or(ExecutionError::OutOfBounds)?, b_mode)?;
                let c_loc = usize::try_from(*memory.get(idx + 3).ok_or(ExecutionError::OutOfBounds)?)?;

                memory[c_loc] = if a == b {
                    1
                }  else {
                    0
                };

                idx += current_instruction.instruction_width();
            }

            IntCode::Halt => return Ok(output),
        }
    }
}
