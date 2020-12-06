#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> Option<u32> {
    let lines = input.lines().into_iter();

    for p1 in lines.clone() {
        for p2 in lines.clone() {
            let pair: (u32, u32) = (p1.parse().unwrap(), p2.parse().unwrap());

            if 2020 == pair.0 + pair.1 {
                return Some(pair.0 * pair.1);
            }
        }
    }

    return None
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> Option<u32> {
    let lines = input.lines().into_iter();

    for p1 in lines.clone() {
        for p2 in lines.clone() {
            for p3 in lines.clone() {
                let pair: (u32, u32, u32) = (p1.parse().unwrap(), p2.parse().unwrap(), p3.parse().unwrap());

                if 2020 == pair.0 + pair.1 + pair.2 {
                    return Some(pair.0 * pair.1 * pair.2);
                }
            }
        }
    }

    return None
}