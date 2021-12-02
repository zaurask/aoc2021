enum Direction {
    Forward(u32),
    Up(u32),
    Down(u32),
}

fn parse_input(input: &str) -> Vec<Direction> {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(dir, num)| match dir {
            "forward" => Direction::Forward(num.parse::<u32>().unwrap()),
            "up" => Direction::Up(num.parse::<u32>().unwrap()),
            "down" => Direction::Down(num.parse::<u32>().unwrap()),
            _ => panic!("unsupported input {}", dir),
        })
        .collect()
}
fn part1(directions: &[Direction]) -> u32 {
    let mut hz_pos = 0;
    let mut depth = 0;
    for direction in directions {
        match direction {
            Direction::Forward(num) => hz_pos += num,
            Direction::Up(num) => depth -= num,
            Direction::Down(num) => depth += num,
        };
    }
    hz_pos * depth
}
fn part2(directions: &[Direction]) -> u32 {
    let mut hz_pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for direction in directions {
        match direction {
            Direction::Forward(num) => {
                hz_pos += num;
                depth += aim * num;
            }
            Direction::Up(num) => aim -= num,
            Direction::Down(num) => aim += num,
        };
    }
    hz_pos * depth
}

fn main() {
    let input = include_str!("input.txt");
    let directions = parse_input(input);
    println!("{}", part1(&directions));
    println!("{}", part2(&directions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("test_input.txt");
        let directions = parse_input(input);
        assert_eq!(part1(&directions), 150);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        let directions = parse_input(input);
        assert_eq!(part2(&directions), 900);
    }
}
