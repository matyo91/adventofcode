pub struct Password {
    min: u32,
    max: u32,
    letter: char,
    password: String
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Password> {
    input
        .lines()
        .map(|l| {
            let mut min: u32 = 0;
            let mut max: u32 = 0;
            let mut ch: char;
            let mut chars = l.chars();
            while {ch = chars.next().unwrap(); ch != '-'} {
                min = min*10 + ch.to_digit(10).unwrap();
            }
    
            while {ch = chars.next().unwrap(); ch != ' '} {
                max = max*10 + ch.to_digit(10).unwrap();
            }
            let letter: char = chars.next().unwrap();
            chars.next(); chars.next();  // Drop the following 2 characters

            //let mut password = l.trim().split('x').map(|d| d.parse().unwrap());
            Password {
                min,
                max,
                letter,
                password: String::from(chars.as_str())
            }
        }).collect()
}

#[aoc(day2, part1)]
pub fn solve_day2_part1(input: &Vec<Password>) -> usize {
    input
        .iter()
        .filter(|p| {
            let count = p
                .password
                .chars()
                .fold(0, |acc, c| if c == p.letter { acc + 1 } else { acc });
            count >= p.min && count <= p.max
        })
        .count()
}

#[aoc(day2, part2)]
pub fn solve_day2_part2(input: &Vec<Password>) -> usize {
    input
        .iter()
        .filter(|p| {
            let firstmatch = p.password.chars().nth(p.min as usize - 1).unwrap_or(' ') == p.letter;
            let secondmatch = p.password.chars().nth(p.max as usize - 1).unwrap_or(' ') == p.letter;
            firstmatch ^ secondmatch
        })
        .count()
}