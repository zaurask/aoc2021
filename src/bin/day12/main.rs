use std::collections::{hash_map::Entry, HashMap, HashSet};

#[derive(PartialEq)]
enum Cave {
    Big,
    Small,
    Start,
    End,
}

fn traverse(
    adjacency_matrix: &[Vec<bool>],
    caves: &[Cave],
    idx: usize,
    mut visited: HashSet<usize>,
) -> usize {
    match caves[idx] {
        Cave::Start => 0,
        Cave::End => 1,
        Cave::Big | Cave::Small => {
            if caves[idx] == Cave::Small {
                visited.insert(idx);
            }
            adjacency_matrix[idx]
                .iter()
                .enumerate()
                .skip(1)
                .filter(|(i, is_connected)| **is_connected && !visited.contains(i))
                .map(|(i, _)| traverse(adjacency_matrix, caves, i, visited.clone()))
                .sum()
        }
    }
}

fn part1(input: &str) -> usize {
    let mut caves: Vec<Cave> = Vec::new();
    let mut created_caves: HashMap<&str, usize> = HashMap::new();

    caves.push(Cave::Start);
    created_caves.insert("start", 0);
    created_caves.insert("end", 0);

    for cave in input.lines().flat_map(|line| line.split('-')) {
        match created_caves.entry(cave) {
            Entry::Occupied(_) => continue,
            Entry::Vacant(_) => {
                if cave.chars().all(|c| c.is_uppercase()) {
                    caves.push(Cave::Big);
                } else {
                    caves.push(Cave::Small);
                }
                created_caves.insert(cave, caves.len() - 1);
            }
        };
    }
    caves.push(Cave::End);
    let entry = created_caves.entry("end").or_default();
    *entry = caves.len() - 1;

    let mut adjacency_matrix = vec![vec![false; caves.len()]; caves.len()];
    for (from, to) in input.lines().map(|line| line.split_once('-').unwrap()) {
        let x = created_caves.get(from).unwrap();
        let y = created_caves.get(to).unwrap();

        adjacency_matrix[*x][*y] = true;
        adjacency_matrix[*y][*x] = true;
    }

    let mut paths = 0;
    for (idx, is_connected) in adjacency_matrix[0].iter().enumerate().skip(1) {
        if *is_connected {
            let visited: HashSet<usize> = HashSet::new();
            paths += traverse(&adjacency_matrix, &caves, idx, visited);
        }
    }
    paths
}

fn traverse_2(
    adjacency_matrix: &[Vec<bool>],
    caves: &[Cave],
    idx: usize,
    mut visited: HashSet<usize>,
    mut twice: bool,
) -> usize {
    match caves[idx] {
        Cave::Start => 0,
        Cave::End => 1,
        Cave::Big | Cave::Small => {
            if visited.contains(&idx) {
                if twice {
                    return 0;
                } else {
                    twice = true;
                }
            }
            if caves[idx] == Cave::Small {
                visited.insert(idx);
            }
            adjacency_matrix[idx]
                .iter()
                .enumerate()
                .skip(1)
                .filter(|(_, is_connected)| **is_connected)
                .map(|(i, _)| traverse_2(adjacency_matrix, caves, i, visited.clone(), twice))
                .sum()
        }
    }
}

fn part2(input: &str) -> usize {
    let mut caves: Vec<Cave> = Vec::new();
    let mut created_caves: HashMap<&str, usize> = HashMap::new();

    caves.push(Cave::Start);
    created_caves.insert("start", 0);
    created_caves.insert("end", 0);

    for cave in input.lines().flat_map(|line| line.split('-')) {
        match created_caves.entry(cave) {
            Entry::Occupied(_) => continue,
            Entry::Vacant(_) => {
                if cave.chars().all(|c| c.is_uppercase()) {
                    caves.push(Cave::Big);
                } else {
                    caves.push(Cave::Small);
                }
                created_caves.insert(cave, caves.len() - 1);
            }
        };
    }
    caves.push(Cave::End);
    let entry = created_caves.entry("end").or_default();
    *entry = caves.len() - 1;

    let mut adjacency_matrix = vec![vec![false; caves.len()]; caves.len()];
    for (from, to) in input.lines().map(|line| line.split_once('-').unwrap()) {
        let x = created_caves.get(from).unwrap();
        let y = created_caves.get(to).unwrap();

        adjacency_matrix[*x][*y] = true;
        adjacency_matrix[*y][*x] = true;
    }

    let mut paths = 0;
    for (idx, is_connected) in adjacency_matrix[0].iter().enumerate().skip(1) {
        if *is_connected {
            let visited: HashSet<usize> = HashSet::new();
            paths += traverse_2(&adjacency_matrix, &caves, idx, visited, false);
        }
    }
    paths
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
    fn test_part1_1() {
        let input = include_str!("test_input_1.txt");
        assert_eq!(part1(input), 10);
    }
    #[test]
    fn test_part1_2() {
        let input = include_str!("test_input_2.txt");
        assert_eq!(part1(input), 19);
    }
    #[test]
    fn test_part1_3() {
        let input = include_str!("test_input_3.txt");
        assert_eq!(part1(input), 226);
    }

    #[test]
    fn test_part2_1() {
        let input = include_str!("test_input_1.txt");
        assert_eq!(part2(input), 36);
    }
    #[test]
    fn test_part2_2() {
        let input = include_str!("test_input_2.txt");
        assert_eq!(part2(input), 103);
    }
    #[test]
    fn test_part2_3() {
        let input = include_str!("test_input_3.txt");
        assert_eq!(part2(input), 3509);
    }
}
