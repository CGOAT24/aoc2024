use std::collections::{HashSet, LinkedList};

#[derive(Clone, Eq, Hash, PartialEq)]
enum CellType {
    Empty,
    Obstacle
}

impl CellType {
    fn clone(&self) -> CellType {
        match self {
            CellType::Empty => CellType::Empty,
            CellType::Obstacle => CellType::Obstacle
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Cell {
    cell_type: CellType,
    position: (usize, usize)
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Visit {
    position: (usize, usize),
    direction: Direction
}

struct Map {
    cells: Vec<Vec<Cell>>,
    guard_pos: (usize, usize),
    visited: HashSet<Visit>,
    current_dir: Direction
}

impl Map {
    fn at(&self, pos: (usize, usize)) -> Option<Cell> {
        if pos.0 >= self.cells.len() || pos.1 >= self.cells[0].len() {
            return None;
        }
        Some(self.cells[pos.0][pos.1].clone())
    }

    fn get_next_position(&self) -> Cell {
        match self.current_dir{
            Direction::Up => self.at((self.guard_pos.0 - 1, self.guard_pos.1)).unwrap(),
            Direction::Down => self.at((self.guard_pos.0 + 1, self.guard_pos.1)).unwrap(),
            Direction::Left => self.at((self.guard_pos.0, self.guard_pos.1 - 1)).unwrap(),
            Direction::Right => self.at((self.guard_pos.0, self.guard_pos.1 + 1)).unwrap()
        }
    }

    fn is_at_edge(&self) -> bool {
        match self.current_dir {
            Direction::Up => self.guard_pos.0 == 0,
            Direction::Down => self.guard_pos.0 == self.cells.len() - 1,
            Direction::Left => self.guard_pos.1 == 0,
            Direction::Right => self.guard_pos.1 == self.cells[0].len() - 1
        }
    }

    fn change_direction(&mut self) {
        self.current_dir = match self.current_dir {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down
        };
    }

    fn get_unique_visited(&self) -> HashSet<(usize, usize)> {
        self.visited.iter().map(|visit| visit.position).collect()
    }

    fn print(&self) {
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len() {
                if self.cells[i][j].position == self.guard_pos {
                    print!("^");
                    continue;
                }
                if self.visited.iter().any(|visit| visit.position == (i, j)) {
                    print!("X");
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

    fn print_obstacle(&self, obstacle: (usize, usize)) {
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len() {
                if self.cells[i][j].position == self.guard_pos {
                    print!("^");
                    continue;
                }
                if self.visited.contains(&Visit { position: (i, j), direction: self.current_dir.clone() }) {
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
            guard_pos: self.guard_pos,
            visited: self.visited.clone(),
            current_dir: self.current_dir.clone(),
        }
    }
}
#[derive(Eq, Hash, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn part1(map: &mut Map) {
    loop {
        map.visited.insert(Visit { position: map.guard_pos, direction: map.current_dir.clone() });

        if map.is_at_edge() {
            break;
        }

        let mut next_position = map.get_next_position();
        if matches!(next_position.cell_type, CellType::Obstacle) {
            map.change_direction();
            next_position = map.get_next_position();
        }

        map.guard_pos = next_position.position;
    }
}

fn part2(map: &mut Map, visited: HashSet<Visit>) -> usize {
    let mut potential = LinkedList::from_iter(visited
        .iter()
        .map(|visit| visit.position)
        .collect::<HashSet<(usize, usize)>>()
    );
    let mut valid_obstacles = HashSet::new();
    let starting_pos = map.guard_pos;

    while potential.len() > 0 {
        let elem = potential.pop_back().unwrap();
        let mut current_map = map.clone();
        current_map.cells[elem.0][elem.1].cell_type = CellType::Obstacle;
        map.guard_pos = starting_pos;

        if is_a_cycle(current_map) {
            println!("Cycle at {:?}", elem);
            valid_obstacles.insert(elem);
        }
        else {
            println!("Not a cycle at {:?}", elem);
        }
    }
    valid_obstacles.len()
}

fn is_a_cycle(mut map: Map) -> bool {
    loop {
        let is_not_cycle = map.visited.insert(Visit { position: map.guard_pos, direction: map.current_dir.clone() });

        if !is_not_cycle {
            return true;
        }

        if map.is_at_edge() {
            break;
        }

        let mut next_position = map.get_next_position();
        if matches!(next_position.cell_type, CellType::Obstacle) {
            map.change_direction();
            next_position = map.get_next_position();
        }

        map.guard_pos = next_position.position;
    }
    false
}

fn parse_map(lines: &Vec<String>) -> Map {
    let mut cells: Vec<Vec<Cell>> = Vec::new();
    let mut guard_pos = (0, 0);
    for i in 0..lines.len() {
        let mut row = Vec::new();
        for j in 0..lines[i].chars().count() {
            match lines[i].chars().nth(j).unwrap() {
                '#' => row.push(Cell { cell_type: CellType::Obstacle, position: (i, j) }),
                '^' =>  {
                    guard_pos = (i, j);
                    row.push(Cell { cell_type: CellType::Empty, position: (i, j) })
                },
                _ => row.push(Cell { cell_type: CellType::Empty, position: (i, j) })
            }
        }
        cells.push(row);
    }
    Map {
        cells,
        guard_pos,
        visited: HashSet::new(),
        current_dir: Direction::Up
    }
}

fn main() {
    let lines = utils::read_file("input.txt".as_ref()).unwrap();
    let mut map = parse_map(&lines);

    let mut map1 = map.clone();
    part1(&mut map1);

    let result2 = part2(&mut map, map1.visited.clone());

    println!("Part 1: {}", map1.get_unique_visited().len());
    println!("Part 2: {}", result2);
}
