fn part1(input: &str) -> usize {
    todo!()
}

fn part2(input: &str) -> usize {
    todo!()
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
    //println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("test_input.txt");
        assert_eq!(part1(input), 37);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 26984457539);
    }
}
