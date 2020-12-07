use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Rule {
    color: String,
    relations: HashMap<String, usize>,
}

impl Rule {
    pub fn new(color: String, relations: HashMap<String, usize>) -> Self {
        Self { color, relations }
    }

    pub fn empty(color: String) -> Self {
        Self::new(color, HashMap::new())
    }

    pub fn add_relation(&mut self, color: String, count: usize) {
        self.relations.insert(color, count);
    }

    pub fn get_color(&self) -> &str {
        &self.color
    }

    pub fn is_empty(&self) -> bool {
        self.relations.is_empty()
    }

    pub fn has_relation(&self, color: &str) -> bool {
        self.relations.contains_key(color)
    }

    pub fn get_relations(&self) -> Vec<(&String, &usize)> {
        self.relations.iter().collect()
    }

    pub fn to_map(rules: &[Rule]) -> HashMap<String, Rule> {
        rules
            .iter()
            .cloned()
            .map(|rule| {
                let color = rule.get_color().to_string();
                (color.clone(), rule)
            })
            .collect()
    }

    // Given a list of rules for how many other bags any given bag can contain,
    // build a list of rules for which other bags can contain a number of any
    // given bag.
    pub fn invert(rules: &[Rule]) -> Vec<Rule> {
        let init: HashMap<_, _> = rules
            .iter()
            .map(|rule| {
                let color = rule.get_color().to_string();
                (color.clone(), Rule::empty(color))
            })
            .collect();

        rules
            .iter()
            .fold(init, |mut map, rule| {
                for (color, count) in rule.get_relations() {
                    map.get_mut(color)
                        .unwrap()
                        .add_relation(rule.get_color().to_string(), *count);
                }
                map
            })
            .values()
            .cloned()
            .collect()
    }

    pub fn parse_desc(desc: &str) -> Result<Vec<Rule>, RuleError> {
        desc.lines().map(|s| s.parse()).collect()
    }
}

pub fn find_direct_containers(color: &str, rules: &[Rule]) -> Vec<String> {
    rules
        .iter()
        .filter_map(|rule| {
            if rule.has_relation(color) {
                Some(rule.get_color().to_string())
            } else {
                None
            }
        })
        .collect()
}

#[derive(Debug)]
pub enum RuleError {
    InvalidRule,
    ParseBagCount,
    InvalidBagCount,
}

impl FromStr for Rule {
    type Err = RuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components: Vec<_> = s.split(" bags contain ").collect();
        if components.len() <= 1 {
            return Err(RuleError::InvalidRule);
        }

        let color = components[0].to_string();
        if &components[1] == &"no other bags." {
            return Ok(Rule::new(color, HashMap::new()));
        }

        let bag = components[1]
            .split(", ")
            .map(|s| {
                let components: Vec<_> = s.split(" ").collect();
                if components.len() < 4 {
                    return Err(RuleError::InvalidBagCount);
                }

                let count = components[0]
                    .parse::<usize>()
                    .map_err(|_| RuleError::ParseBagCount)?;
                let color = components[1].to_string() + " " + components[2];
                Ok((color, count))
            })
            .collect::<Result<_, _>>()?;

        Ok(Rule::new(color, bag))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::core::util::hash_map;

    use super::{find_direct_containers, Rule};

    const RULES: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn it_parses_rules_from_str() {
        let rules: Result<Vec<Rule>, _> = RULES.lines().map(|s| s.parse()).collect();
        assert!(rules.is_ok());
        assert_eq!(
            rules.unwrap(),
            vec![
                Rule::new(
                    "light red".to_string(),
                    hash_map(vec![
                        ("bright white".to_string(), 1),
                        ("muted yellow".to_string(), 2)
                    ])
                ),
                Rule::new(
                    "dark orange".to_string(),
                    hash_map(vec![
                        ("bright white".to_string(), 3),
                        ("muted yellow".to_string(), 4)
                    ])
                ),
                Rule::new(
                    "bright white".to_string(),
                    hash_map(vec![("shiny gold".to_string(), 1)])
                ),
                Rule::new(
                    "muted yellow".to_string(),
                    hash_map(vec![
                        ("shiny gold".to_string(), 2),
                        ("faded blue".to_string(), 9)
                    ])
                ),
                Rule::new(
                    "shiny gold".to_string(),
                    hash_map(vec![
                        ("dark olive".to_string(), 1),
                        ("vibrant plum".to_string(), 2)
                    ])
                ),
                Rule::new(
                    "dark olive".to_string(),
                    hash_map(vec![
                        ("faded blue".to_string(), 3),
                        ("dotted black".to_string(), 4)
                    ])
                ),
                Rule::new(
                    "vibrant plum".to_string(),
                    hash_map(vec![
                        ("faded blue".to_string(), 5),
                        ("dotted black".to_string(), 6)
                    ])
                ),
                Rule::new("faded blue".to_string(), HashMap::new()),
                Rule::new("dotted black".to_string(), HashMap::new()),
            ]
        );
    }

    #[test]
    fn it_finds_direct_containers() {
        let rules = RULES
            .lines()
            .map(|s| s.parse())
            .collect::<Result<Vec<Rule>, _>>()
            .unwrap();

        let containers = find_direct_containers("shiny gold", &rules);

        assert_eq!(
            containers,
            vec!["bright white".to_string(), "muted yellow".to_string()]
        );
    }

    #[test]
    fn it_inverts_rules() {
        let rules = vec![
            Rule::new(
                "shiny gold".to_string(),
                hash_map(vec![
                    ("dark olive".to_string(), 1),
                    ("vibrant plum".to_string(), 2),
                ]),
            ),
            Rule::empty("dark olive".to_string()),
            Rule::empty("vibrant plum".to_string()),
        ];

        let mut inverted = Rule::invert(&rules);
        inverted.sort_by_key(|r| r.get_color().to_string());

        assert_eq!(
            inverted,
            vec![
                Rule::new(
                    "dark olive".to_string(),
                    hash_map(vec![("shiny gold".to_string(), 1)])
                ),
                Rule::empty("shiny gold".to_string()),
                Rule::new(
                    "vibrant plum".to_string(),
                    hash_map(vec![("shiny gold".to_string(), 2)])
                ),
            ]
        );
    }
}
