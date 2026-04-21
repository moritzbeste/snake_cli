use std::ops::Add;

#[derive(Debug, Copy, Clone)]
pub struct Ivec2 {
    pub x: i32,
    pub y: i32,
}

impl Add for Ivec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Uvec2 {
    pub x: usize,
    pub y: usize,
}

impl Add for Uvec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Uvec2 {
    pub fn add_delta(&self, delta: Direction, size: &Uvec2) -> Option<Uvec2> {
        let d = delta.to_vec();
        let x = self.x as i32 + d.x;
        let y: i32 = self.y as i32 + d.y;
        if x < 0 || x >= size.x as i32 || y < 0 || y >= size.y as i32 {
            return None;
        }
        Some(Uvec2 {
            x: x as usize,
            y: y as usize, 
        })
    }

    pub fn equals(one: &Self, two: &Self) -> bool {
        one.x == two.x && one.y == two.y
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    pub const ALL: [Direction; 4] = [
        Direction::Right,
        Direction::Left,
        Direction::Up,
        Direction::Down,
    ];

    pub fn to_vec(self) -> Ivec2 {
        match self {
            Direction::Right => Ivec2 { x:  1, y:  0},
            Direction::Left  => Ivec2 { x: -1, y:  0}, 
            Direction::Up    => Ivec2 { x:  0, y: -1},
            Direction::Down  => Ivec2 { x:  0, y:  1}, 
        }
    }

    pub fn right_of(current: Self) -> Self {
        match current {
            Direction::Right => Direction::Down,
            Direction::Left  => Direction::Up, 
            Direction::Up    => Direction::Right,
            Direction::Down  => Direction::Left, 
        }
    }

    pub fn left_of(current: Self) -> Self {
        match current {
            Direction::Right => Direction::Up,
            Direction::Left  => Direction::Down, 
            Direction::Up    => Direction::Left,
            Direction::Down  => Direction::Right, 
        }
    }

    pub fn identity(current: Self) -> Self {
        current
    }

    pub fn get_connection(from: Self, to: Self) -> char {
        let mask = Self::bit(from) | Self::bit(to);
        match mask {
            0b1100 | 0b1000 | 0b0100 => '\u{2501}', // ━
            0b0011 | 0b0010 | 0b0001 => '\u{2503}', // ┃
            0b1001                   => '\u{250F}', // ┏
            0b0110                   => '\u{251B}', // ┛
            0b0101                   => '\u{2513}', // ┓
            0b1010                   => '\u{2517}', // ┗
            _ => panic!("Unknown Connection!"),
        }
    }

    fn bit(dir: Self) -> u8 {
        match dir {
            Direction::Right => 0b1000,
            Direction::Left => 0b0100,
            Direction::Up => 0b0010,
            Direction::Down => 0b0001,
        }
    }
}
