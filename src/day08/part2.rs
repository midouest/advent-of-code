use std::collections::HashSet;

use crate::core::Solver;

use super::{cpu::Cpu, op::Operation, part1::SolvePart1};

use cursive::Printer;

pub struct SolvePart2 {
    mem: Vec<Operation>,
    cpu: Option<Cpu>,
    candidates: Vec<usize>,
    i: usize,
    terminated: bool,
}

impl SolvePart2 {
    pub fn new(mem: Vec<Operation>) -> Self {
        // We can only change the outcome by modifying instructions that would
        // be executed normally.
        let mut part1 = SolvePart1::new(mem.clone());
        part1.solve();
        let candidates = part1
            .instructions()
            .iter()
            .filter(|&&i| match mem[i] {
                // Acc operations should not be modified
                Operation::Acc(_) => false,
                _ => true,
            })
            .cloned()
            .collect();

        Self {
            mem,
            cpu: None,
            candidates,
            i: 0,
            terminated: false,
        }
    }

    fn flip(&self) -> Option<Vec<Operation>> {
        let pc = self.candidates[self.i];
        let mut mem = self.mem.clone();

        let op = match mem[pc] {
            Operation::Nop(x) => Operation::Jmp(x),
            Operation::Jmp(x) => Operation::Nop(x),
            _ => return None,
        };

        mem[pc] = op;

        Some(mem)
    }
}

impl Solver for SolvePart2 {
    fn is_done(&self) -> bool {
        self.terminated
    }

    fn solution(&self) -> Option<i64> {
        self.cpu.as_ref().map(|c| c.acc())
    }

    fn step(&mut self) {
        let mem = self.flip().unwrap();
        let mut cpu = Cpu::new(mem);

        let mut instructions = HashSet::new();
        loop {
            instructions.insert(cpu.pc());
            cpu.step();

            if cpu.pc() == self.mem.len() {
                self.terminated = true;
                break;
            }

            if instructions.contains(&cpu.pc()) {
                break;
            }
        }

        self.cpu = Some(cpu);
        self.i += 1;
    }

    fn draw(&self, _printer: &Printer) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::Solver;
    use crate::day08::op::Operation;

    use super::SolvePart2;

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
        let mut solver = SolvePart2::new(mem);
        let solution = solver.solve();
        assert_eq!(solution, Some(8));
    }
}
