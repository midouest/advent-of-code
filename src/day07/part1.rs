use std::collections::{HashMap, HashSet};

use crate::core::Solver;

use cursive::Printer;

use super::rule::Rule;

pub struct SolvePart1 {
    color: String,
    inverted: HashMap<String, Rule>,
    visited: HashSet<String>,
    frontier: HashSet<String>,
    containers: HashSet<String>,
}

impl SolvePart1 {
    pub fn new(color: String, rules: Vec<Rule>) -> Self {
        let inverted = Rule::to_map(&Rule::invert(&rules));
        let mut frontier = HashSet::new();
        frontier.insert(color.clone());

        Self {
            color,
            inverted,
            visited: HashSet::new(),
            frontier,
            containers: HashSet::new(),
        }
    }

    fn pop_frontier(&mut self) -> Option<String> {
        let next = self.frontier.iter().next().cloned()?;
        self.frontier.remove(&next);
        Some(next)
    }
}

impl Solver<i64> for SolvePart1 {
    fn is_done(&self) -> bool {
        self.frontier.is_empty()
    }

    fn solution(&self) -> Option<i64> {
        Some(self.containers.len() as i64)
    }

    fn step(&mut self) {
        let next = self.pop_frontier().unwrap();

        if next != self.color {
            self.containers.insert(next.clone());
        }
        self.visited.insert(next.clone());

        let rule = self.inverted.get(&next).unwrap();

        for (color, _) in rule.get_relations() {
            if !self.visited.contains(color) {
                self.frontier.insert(color.clone());
            }
        }
    }

    fn draw(&self, _printer: &Printer) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::Solver;
    use crate::day07::rule::Rule;

    use super::SolvePart1;

    const DESC: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn it_solves_the_example() {
        let rules = Rule::parse_desc(DESC).unwrap();

        let mut solver = SolvePart1::new("shiny gold".to_string(), rules);
        assert_eq!(solver.solve(), Some(4));
    }
}
