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
        assert_eq!(part1(input), 198);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 230);
    }
}
