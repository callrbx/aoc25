use crate::util;

const DAY: u32 = 0;

fn parse(input: Option<&str>) -> Vec<i32> {
    let puz_input = match input {
        Some(i) => i,
        None => &util::get_input(DAY),
    };

    // day specific parsing follows
    puz_input
        .chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .collect()
}

fn part1(input: &[i32]) -> usize {
    input.iter().sum::<i32>() as usize
}

fn part2(input: &[i32]) -> usize {
    let mut pos = 0;

    for (i, &x) in input.iter().enumerate() {
        pos += x;
        if pos < 0 {
            return i + 1;
        }
    }

    pos as usize
}

pub fn solve() -> (String, String) {
    let input = parse(None);

    (part1(&input).to_string(), part2(&input).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::get_input;
    use test::Bencher;

    #[test]
    fn part1_test() {
        let input: Vec<(&str, usize)> = [("()()", 0), ("(())", 0)].to_vec();

        for (input, answer) in input {
            assert_eq!(part1(&parse(Some(input))), answer);
        }
    }

    #[bench]
    fn part1_bench(b: &mut Bencher) {
        let input: Vec<i32> = parse(Some(&get_input(DAY)));

        assert_eq!(part1(&input), 232);

        b.iter(|| part1(&input));
    }

    #[test]
    fn part2_test() {
        let input: Vec<(&str, usize)> = [(")", 1), ("()())", 5)].to_vec();

        for (input, answer) in input {
            assert_eq!(part2(&parse(Some(input))), answer);
        }
    }

    #[bench]
    fn part2_bench(b: &mut Bencher) {
        let input: Vec<i32> = parse(Some(&get_input(DAY)));

        assert_eq!(part2(&input), 1783);

        b.iter(|| part2(&input));
    }
}
