use std::vec;

use crate::utility::{Uvec2, Direction};
use rand::Rng;
use rand::seq::{SliceRandom};


pub struct Hamilton {
    size: Uvec2,
    len: usize,
    spanning_tree: SpanningTree,
    pub cycle: Vec<usize>,
}

impl Hamilton {
    pub fn new(size: Uvec2) -> Self {
        let spanning_tree_size = Uvec2 { x: size.x / 2, y: size.y / 2};
        let mut spanning_tree = SpanningTree::new(spanning_tree_size);
        spanning_tree.build();
        let cycle: Vec<usize> = vec![0; (size.x * size.y) as usize];
        Self { size: size, len: size.x * size.y, spanning_tree: spanning_tree, cycle:cycle }
    }

    pub fn build(&mut self) {
        let total_size: usize = self.size.x * self.size.y;
        let turn_priority: Vec<fn(Direction) -> Direction> = vec![Direction::right_of, Direction::identity, Direction::left_of];
        let mut current_coordinates: Uvec2 = Uvec2 { x: 0, y: 0 };
        let mut current_facing: Direction = Direction::Up;
        let mut current_member: usize = 1;
        while current_member < total_size {
            // attempt to turn right, then straight, then left to hug the SpanningTree wall
            for i in 0..3 {
                let turn = turn_priority[i](current_facing);
                if self.check_direction(turn, &current_coordinates) {
                    current_facing = turn;
                    break;
                }
            }
            let option = current_coordinates.add_delta(current_facing, &self.size);
            current_coordinates = match option {
                Option::None => panic!("Invalid Step executed!"),
                Option::Some(x) => x,
            };
            self.set_member(current_coordinates, current_member);
            current_member += 1;
        }
    }

    fn check_direction(&self, turn: Direction, current_coordinates: &Uvec2) -> bool {
        let to_option: Option<Uvec2> = current_coordinates.add_delta(turn, &self.size);
        let to_coordinates = match to_option {
            Option::None => return false,
            Option::Some(ref c) => c,
        };

        let option: Option<Uvec2> = self.edge_to_check(current_coordinates, to_coordinates);
        let to_node: Uvec2 = match option {
            Option::None => return true,
            Option::Some(x) => x,
        };
        let current_node: &CellNode = self.spanning_tree.get(Uvec2 { x: current_coordinates.x / 2, y: current_coordinates.y / 2 });
        !current_node.has_edge(&to_node)
    }

    fn edge_to_check(&self, from: &Uvec2, to: &Uvec2) -> Option<Uvec2> {
        let from_tile: Uvec2 = Uvec2 { x: from.x / 2, y: from.y / 2 };
        let to_tile:   Uvec2 = Uvec2 { x: to.x / 2,   y: to.y / 2 };

        if from_tile == to_tile {
            let from_pos: Uvec2 = Uvec2 { x: from.x % 2, y: from.y % 2 };
            let to_pos:   Uvec2 = Uvec2 { x: to.x % 2,   y: to.y % 2 };

            let dx = to_pos.x as isize - from_pos.x as isize;
            let dy = to_pos.y as isize - from_pos.y as isize;

            let dir: Direction = match (dx, dy) {
                (0, _y) if from_pos.x == 0 => Direction::Left,
                (0, _y) if from_pos.x == 1 => Direction::Right,
                (_x, 0) if from_pos.y == 0 => Direction::Up,
                (_x, 0) if from_pos.y == 1 => Direction::Down,
                _ => panic!("Invalid Step Tested!"),
            };

            let check_option = from_tile.add_delta(dir, &self.spanning_tree.size);
            match check_option {
                Option::None => return Option::None,
                Option::Some(c) => return Option::Some(c),
            };
        }
        Option::None
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        let mut count = 0;
        let length: usize = self.size.x * self.size.y;
        for i in 0..self.cycle.len() {
            let coordinates = Uvec2 { x: i % self.size.x, y: i / self.size.x };
            let mut v: Vec<bool> = Vec::new();
            for d in Direction::ALL {
                let b = match coordinates.add_delta(d, &self.size) {
                    Option::None => false,
                    Option::Some(pos) => self.is_prev_or_next(length, pos, i),
                };
                v.push(b);
            }
            let c = match v.as_slice() {
                [false, false, true, true] => '│',
                [false, true, false, true] => '┐',
                [false, true, true, false] => '┘',
                [true, false, false, true] => '┌',
                [true, false, true, false] => '└',
                [true, true, false, false] => '─',
                _ => ' ',
            };
            print!("{}", c);
            if count % self.size.x == self.size.x - 1 {
                print!("\n");
            }

            count += 1;
        }
    }

    #[allow(dead_code)]
    fn is_prev_or_next(&self, length: usize, current: Uvec2, target: usize) -> bool {
        let current_index = current.y * self.size.x + current.x;
        let p = self.cycle[target] as i32 - 1;
        let prev;
        if p < 0 { prev = length - 1; }
        else { prev = p as usize }
        let n = self.cycle[target] + 1;
        let next;
        if n >= length { next = 0; }
        else { next = n; }
        self.cycle[current_index] == prev || self.cycle[current_index] == next
    }

    pub fn is_next(&self, current: usize, is_next: usize) -> bool {
        is_next == (current + 1) % self.len
    }

    pub fn is_between(&self, bound1: usize, bound2: usize, index: usize) -> bool {
        if bound1 > bound2 {
            index >= bound2 && index <= bound1
        }
        else {
            index >= bound2 || index <= bound1
        }
    } 

    pub fn get_member(&self, pos: Uvec2) -> usize {
        self.cycle[pos.y * self.size.x + pos.x]
    }

    fn set_member(&mut self, pos: Uvec2, member: usize) {
        self.cycle[pos.y * self.size.x + pos.x] = member;
    }
}

struct SpanningTree {
    size: Uvec2,
    length: usize,
    nodes: Vec<Option<CellNode>>,
    visited: Vec<bool>,
}

impl SpanningTree {
    fn new(size: Uvec2) -> Self {
        let length = size.x * size.y;
        let nodes: Vec<Option<CellNode>> = (0..length).map(|_| Option::None).collect();
        let visited: Vec<bool> = vec![false; length];
        Self { size: size, length: length, nodes: nodes, visited: visited }
    }

    fn set(&mut self, coordinates: Uvec2, node: CellNode) {
        self.nodes[coordinates.y * self.size.x + coordinates.x] = Option::Some(node);
    }

    pub fn get(&self, coordinates: Uvec2) -> &CellNode {
        let value: Option<&CellNode> = self.nodes[coordinates.y * self.size.x + coordinates.x].as_ref();
        match value {
            Option::None => panic!("Accessed Value is None!"),
            Option::Some(v) => v,
        }
    }

    pub fn build(&mut self) {
        let mut rng = rand::thread_rng();
        let start_x = rng.gen_range(0..self.size.x);
        let start_y = rng.gen_range(0..self.size.y);
        let start_coordinates: Uvec2 = Uvec2 { x: start_x, y: start_y };
        self.set(start_coordinates, CellNode::new(start_coordinates));
        let mut items: usize = 1;
        while items < self.length {
            let neighbors: Vec<Neighbor> = self.find_current_neighbors();
            let choice = neighbors.choose(&mut rng);
            if let Option::Some(nref) = choice {
                let from_idx = nref.from;
                let to_coord = nref.to;
                let mut to_node = CellNode::new(to_coord);
            
                {
                    let from_node = self.nodes[from_idx].as_mut().unwrap();
                    from_node.add_edge(to_coord);
                }
            
                let from_coord = {
                    let from_node = self.nodes[from_idx].as_ref().unwrap();
                    from_node.coordinates
                };
                to_node.add_edge(from_coord);
            
                self.set(to_coord, to_node);
            }
            else {
                panic!("No Neighbors Found!");
            }
            items += 1;
        }
    }

    fn find_current_neighbors(&mut self) -> Vec<Neighbor> {
        let mut neighbors: Vec<Neighbor> = vec![];
        self.set_visited();
        for i in 0..self.nodes.len() {
            let node = &self.nodes[i];
            if let Option::Some(nref) = node.as_ref() {
                let visible: Vec<Uvec2> = nref.neighbors(self.size);
                for n in visible {
                    let idx = n.y * self.size.x + n.x;
                    if !self.visited[idx] {
                        neighbors.push(Neighbor { from: i, to: n });
                        self.visited[idx] = true;
                    }
                }
            }
        }
        neighbors
    }

    fn set_visited(&mut self) {
        for (i, node) in self.nodes.iter().enumerate() {
            self.visited[i] = node.is_some();
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        let mut count = 0;
        for op in &self.nodes {
            let node = match op {
                Option::None => &CellNode::new(Uvec2 { x: self.size.x + 100, y: self.size.y + 100 }),
                Option::Some(x) => x,
            };
            let mut v: Vec<bool> = Vec::new();
            for d in Direction::ALL {
                let b = match node.get_coordinates().add_delta(d, &self.size) {
                    Option::None => false,
                    Option::Some(pos) => node.has_edge(&pos),
                };
                v.push(b);
            }

            let c = match v.as_slice() {
                [false, false, false, false] => ' ',
                [false, false, false, true] => '╷',
                [false, false, true, false] => '╵',
                [false, false, true, true] => '│',
                [false, true, false, false] => '╴',
                [false, true, false, true] => '┐',
                [false, true, true, false] => '┘',
                [false, true, true, true] => '┤',
                [true, false, false, false] => '╶',
                [true, false, false, true] => '┌',
                [true, false, true, false] => '└',
                [true, false, true, true] => '├',
                [true, true, false, false] => '─',
                [true, true, false, true] => '┬',
                [true, true, true, false] => '┴',
                [true, true, true, true] => '┼',
                _ => unreachable!(),
            };
            print!("{}", c);
            if count % self.size.x == self.size.x - 1 {
                print!("\n");
            }

            count += 1;
        }
    }
}

struct Neighbor {
    pub from: usize,
    pub to: Uvec2,
}

#[derive(Clone)]
struct CellNode {
    coordinates: Uvec2,
    edges: Vec<Uvec2>,
}

impl CellNode {
    pub fn new(coordinates: Uvec2) -> Self {
        Self { coordinates: coordinates, edges: vec![] }
    }

    pub fn neighbors(&self, size: Uvec2) -> Vec<Uvec2> {
        let mut neighbors = vec![];
        for d in Direction::ALL {
            match self.coordinates.add_delta(d, &size) {
                Option::None => continue,
                Option::Some(pos) => neighbors.push(pos),
            }
        }
        neighbors
    }

    pub fn add_edge(&mut self, to: Uvec2) {
        self.edges.push(to);
    }

    pub fn has_edge(&self, to: &Uvec2) -> bool {
        self.edges.iter().any(|edge| edge == to)
    }

    pub fn get_coordinates(&self) -> Uvec2 {
        self.coordinates
    }
}
