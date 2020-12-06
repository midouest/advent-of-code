use std::collections::HashSet;

pub trait CustomsPolicy {
    fn count(group: &Group) -> usize;
}

pub struct Group {
    answers: Vec<HashSet<char>>,
}

impl Group {
    pub fn new(responses: &Vec<&str>) -> Self {
        let answers = responses.iter().map(|s| s.chars().collect()).collect();
        Self { answers }
    }

    pub fn get_answers(&self) -> &Vec<HashSet<char>> {
        &self.answers
    }

    pub fn count<P: CustomsPolicy>(&self) -> usize {
        P::count(self)
    }
}

pub struct MisreadPolicy {}

impl CustomsPolicy for MisreadPolicy {
    fn count(group: &Group) -> usize {
        group
            .answers
            .iter()
            .fold(HashSet::new(), |acc, answer| {
                acc.union(answer).cloned().collect()
            })
            .len()
    }
}

pub struct CorrectPolicy {}

impl CustomsPolicy for CorrectPolicy {
    fn count(group: &Group) -> usize {
        group.answers[1..]
            .iter()
            .fold(group.answers[0].clone(), |acc, answer| {
                acc.intersection(answer).cloned().collect()
            })
            .len()
    }
}
