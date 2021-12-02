fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .windows(3)
        .map(|window| window[0] + window[1] + window[2])
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count()
}

fn main() {
    let input = include_str!("input.txt");

    print!("{}", part1(input));
    print!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("test_input.txt");

        assert_eq!(part1(input), 7);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");

        assert_eq!(part2(input), 5);
    }
}
