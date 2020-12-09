use std::collections::HashSet;

use crate::core::Solver;

use super::{cpu::Cpu, op::Operation};

use cursive::Printer;

pub struct SolvePart1 {
    cpu: Cpu,
    instructions: HashSet<usize>,
}

impl SolvePart1 {
    pub fn new(mem: Vec<Operation>) -> Self {
        Self {
            cpu: Cpu::new(mem),
            instructions: HashSet::new(),
        }
    }

    pub fn instructions(&self) -> &HashSet<usize> {
        &self.instructions
    }
}

impl Solver<i64> for SolvePart1 {
    fn is_done(&self) -> bool {
        self.instructions.contains(&self.cpu.pc())
    }

    fn solution(&self) -> Option<i64> {
        Some(self.cpu.acc())
    }

    fn step(&mut self) {
        self.instructions.insert(self.cpu.pc());
        self.cpu.step();
    }

    fn draw(&self, _printer: &Printer) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::Solver;
    use crate::day08::op::Operation;

    use super::SolvePart1;

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
    fn it_solves_the_example() {
        let mem = Operation::parse_mem(EXAMPLE).unwrap();
        let mut solver = SolvePart1::new(mem);
        let solution = solver.solve();
        assert_eq!(solution, Some(5));
    }
}
