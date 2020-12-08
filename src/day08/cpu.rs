use super::op::Operation;

pub struct Cpu {
    mem: Vec<Operation>,
    pc: usize,
    acc: i64,
}

impl Cpu {
    pub fn new(mem: Vec<Operation>) -> Self {
        Self { mem, pc: 0, acc: 0 }
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    pub fn acc(&self) -> i64 {
        self.acc
    }

    pub fn step(&mut self) {
        if let Some(&op) = self.fetch() {
            self.exec(op);
        }
    }

    fn fetch(&self) -> Option<&Operation> {
        self.mem.get(self.pc)
    }

    fn exec(&mut self, operation: Operation) {
        use Operation::*;

        match operation {
            Nop(_) => self.inc_pc(),
            Acc(x) => {
                self.acc += x;
                self.inc_pc();
            }
            Jmp(off) => self.set_pc_rel(off),
        }
    }

    fn inc_pc(&mut self) {
        self.pc += 1;
    }

    fn set_pc_rel(&mut self, off: i64) {
        self.pc = ((self.pc as i64) + off) as usize;
    }
}

#[cfg(test)]
mod tests {
    use crate::day08::op::Operation;

    use super::Cpu;

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
    fn it_executes_the_example() {
        let mem = Operation::parse_mem(EXAMPLE).unwrap();
        let mut cpu = Cpu::new(mem);

        for _ in 0..7 {
            cpu.step();
        }

        assert_eq!(cpu.pc(), 1);
        assert_eq!(cpu.acc(), 5);
    }
}
