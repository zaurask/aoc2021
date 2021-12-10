fn part1(input: &str) -> u32 {
    let x_max = input.lines().next().unwrap().chars().count() + 2;
    let y_max = input.lines().count() + 2;
    let mut grid: Vec<Vec<u32>> = vec![vec![9; y_max]; x_max];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[x + 1][y + 1] = c.to_digit(10).unwrap();
        }
    }

    let mut risk_level = 0;

    for x in 1..x_max - 1 {
        for y in 1..y_max - 1 {
            if grid[x][y] < grid[x + 1][y]
                && grid[x][y] < grid[x - 1][y]
                && grid[x][y] < grid[x][y + 1]
                && grid[x][y] < grid[x][y - 1]
            {
                risk_level += grid[x][y] + 1;
            }
        }
    }
    risk_level
}

fn get_lowpoints(grid: &[Vec<Location>], x_max: usize, y_max: usize) -> Vec<(usize, usize)> {
    let mut lowpoints = Vec::new();

    for x in 1..x_max - 1 {
        for y in 1..y_max - 1 {
            if grid[x][y].height < grid[x + 1][y].height
                && grid[x][y].height < grid[x - 1][y].height
                && grid[x][y].height < grid[x][y + 1].height
                && grid[x][y].height < grid[x][y - 1].height
            {
                lowpoints.push((x, y));
            }
        }
    }
    lowpoints
}
fn fill(x: usize, y: usize, grid: &mut Vec<Vec<Location>>) -> usize {
    if grid[x][y].height == 9 || grid[x][y].filled {
        return 0;
    }
    grid[x][y].filled = true;
    let mut n_filled = 1;

    n_filled += fill(x - 1, y, grid);
    n_filled += fill(x + 1, y, grid);
    n_filled += fill(x, y + 1, grid);
    n_filled += fill(x, y - 1, grid);

    n_filled
}

#[derive(Copy, Clone)]
struct Location {
    height: u32,
    filled: bool,
}

impl Default for Location {
    fn default() -> Self {
        Self {
            height: 9,
            filled: false,
        }
    }
}
fn part2(input: &str) -> usize {
    let x_max = input.lines().next().unwrap().chars().count() + 2;
    let y_max = input.lines().count() + 2;

    let mut grid: Vec<Vec<Location>> = vec![vec![Location::default(); y_max]; x_max];

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[x + 1][y + 1].height = c.to_digit(10).unwrap();
        }
    }

    let lowpoints = get_lowpoints(&grid, x_max, y_max);

    let mut basin_sizes = Vec::new();
    for lowpoint in lowpoints {
        basin_sizes.push(fill(lowpoint.0, lowpoint.1, &mut grid));
    }
    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).product()
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
        assert_eq!(part1(input), 15);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 1134);
    }
}
