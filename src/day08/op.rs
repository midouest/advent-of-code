use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operation {
    Nop,
    Acc(i64),
    Jmp(i64),
}

impl Operation {
    pub fn parse_mem(contents: &str) -> Result<Vec<Operation>, OperationError> {
        contents.lines().map(|line| line.parse()).collect()
    }
}

#[derive(Debug, Clone)]
pub enum OperationError {
    BadSyntax,
    BadArgument,
    BadOperation,
}

impl FromStr for Operation {
    type Err = OperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components: Vec<_> = s.split(" ").collect();
        if components.len() != 2 {
            return Err(OperationError::BadSyntax);
        }

        let arg: i64 = components[1]
            .parse()
            .map_err(|_| OperationError::BadArgument)?;

        match components[0] {
            "nop" => Ok(Operation::Nop),
            "acc" => Ok(Operation::Acc(arg)),
            "jmp" => Ok(Operation::Jmp(arg)),
            _ => Err(OperationError::BadOperation),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Operation;

    const EXAMPLE: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn it_parses_from_str() {
        let ops = Operation::parse_mem(EXAMPLE).unwrap();

        use Operation::*;

        assert_eq!(
            ops,
            vec![
                Nop,
                Acc(1),
                Jmp(4),
                Acc(3),
                Jmp(-3),
                Acc(-99),
                Acc(1),
                Jmp(-4),
                Acc(6)
            ]
        );
    }
}
