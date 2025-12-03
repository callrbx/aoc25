use std::time::Instant;

use crate::{Answer, util};

const DAY: u32 = 3;

fn parse(input: Option<&str>) -> Vec<Vec<u8>> {
    let puz_input = match input {
        Some(i) => i,
        None => &util::get_input(DAY),
    };

    // day specific parsing follows
    puz_input
        .split_ascii_whitespace()
        .map(|line| line.chars().map(|c| (c as u8) - b'0').collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>()
}

fn find_top_n(num_digits: usize, bank: &[u8]) -> usize {
    let len = bank.len();
    let mut start_idx = 0;
    let mut jolt = 0;

    for x in (0..num_digits).rev() {
        let remaining = len - x;
        let mut max = 0;
        let mut next_start = start_idx;

        for (i, v) in bank.iter().enumerate().take(remaining).skip(start_idx) {
            if *v > max {
                max = *v;
                next_start = i + 1;
            }
        }

        start_idx = next_start;
        jolt = jolt * 10 + max as usize;
    }

    jolt
}

fn part1(input: &Vec<Vec<u8>>) -> usize {
    let mut ans = 0;

    for bank in input {
        ans += find_top_n(2, bank);
    }

    ans
}

fn part2(input: &Vec<Vec<u8>>) -> usize {
    let mut ans = 0;

    for bank in input {
        ans += find_top_n(12, bank);
    }

    ans
}

pub fn solve() -> Answer {
    let input = parse(None);

    let now = Instant::now();
    let (p1, p2) = (part1(&input).to_string(), part2(&input).to_string());
    Answer {
        p1,
        p2,
        t: now.elapsed(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::get_input;
    use test::Bencher;

    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn part1_test() {
        assert_eq!(part1(&parse(Some(INPUT))), 357);
    }

    #[bench]
    fn part1_bench(b: &mut Bencher) {
        let input = parse(Some(&get_input(DAY)));

        assert_eq!(part1(&input), 16854);

        b.iter(|| part1(&input));
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(&parse(Some(INPUT))), 3121910778619);
    }

    #[bench]
    fn part2_bench(b: &mut Bencher) {
        let input = parse(Some(&get_input(DAY)));

        assert_eq!(part2(&input), 167526011932478);

        b.iter(|| part2(&input));
    }
}
