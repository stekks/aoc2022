use std::cmp;

pub fn part_a(input: &str) -> i64 {
    let mut max = 0;
    let mut acc = 0;

    for line in input.trim().split('\n') {
        if line.is_empty() {
            max = cmp::max(max, acc); 
            acc = 0;
        } else {
            acc += line.parse::<i64>().unwrap();
        }
    }
    max
}

pub fn part_b(input: &str) -> i64 {
    let (mut acc, mut a, mut b, mut c) = (0, 0, 0, 0);

    for line in input.trim().split('\n') {
        if line.is_empty() {
            if acc >= a { c = b; b = a; a = acc}
            else if acc >= b { c = b; b = acc }
            else if acc >= c { c = acc }
            acc = 0
        } else {
            acc += line.parse::<i64>().unwrap();
        }
    }
    a+b+c
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 66616);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 199172);
    }
}
