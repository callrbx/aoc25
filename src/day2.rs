use std::time::Instant;

use crate::{
    Answer,
    util::{self, extract_day},
};

#[derive(Debug)]
struct ProdRange {
    first: u64,
    last: u64,
}

fn parse(input: Option<&str>) -> Vec<ProdRange> {
    let day = extract_day(file!());

    let puz_input = match input {
        Some(i) => i,
        None => &util::get_input(day),
    };

    // day specific parsing follows
    puz_input
        .split(',')
        .filter_map(|r| {
            let (l, r) = match r.split_once('-') {
                Some((l, r)) => (l.parse().unwrap_or(0), r.parse().unwrap_or(0)),
                None => return None,
            };

            Some(ProdRange { first: l, last: r })
        })
        .collect::<Vec<ProdRange>>()
}

fn is_repeating_number(mut n: u64) -> bool {
    if n < 10 {
        return false;
    }

    // gross string chunk parsing - probably math is better here
    let mut digits = Vec::new();
    while n > 0 {
        digits.push((n % 10) as u8);
        n /= 10;
    }
    digits.reverse();

    let d = digits.len();

    for p in 1..=d / 2 {
        if d % p != 0 {
            continue;
        }

        let mut ok = true;
        for i in 0..d {
            if digits[i] != digits[i % p] {
                ok = false;
                break;
            }
        }

        if ok {
            return true;
        }
    }

    false
}

fn part1(input: &[ProdRange]) -> u64 {
    input
        .iter()
        .flat_map(|r| r.first..=r.last)
        .filter(|&n| {
            let digits = ((n as f64).log10().floor() as u32) + 1;
            let cut = 10u64.pow(digits / 2);
            n / cut == n % cut
        })
        .sum()
}

fn part2(input: &[ProdRange]) -> u64 {
    input
        .iter()
        .flat_map(|r| r.first..=r.last)
        .filter(|&n| is_repeating_number(n))
        .sum()
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

    #[test]
    fn part1_test() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        assert_eq!(part1(&parse(Some(input))), 1227775554);
    }

    #[bench]
    fn part1_bench(b: &mut Bencher) {
        let day = extract_day(file!());
        let input: Vec<ProdRange> = parse(Some(&get_input(day)));

        assert_eq!(part1(&input), 54641809925);

        b.iter(|| part1(&input));
    }

    #[test]
    fn part2_test() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        assert_eq!(part2(&parse(Some(input))), 4174379265);
    }

    #[bench]
    fn part2_bench(b: &mut Bencher) {
        let day = extract_day(file!());
        let input: Vec<ProdRange> = parse(Some(&get_input(day)));

        assert_eq!(part2(&input), 73694270688);

        b.iter(|| part2(&input));
    }
}
