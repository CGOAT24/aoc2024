use std::collections::{HashSet, LinkedList};
use std::ptr::replace;

enum CellType {
    Empty,
    Obstacle
}

impl CellType {
    pub(crate) fn clone(&self) -> CellType {
        match self {
            CellType::Empty => CellType::Empty,
            CellType::Obstacle => CellType::Obstacle
        }
    }
}

struct Cell {
    cell_type: CellType,
    position: (usize, usize)
}

struct Map {
    cells: Vec<Vec<Cell>>,
    player_position: (usize, usize),
    visited: HashSet<(usize, usize)>
}

impl Map {
    fn at(&self, pos: (usize, usize)) -> Option<CellType> {
        self.cells.get(pos.0).and_then(|row| row.get(pos.1)).map(|cell| cell.cell_type.clone())
    }

    fn print(&self, obstacle: (usize, usize)) {
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len() {
                if self.cells[i][j].position == self.player_position {
                    print!("^");
                    continue;
                }

                if self.visited.contains(&self.cells[i][j].position) {
                    print!("X");
                    continue;
                }

                if self.cells[i][j].position == obstacle {
                    print!("O");
                    continue;
                }

                match self.cells[i][j].cell_type {
                    CellType::Empty => print!("."),
                    CellType::Obstacle => print!("#")
                }
            }
            println!();
        }
        println!();
    }
}

impl Clone for Map {
    fn clone(&self) -> Self {
        Map {
            cells: self.cells.iter().map(|row| row.iter().map(|cell| Cell { cell_type: cell.cell_type.clone(), position: cell.position }).collect()).collect(),
            player_position: self.player_position,
            visited: self.visited.clone()
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn part1(map: &mut Map) -> HashSet<(usize, usize)> {
    let mut current_dir = Direction::Up;

    loop {
        map.visited.insert(map.player_position);

        if is_at_edge(map.player_position, (map.cells.len(), map.cells[0].len()), &current_dir) {
            break;
        }

        let mut next_position = get_next_position(&map, &current_dir);
        if matches!(map.at(next_position).unwrap(), CellType::Obstacle) {
            current_dir = match current_dir {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down
            };
            next_position = get_next_position(&map, &current_dir);
        }

        map.player_position = next_position;
    }
    map.visited.clone()
}

fn part2(map: &mut Map, og_visited: &HashSet<(usize, usize)>) -> usize {
    let mut potential = LinkedList::from_iter(og_visited.iter());
    let mut valid_count = 0;
    let starting_pos = map.player_position;

    while potential.len() > 0 {
        let elem = potential.pop_back().unwrap();
        let mut current_dir = Direction::Up;
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        map.player_position = starting_pos;

        loop {
            map.print(*elem);
            if visited.contains(&map.player_position) {
                break;
            }
            visited.insert(map.player_position);

            if is_at_edge(map.player_position, (map.cells.len(), map.cells[0].len()), &current_dir) {
                break;
            }

            let mut next_position = get_next_position(&map, &current_dir);
            if matches!(map.at(next_position).unwrap(), CellType::Obstacle) || next_position == *elem {
                current_dir = match current_dir {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down
                };
                next_position = get_next_position(&map, &current_dir);
            }

            map.player_position = next_position;
        }
        if visited.len() > og_visited.len() {
            valid_count += 1;
        }
    }
    valid_count
}

fn is_at_edge(current_position: (usize, usize), map_size: (usize, usize), direction: &Direction) -> bool {
    match direction {
        Direction::Up => current_position.0 == 0,
        Direction::Down => current_position.0 == map_size.0 - 1,
        Direction::Left => current_position.1 == 0,
        Direction::Right => current_position.1 == map_size.1 - 1
    }
}

fn get_next_position(map: &Map, direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::Up => (map.player_position.0 - 1, map.player_position.1),
        Direction::Down => (map.player_position.0 + 1, map.player_position.1),
        Direction::Left => (map.player_position.0, map.player_position.1 - 1),
        Direction::Right => (map.player_position.0, map.player_position.1 + 1)
    }
}

fn parse_map(lines: &Vec<String>) -> Map {
    let mut cells: Vec<Vec<Cell>> = Vec::new();
    let mut player_position = (0, 0);
    for i in 0..lines.len() {
        let mut row = Vec::new();
        for j in 0..lines[i].chars().count() {

            match lines[i].chars().nth(j).unwrap() {
                '#' => row.push(Cell { cell_type: CellType::Obstacle, position: (i, j) }),
                '^' =>  {
                    player_position = (i, j);
                    row.push(Cell { cell_type: CellType::Empty, position: (i, j) })
                },
                _ => row.push(Cell { cell_type: CellType::Empty, position: (i, j) })
            }
        }
        cells.push(row);
    }
    Map {
        cells,
        player_position,
        visited: HashSet::new()
    }
}

fn main() {
    let lines = utils::read_file("input_test.txt".as_ref()).unwrap();

    let mut map = parse_map(&lines);

    let visited = part1(&mut map.clone());

    let result2 = part2(&mut map, &visited);

    println!("Part 1: {}", visited.len());
    println!("Part 2: {}", result2);
}
