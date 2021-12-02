enum Direction {
    Forward(u32),
    Up(u32),
    Down(u32),
}
impl std::str::FromStr for Direction {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd, num) = s.split_once(' ').unwrap();
        let num = num.parse::<u32>().unwrap();
        match cmd {
            "forward" => Ok(Direction::Forward(num)),
            "up" => Ok(Direction::Up(num)),
            "down" => Ok(Direction::Down(num)),
            _ => panic!("unsupported input {}", cmd),
        }
    }
}
fn part1(input: &str) -> u32 {
    let mut hz_pos = 0;
    let mut depth = 0;
    for line in input.lines() {
        let direction = line.parse().unwrap();
        match direction {
            Direction::Forward(num) => hz_pos += num,
            Direction::Up(num) => depth -= num,
            Direction::Down(num) => depth += num,
        };
    }
    hz_pos * depth
}
fn part2(input: &str) -> u32 {
    let mut hz_pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in input.lines() {
        let direction = line.parse().unwrap();
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
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("test_input.txt");
        assert_eq!(part1(input), 150);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 900);
    }
}
