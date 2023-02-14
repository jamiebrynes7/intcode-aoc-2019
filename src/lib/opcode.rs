use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpCode {
    Add,
    Mul,
    Input,
    Output,
    Jnz, // Jump if non-zero
    Jz,  // Jump if zero
    Lt,  // Less than
    Eq,  // Equal
    Rel, // Relative base
    Halt,
}

impl TryFrom<i64> for OpCode {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value % 100 {
            1 => Ok(OpCode::Add),
            2 => Ok(OpCode::Mul),
            3 => Ok(OpCode::Input),
            4 => Ok(OpCode::Output),
            5 => Ok(OpCode::Jnz),
            6 => Ok(OpCode::Jz),
            7 => Ok(OpCode::Lt),
            8 => Ok(OpCode::Eq),
            9 => Ok(OpCode::Rel),
            99 => Ok(OpCode::Halt),
            _ => Err(format!("Unknown opcode encountered: {}", value % 100)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParameterMode {
    // This is 'pointer' mode. The address of the parameter is the value at the address of (instruction pointer + offset).
    Position,

    // The address of the parameter is the value of the (instruction pointer + offset)
    Immediate,

    // Extension of 'Position' mode, we also add the relative base to the address.
    Relative,
}

impl TryFrom<i64> for ParameterMode {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ParameterMode::Position),
            1 => Ok(ParameterMode::Immediate),
            2 => Ok(ParameterMode::Relative),
            _ => Err(format!("Unknown parameter mode encountered: {}", value)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_opcode_parsed_correctly() {
        let value = 1005;
        let result = OpCode::try_from(value);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), OpCode::Jnz);
    }

    #[test]
    fn invalid_opcode_rejected() {
        let value = 1010;
        let result = OpCode::try_from(value);
        assert!(result.is_err());
    }

    #[test]
    fn valid_parameter_mode_parsed_correctly() {
        let value = 0;
        let result = ParameterMode::try_from(value);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ParameterMode::Position);
    }

    #[test]
    fn invalid_parameter_mode_rejected() {
        let value = 3;
        let result = ParameterMode::try_from(value);

        assert!(result.is_err());
    }
}
