use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Border,
    Flashed,
    Level(u8),
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Border
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Border => write!(f, "-"),
            Self::Flashed => write!(f, "*"),
            Self::Level(l) => write!(f, "{}", l),
        }
    }
}

fn flash(grid: &mut [[Tile; 12]; 12], x: usize, y: usize, flashes: &mut usize) {
    *flashes += 1;
    grid[x][y] = Tile::Flashed;

    for x in (x - 1)..=(x + 1) {
        for y in (y - 1)..=(y + 1) {
            match &mut grid[x][y] {
                Tile::Border => continue,
                Tile::Flashed => continue,
                Tile::Level(l) => {
                    *l += 1;
                    if *l > 9 {
                        flash(grid, x, y, flashes);
                    }
                }
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let mut grid = [[Tile::default(); 12]; 12];

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[x + 1][y + 1] = Tile::Level(c.to_digit(10).unwrap() as u8);
        }
    }
    let mut flashes = 0;

    for _ in 0..100 {
        for row in grid.iter_mut() {
            for tile in row.iter_mut() {
                match tile {
                    Tile::Border => continue,
                    Tile::Flashed => continue,
                    Tile::Level(l) => *l += 1,
                }
            }
        }
        for x in 1..11 {
            for y in 1..11 {
                match grid[x][y] {
                    Tile::Border => continue,
                    Tile::Flashed => continue,
                    Tile::Level(l) => {
                        if l > 9 {
                            flash(&mut grid, x, y, &mut flashes);
                        }
                    }
                }
            }
        }

        for row in grid.iter_mut() {
            for tile in row.iter_mut() {
                match tile {
                    Tile::Border => continue,
                    Tile::Flashed => *tile = Tile::Level(0),
                    Tile::Level(_) => continue,
                }
            }
        }
    }
    flashes
}
fn part2(input: &str) -> usize {
    let mut grid = [[Tile::default(); 12]; 12];

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[x + 1][y + 1] = Tile::Level(c.to_digit(10).unwrap() as u8);
        }
    }
    let mut flashes = 0;

    for step in 1.. {
        for row in grid.iter_mut() {
            for tile in row.iter_mut() {
                match tile {
                    Tile::Border => continue,
                    Tile::Flashed => continue,
                    Tile::Level(l) => *l += 1,
                }
            }
        }
        for x in 1..11 {
            for y in 1..11 {
                match grid[x][y] {
                    Tile::Border => continue,
                    Tile::Flashed => continue,
                    Tile::Level(l) => {
                        if l > 9 {
                            flash(&mut grid, x, y, &mut flashes);
                        }
                    }
                }
            }
        }

        let all_flashed = grid.iter().all(|row| {
            row.iter()
                .all(|&tile| matches!(tile, Tile::Border | Tile::Flashed))
        });
        if all_flashed {
            return step;
        }

        for row in grid.iter_mut() {
            for tile in row.iter_mut() {
                match tile {
                    Tile::Border => continue,
                    Tile::Flashed => *tile = Tile::Level(0),
                    Tile::Level(_) => continue,
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
        assert_eq!(part1(input), 1656);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 195);
    }
}
