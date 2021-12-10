use std::collections::HashSet;

fn part1(input: &str) -> usize {
    input
        .lines()
        .flat_map(|s| s.split_once(" | ").unwrap().1.split(' '))
        .map(|s| s.len())
        .filter(|&x| matches!(x, 2 | 3 | 4 | 7))
        .count()
}

fn part2(input: &str) -> usize {
    let entries: Vec<(&str, &str)> = input
        .lines()
        .map(|s| s.split_once(" | ").unwrap())
        .collect();
    let mut sum = 0;
    for (input, output) in entries {
        let character_sets: Vec<HashSet<char>> = input
            .split(' ')
            .map(|s| HashSet::from_iter(s.chars()))
            .collect();
        let mut map_chars_to_num: Vec<Option<HashSet<char>>> = vec![None; 10];
        let mut todo = vec![];
        for set in character_sets {
            match set.len() {
                2 => map_chars_to_num[1] = Some(set),
                3 => map_chars_to_num[7] = Some(set),
                4 => map_chars_to_num[4] = Some(set),
                7 => map_chars_to_num[8] = Some(set),
                _ => todo.push(set),
            }
        }

        let character_sets = todo;
        let mut todo = vec![];
        for set in character_sets {
            if set.len() == 6 && set.is_superset(map_chars_to_num[4].as_ref().unwrap()) {
                map_chars_to_num[9] = Some(set)
            } else if set.len() == 5 && set.is_superset(map_chars_to_num[1].as_ref().unwrap()) {
                map_chars_to_num[3] = Some(set)
            } else {
                todo.push(set)
            }
        }

        let character_sets = todo;
        let mut todo = vec![];
        for set in character_sets {
            if set.len() == 6 && set.is_superset(map_chars_to_num[1].as_ref().unwrap()) {
                map_chars_to_num[0] = Some(set)
            } else {
                todo.push(set)
            }
        }

        let character_sets = todo;
        let mut todo = vec![];
        for set in character_sets {
            if set.len() == 6 {
                map_chars_to_num[6] = Some(set)
            } else {
                todo.push(set)
            }
        }

        let character_sets = todo;
        for set in character_sets {
            if set.is_subset(map_chars_to_num[6].as_ref().unwrap()) {
                map_chars_to_num[5] = Some(set)
            } else {
                map_chars_to_num[2] = Some(set)
            }
        }
        let map_chars_to_num: Vec<HashSet<char>> =
            map_chars_to_num.into_iter().map(|o| o.unwrap()).collect();

        let mut display = 0;
        for s in output.split(' ') {
            let set = HashSet::from_iter(s.chars());
            let digit = map_chars_to_num.iter().position(|cmp| cmp == &set).unwrap();
            display *= 10;
            display += digit;
        }
        sum += display;
    }
    sum
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
        assert_eq!(part1(input), 26);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 61229);
    }
}
