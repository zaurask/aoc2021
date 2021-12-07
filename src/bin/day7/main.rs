fn part1(input: &str) -> usize {
    let mut crab_positions: Vec<usize> = Vec::new();

    for pos in input.split(',') {
        crab_positions.push(pos.parse::<usize>().unwrap());
    }
    crab_positions.sort_unstable();

    let mut fuel_consumption = usize::MAX;
    let min = *crab_positions.first().unwrap();
    let max = *crab_positions.last().unwrap();

    for hz_pos in min..=max {
        let fuel_consump_sum: usize = crab_positions
            .iter()
            .map(|&pos| (pos as isize - hz_pos as isize).abs() as usize)
            .sum();
        if fuel_consump_sum < fuel_consumption {
            fuel_consumption = fuel_consump_sum;
        }
    }
    fuel_consumption
}

fn part2(input: &str) -> usize {
    let mut crab_positions: Vec<usize> = Vec::new();

    for pos in input.split(',') {
        crab_positions.push(pos.parse::<usize>().unwrap());
    }
    crab_positions.sort_unstable();

    let mut fuel_consumption = usize::MAX;
    let min = *crab_positions.first().unwrap();
    let max = *crab_positions.last().unwrap();

    for hz_pos in min..=max {
        let fuel_consump_sum: usize = crab_positions
            .iter()
            .map(|&pos| {
                let n = (pos as isize - hz_pos as isize).abs() as usize;
                n * (n + 1) / 2
            })
            .sum();
        if fuel_consump_sum < fuel_consumption {
            fuel_consumption = fuel_consump_sum;
        }
    }
    fuel_consumption
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
        assert_eq!(part1(input), 37);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 168);
    }
}
