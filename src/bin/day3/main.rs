fn part1(input: &str) -> u32 {
    let line_length = input.lines().next().unwrap().len();
    let num_of_lines = input.lines().count();
    let byte_arr = input.as_bytes();

    let mut bits: Vec<u8> = vec![b'0'; line_length];
    let mut count = 0;
    for pos in 0..line_length {
        for line in 0..num_of_lines {
            if byte_arr[pos + (line * (line_length + 1))] == b'1' {
                count += 1;
            }
        }
        if count > num_of_lines / 2 {
            bits[pos] = b'1';
        }
        count = 0;
    }
    let gamma = u32::from_str_radix(std::str::from_utf8(&bits).unwrap(), 2).unwrap();
    for bit in bits.iter_mut() {
        if *bit == b'1' {
            *bit = b'0';
        } else {
            *bit = b'1';
        }
    }
    let epsilon = u32::from_str_radix(std::str::from_utf8(&bits).unwrap(), 2).unwrap();
    epsilon * gamma
}

fn part2(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().map(|s| s.as_bytes()).collect();

    let mut oxygen = lines.clone();
    for pos in 0.. {
        let mut ones = 0;
        for num in oxygen.clone().into_iter() {
            if num[pos] == b'1' {
                ones += 1;
            }
        }
        let keep = ones >= (oxygen.len() - ones);
        oxygen = oxygen
            .clone()
            .into_iter()
            .filter(|num| {
                let num = num[pos];
                if keep {
                    num == b'1'
                } else {
                    num == b'0'
                }
            })
            .collect();

        if oxygen.len() == 1 {
            break;
        }
    }
    let oxygen_rating = u32::from_str_radix(std::str::from_utf8(oxygen[0]).unwrap(), 2).unwrap();

    let mut co2 = lines;
    for pos in 0.. {
        let mut zeros = 0;
        for num in co2.clone().into_iter() {
            if num[pos] == b'0' {
                zeros += 1;
            }
        }
        let keep = zeros <= (co2.len() - zeros);
        co2 = co2
            .clone()
            .into_iter()
            .filter(|num| {
                let num = num[pos];
                if keep {
                    num == b'0'
                } else {
                    num == b'1'
                }
            })
            .collect();

        if co2.len() == 1 {
            break;
        }
    }
    let co2_rating = u32::from_str_radix(std::str::from_utf8(co2[0]).unwrap(), 2).unwrap();

    co2_rating * oxygen_rating
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
        assert_eq!(part1(input), 198);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 230);
    }
}
