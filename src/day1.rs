use std::time::Instant;

use crate::{Answer, util};
use num_integer::Integer;

const DAY: u32 = 1;

fn parse(input: Option<&str>) -> Vec<i32> {
    let puz_input = match input {
        Some(i) => i,
        None => &util::get_input(DAY),
    };

    // day specific parsing follows
    puz_input
        .split_ascii_whitespace()
        .filter_map(|line| {
            let (dir, num) = line.split_at(1);
            let value: i32 = match num.parse() {
                Ok(n) => n,
                Err(_) => return None,
            };

            match dir {
                "L" => Some(-value),
                "R" => Some(value),
                _ => None,
            }
        })
        .collect::<Vec<i32>>()
}

fn part1(input: &[i32]) -> usize {
    let mut pos = 50;
    let mut ans = 0;

    for x in input {
        pos += x;
        pos = pos.mod_floor(&100);
        if pos == 0 {
            ans += 1
        }
    }

    ans
}

fn part2(input: &[i32]) -> usize {
    let mut pos = 50;
    let mut ans = 0;

    for &x in input {
        let old = pos;
        let steps = x.unsigned_abs() as usize;

        // distance to zero in clicks
        let dtz: usize = if x >= 0 {
            if old == 0 { 100 } else { (100 - old) as usize }
        } else if old == 0 {
            100
        } else {
            old as usize
        };

        if steps >= dtz {
            // increment for d to z and every 100 after
            ans += 1 + (steps - dtz) / 100;
        }

        // apply rotation
        pos += x;
        pos = pos.mod_floor(&100);
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

    #[test]
    fn part1_test() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(part1(&parse(Some(input))), 3);
    }

    #[bench]
    fn part1_bench(b: &mut Bencher) {
        let input: Vec<i32> = parse(Some(&get_input(DAY)));

        assert_eq!(part1(&input), 1118);

        b.iter(|| part1(&input));
    }

    #[test]
    fn part2_test() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(part2(&parse(Some(input))), 6);
    }

    #[bench]
    fn part2_bench(b: &mut Bencher) {
        let input: Vec<i32> = parse(Some(&get_input(DAY)));

        assert_eq!(part2(&input), 6289);

        b.iter(|| part2(&input));
    }
}
