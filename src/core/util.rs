use std::ops::Range;

pub fn last_n(n: usize, max: usize) -> Range<usize> {
    if n <= max {
        0..n
    } else {
        n - max..n
    }
}

pub fn split_groups(contents: &str) -> Vec<Vec<&str>> {
    let mut groups = Vec::new();
    let mut buffer = Vec::new();

    for line in contents.lines() {
        if line.is_empty() {
            groups.push(buffer);
            buffer = Vec::new();
        } else {
            buffer.push(line);
        }
    }
    groups.push(buffer);
    groups
}

#[cfg(test)]
mod tests {
    use super::split_groups;

    #[test]
    fn it_splits_groups() {
        let contents = "foo
bar
baz

qux
quux
quuz

corge
grault
garply

waldo
fred
plugh
xyzzy

thud";

        let groups = split_groups(contents);
        assert_eq!(
            groups,
            vec![
                vec!["foo", "bar", "baz"],
                vec!["qux", "quux", "quuz"],
                vec!["corge", "grault", "garply"],
                vec!["waldo", "fred", "plugh", "xyzzy"],
                vec!["thud"]
            ]
        );
    }
}
