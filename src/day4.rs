use pathfinding::grid::Grid;
use std::time::Instant;

use crate::{
    Answer,
    util::{self, extract_day},
};

fn parse(input: Option<&str>) -> Grid {
    let day = extract_day(file!());

    let puz_input = match input {
        Some(i) => i,
        None => &util::get_input(day),
    };

    let mut points: Vec<(usize, usize)> = Vec::new();

    // day specific parsing follows
    for (y, line) in puz_input.split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                points.push((x, y));
            }
        }
    }

    let mut grid = Grid::from_coordinates(&points).unwrap();
    grid.enable_diagonal_mode();
    grid
}

fn remove_paper(input: &Grid) -> (usize, Grid) {
    let mut copy = input.clone();

    let n = input.iter().fold(0usize, |acc, vertex| {
        let occ_spaces = input
            .neighbours(vertex)
            .iter()
            .map(|space| input.has_vertex(*space) as usize)
            .sum::<usize>();

        if occ_spaces < 4 {
            copy.remove_vertex(vertex);
        }

        acc + (occ_spaces < 4) as usize
    });

    (n, copy)
}

fn part1(input: &Grid) -> usize {
    remove_paper(input).0
}

fn part2(input: &Grid) -> usize {
    let mut copy = input.clone();
    let mut ans = 0;

    loop {
        let (n, new_copy) = remove_paper(&copy);
        if n == 0 {
            break;
        }

        ans += n;
        copy = new_copy;
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

    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn part1_test() {
        assert_eq!(part1(&parse(Some(INPUT))), 13);
    }

    #[bench]
    fn part1_bench(b: &mut Bencher) {
        let day = extract_day(file!());
        let input = parse(Some(&get_input(day)));

        assert_eq!(part1(&input), 1537);

        b.iter(|| part1(&input));
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(&parse(Some(INPUT))), 43);
    }

    #[bench]
    fn part2_bench(b: &mut Bencher) {
        let day = extract_day(file!());
        let input = parse(Some(&get_input(day)));

        assert_eq!(part2(&input), 8707);

        b.iter(|| part2(&input));
    }
}
