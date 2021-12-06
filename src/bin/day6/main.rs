fn part1(input: &str) -> usize {
    let mut lanternfish_state = [0; 9];

    for state in input.split(',') {
        lanternfish_state[state.parse::<usize>().unwrap()] += 1;
    }

    for _ in 1..=80 {
        lanternfish_state.rotate_left(1);
        lanternfish_state[6] += lanternfish_state[8]
    }
    lanternfish_state.iter().sum()
}

fn part2(input: &str) -> usize {
    let mut lanternfish_state = [0; 9];

    for state in input.split(',') {
        lanternfish_state[state.parse::<usize>().unwrap()] += 1;
    }

    for _ in 1..=256 {
        lanternfish_state.rotate_left(1);
        lanternfish_state[6] += lanternfish_state[8]
    }
    lanternfish_state.iter().sum()
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
        assert_eq!(part1(input), 5934);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 26984457539);
    }
}
