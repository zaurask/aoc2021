fn check_and_mark(bingo_board: &mut [(usize, bool)], num: usize) {
    for mut number in bingo_board {
        if number.0 == num {
            (*number).1 = true;
            break;
        }
    }
}

fn bingo(bingo_board: &[(usize, bool)]) -> bool {
    for row in 0..5 {
        let offset = row * 5;
        if (bingo_board[offset].1
            && bingo_board[1 + offset].1
            && bingo_board[2 + offset].1
            && bingo_board[3 + offset].1
            && bingo_board[4 + offset].1)
            || (bingo_board[row].1
                && bingo_board[row + 5].1
                && bingo_board[row + 10].1
                && bingo_board[row + 15].1
                && bingo_board[row + 20].1)
        {
            return true;
        }
    }
    false
}

fn part1(input: &str) -> usize {
    let numbers: Vec<usize> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let mut bingo_boards = Vec::new();
    let mut bingo_board = [(0, false); 25];
    let mut row = 0;
    for line in input.lines().skip(2) {
        if line.is_empty() {
            bingo_boards.push(bingo_board);
            bingo_board = [(0, false); 25];
            row = 0;
            continue;
        }
        for (pos, s) in line.split_whitespace().enumerate() {
            bingo_board[pos + 5 * row].0 = s.parse::<usize>().unwrap();
        }
        row += 1;
    }
    bingo_boards.push(bingo_board);
    for num in numbers {
        for bingo_board in &mut bingo_boards {
            check_and_mark(bingo_board, num);
            if bingo(bingo_board) {
                let sum: usize = bingo_board
                    .iter()
                    .filter_map(|(x, marked)| if !marked { Some(x) } else { None })
                    .sum();
                return num * sum;
            }
        }
    }
    unreachable!()
}

fn part2(input: &str) -> usize {
    let numbers: Vec<usize> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let mut bingo_boards = Vec::new();
    let mut bingo_board = [(0, false); 25];
    let mut row = 0;
    for line in input.lines().skip(2) {
        if line.is_empty() {
            bingo_boards.push(bingo_board);
            bingo_board = [(0, false); 25];
            row = 0;
            continue;
        }
        for (pos, s) in line.split_whitespace().enumerate() {
            bingo_board[pos + 5 * row].0 = s.parse::<usize>().unwrap();
        }
        row += 1;
    }
    bingo_boards.push(bingo_board);
    let num_of_boards = bingo_boards.len();
    let mut skip = Vec::<usize>::new();
    for num in numbers {
        for (idx, bingo_board) in bingo_boards.iter_mut().enumerate() {
            if skip.contains(&idx) {
                continue;
            }
            check_and_mark(bingo_board, num);
            if bingo(bingo_board) {
                if skip.len() == num_of_boards - 1 {
                    let sum: usize = bingo_board
                        .iter()
                        .filter_map(|(x, marked)| if !marked { Some(x) } else { None })
                        .sum();
                    return num * sum;
                } else {
                    skip.push(idx);
                }
            }
        }
    }
    unreachable!()
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
        assert_eq!(part1(input), 4512);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 1924);
    }
}
