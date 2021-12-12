fn part1(input: &str) -> usize {
    let mut points = 0;
    for line in input.lines() {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' => {
                    if stack.pop().unwrap() != '(' {
                        points += 3;
                    }
                }
                ']' => {
                    if stack.pop().unwrap() != '[' {
                        points += 57;
                    }
                }
                '}' => {
                    if stack.pop().unwrap() != '{' {
                        points += 1197;
                    }
                }
                '>' => {
                    if stack.pop().unwrap() != '<' {
                        points += 25137;
                    }
                }
                _ => (),
            }
        }
    }
    points
}
fn is_corrupted(line: &str) -> bool {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            _ => {
                if stack.pop().unwrap()
                    != match c {
                        ')' => '(',
                        ']' => '[',
                        '}' => '{',
                        '>' => '<',
                        _ => unreachable!(),
                    }
                {
                    return true;
                }
            }
        }
    }
    false
}
fn part2(input: &str) -> usize {
    let mut scores: Vec<usize> = Vec::new();
    for line in input.lines() {
        if is_corrupted(line) {
            continue;
        }

        let mut stack = Vec::new();
        let mut completion = Vec::new();
        for c in line.chars().rev() {
            match c {
                ')' | ']' | '}' | '>' => stack.push(c),
                _ => {
                    if stack.is_empty() {
                        let c = match c {
                            '(' => ')',
                            '[' => ']',
                            '{' => '}',
                            '<' => '>',
                            _ => unreachable!(),
                        };
                        completion.push(c);
                    } else {
                        stack.pop();
                    }
                }
            }
        }
        let mut score = 0;
        for c in completion {
            score *= 5;
            score += match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => 0,
            }
        }
        scores.push(score);
    }
    scores.sort_unstable();
    scores[scores.len() / 2]
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
        assert_eq!(part1(input), 26397);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 288957);
    }
}
