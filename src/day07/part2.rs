use std::collections::HashMap;

use cursive::Printer;

use crate::core::Solver;

use super::rule::Rule;

pub struct SolvePart2 {
    color: String,
    rules: HashMap<String, Rule>,
    count: usize,
    frontier: HashMap<String, usize>,
}

impl SolvePart2 {
    pub fn new(color: String, rules: Vec<Rule>) -> Self {
        let rules = Rule::to_map(&rules);
        let mut frontier = HashMap::new();
        frontier.insert(color.clone(), 1);
        Self {
            color,
            rules,
            count: 0,
            frontier,
        }
    }

    fn pop_frontier(&mut self) -> Option<(String, usize)> {
        let (color, &count) = self.frontier.iter().next()?;
        let color = color.clone();
        self.frontier.remove(&color);
        Some((color.clone(), count))
    }
}

impl Solver for SolvePart2 {
    fn is_done(&self) -> bool {
        self.frontier.is_empty()
    }

    fn solution(&self) -> Option<i64> {
        Some(self.count as i64)
    }

    fn step(&mut self) {
        let (color, count) = self.pop_frontier().unwrap();

        if color != self.color {
            self.count += count;
        }

        let rule = self.rules.get(&color).unwrap();
        if rule.is_empty() {
            return;
        }

        for (next_color, base_count) in rule.get_relations() {
            let next_count = count * base_count;
            if !self.frontier.contains_key(next_color) {
                self.frontier.insert(next_color.clone(), next_count);
            } else {
                let prev_count = *self.frontier.get(next_color).unwrap();
                self.frontier
                    .insert(next_color.clone(), prev_count + next_count);
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

    use super::SolvePart2;

    const EXAMPLE1: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn it_solves_the_first_example() {
        let rules = Rule::parse_desc(EXAMPLE1).unwrap();

        let mut solver = SolvePart2::new("shiny gold".to_string(), rules);
        assert_eq!(solver.solve(), Some(32));
    }

    const EXAMPLE2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn it_solves_the_second_example() {
        let rules = Rule::parse_desc(EXAMPLE2).unwrap();

        let mut solver = SolvePart2::new("shiny gold".to_string(), rules);
        assert_eq!(solver.solve(), Some(126));
    }
}
