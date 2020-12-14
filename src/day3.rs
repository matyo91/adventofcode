#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|l| String::from(l))
        .collect::<Vec<String>>()
}

pub fn solve_generic(input: &Vec<String>, right: usize, down: usize) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(i, l)| {
            // Filter each string based on whether we hit the "#" or not
            l.len() > 0
                && i % down == 0
                && l.chars().nth((i / down * right) % l.len()).unwrap_or(' ') == '#'
        })
        .count()
}

#[aoc(day3, part1)]
pub fn solve_day3_part1(input: &Vec<String>) -> usize {
    solve_generic(input, 3, 1)
}

#[aoc(day3, part2)]
pub fn solve_day3_part2(input: &Vec<String>) -> usize {
    solve_generic(input, 1, 1)
        * solve_generic(input, 3, 1)
        * solve_generic(input, 5, 1)
        * solve_generic(input, 7, 1)
        * solve_generic(input, 1, 2)
}