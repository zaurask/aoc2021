fn part1(input: &str) -> usize {
    let mut lanternfish: Vec<usize> = vec![0; 9];

    for item in input.split(',') {
        lanternfish[item.parse::<usize>().unwrap()] += 1;
    }

    for _ in 1..=80 {
        for (idx, num) in lanternfish.clone().into_iter().enumerate() {
            if idx == 0 {
                lanternfish[8] += num;
                lanternfish[6] += num;
                lanternfish[0] -= num;
                continue;
            }
            lanternfish[idx] -= num;
            lanternfish[idx - 1] += num;
        }
    }
    lanternfish.iter().sum()
}

fn part2(input: &str) -> usize {
    let mut lanternfish: Vec<usize> = vec![0; 9];

    for item in input.split(',') {
        lanternfish[item.parse::<usize>().unwrap()] += 1;
    }

    for _ in 1..=256 {
        for (idx, num) in lanternfish.clone().into_iter().enumerate() {
            if idx == 0 {
                lanternfish[8] += num;
                lanternfish[6] += num;
                lanternfish[0] -= num;
                continue;
            }
            lanternfish[idx] -= num;
            lanternfish[idx - 1] += num;
        }
    }
    lanternfish.iter().sum()
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
