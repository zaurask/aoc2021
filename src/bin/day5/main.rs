struct Vec2 {
    x: usize,
    y: usize,
}
impl From<(&str, &str)> for Vec2 {
    fn from((x, y): (&str, &str)) -> Self {
        Self {
            x: x.parse::<usize>().unwrap(),
            y: y.parse::<usize>().unwrap(),
        }
    }
}
impl From<(usize, usize)> for Vec2 {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

enum Tile {
    Dot,
    Number(usize),
}
impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Dot => 0usize,
            Tile::Number(num) => *num,
        };
        write!(f, "{}", c)
    }
}
impl Default for Tile {
    fn default() -> Self {
        Self::Dot
    }
}

struct Diagram {
    size: Vec2,
    tiles: Vec<Tile>,
}

impl Diagram {
    fn new(size: Vec2) -> Self {
        let num_tiles = size.x * size.y;
        Self {
            size,
            tiles: (0..num_tiles)
                .into_iter()
                .map(|_| Default::default())
                .collect(),
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.size.y
    }

    fn set(&mut self, pos: usize) {
        match self.tiles[pos] {
            Tile::Dot => self.tiles[pos] = Tile::Number(1),
            Tile::Number(num) => self.tiles[pos] = Tile::Number(num + 1),
        }
    }

    fn set_hz_and_vert(&mut self, lines: &[(Vec2, Vec2)]) {
        for (p1, p2) in lines {
            if p1.x == p2.x {
                for y in p1.y.min(p2.y)..=p1.y.max(p2.y) {
                    self.set(self.index(p1.x, y));
                }
            } else if p1.y == p2.y {
                for x in p1.x.min(p2.x)..=p1.x.max(p2.x) {
                    self.set(self.index(x, p1.y));
                }
            }
        }
    }

    fn set_diag(&mut self, lines: &[(Vec2, Vec2)]) {
        for (p1, p2) in lines {
            if p1.x != p2.x && p1.y != p2.y {
                let delta_x: isize = if p1.x > p2.x { -1 } else { 1 };
                let delta_y: isize = if p1.y > p2.y { -1 } else { 1 };
                let mut x = p1.x as isize;
                let mut y = p1.y as isize;
                loop {
                    self.set(self.index(x as usize, y as usize));
                    if x == p2.x as isize {
                        break;
                    }
                    x += delta_x;
                    y += delta_y;
                }
            }
        }
    }
}

impl std::fmt::Debug for Diagram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                write!(f, "{:?}", self.tiles[self.index(x, y)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
fn get_dimensions(lines: &[(Vec2, Vec2)], size: &mut Vec2) {
    for (p1, p2) in lines.iter() {
        if p1.x > p2.x {
            if p1.x > size.x {
                size.x = p1.x;
            }
        } else if p2.x > size.x {
            size.x = p2.x;
        }
        if p1.y > p2.y {
            if p1.y > size.y {
                size.y = p1.y;
            }
        } else if p2.y > size.y {
            size.y = p2.y;
        }
    }
    size.x += 1;
    size.y += 1;
}
fn part1(input: &str) -> usize {
    let lines: Vec<(Vec2, Vec2)> = input
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(start, end)| (start.split_once(',').unwrap(), end.split_once(',').unwrap()))
        .map(|(p1, p2)| (p1.into(), p2.into()))
        .collect();
    let mut size: Vec2 = (0, 0).into();
    get_dimensions(&lines, &mut size);
    let mut diagram = Diagram::new(size);
    diagram.set_hz_and_vert(&lines);
    diagram
        .tiles
        .into_iter()
        .filter(|t| match t {
            Tile::Dot => false,
            Tile::Number(num) => num > &1,
        })
        .count()
}

fn part2(input: &str) -> usize {
    let lines: Vec<(Vec2, Vec2)> = input
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(start, end)| (start.split_once(',').unwrap(), end.split_once(',').unwrap()))
        .map(|(p1, p2)| (p1.into(), p2.into()))
        .collect();
    let mut size: Vec2 = (0, 0).into();
    get_dimensions(&lines, &mut size);
    let mut diagram = Diagram::new(size);
    diagram.set_hz_and_vert(&lines);
    diagram.set_diag(&lines);
    diagram
        .tiles
        .into_iter()
        .filter(|t| match t {
            Tile::Dot => false,
            Tile::Number(num) => num > &1,
        })
        .count()
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
        assert_eq!(part1(input), 5);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 12);
    }
}
