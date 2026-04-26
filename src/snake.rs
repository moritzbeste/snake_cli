use std::collections::VecDeque;

use crate::hamilton::Hamilton;
use crate::utility::{Uvec2, Direction};

pub struct Snake {
    size:     Uvec2,
    body:     VecDeque<BodySegment>,
    hamilton: Hamilton,
}

impl Snake {
    pub fn new(start: Uvec2, n_parts: usize, size: Uvec2) -> Self {        
        // ============================================ Build Hamilton =============================================
        let mut hamilton = Hamilton::new(size);
        hamilton.build();
        // =========================================================================================================

        // ============================================= Build Snake ===============================================
        let mut body: VecDeque<BodySegment> = VecDeque::new();

        let start_index = hamilton.get_member(start);
        let mut facing: Direction = Direction::Right;
        for d in Direction::ALL {
            if let Option::Some(v) = start.add_delta(d, &size) {
                let check_index = hamilton.get_member(v);
                if hamilton.is_next(start_index, check_index) {
                    facing = d;
                    break;
                }
            }
        }

        for _i in 0..n_parts {
            let part: BodySegment = BodySegment::new(start, Direction::opposite(facing), facing);
            body.push_back(part);
        }
        // =========================================================================================================

        Self { size: size, body: body, hamilton: hamilton }
    }

    pub fn step(&mut self, food_coordinates: Uvec2) -> (Option<Uvec2>, Uvec2) {
        let (head_pos, head_to) = {
            let head = self.peek_head();
            (head.get_coordinates(), head.get_to())
        };
        let new_pos: Uvec2 = head_pos.add_delta(head_to, &self.size).unwrap();
        let path: Direction = self.find_path(new_pos, head_to, self.hamilton.get_member(food_coordinates));

        self.update_snake(new_pos == food_coordinates, new_pos, path)
    }

    fn update_snake(&mut self, ate: bool, new_pos: Uvec2, path: Direction) -> (Option<Uvec2>, Uvec2) {
        let head_to: Direction = self.peek_head().get_to();
        if ate {
            // if the snake ate, a new segment is added
            let new_head: BodySegment = BodySegment::new(new_pos, Direction::opposite(head_to), path); 
            self.body.push_front(new_head);

            return (Option::None, new_pos);
        }
        else {
            // if the snake did not eat, the tail is moved to the front and is the new head
            let mut segment: BodySegment = self.body.pop_back().unwrap();
            let clear_pos: Uvec2 = segment.get_coordinates();
            segment.set_coordinates(new_pos);
            segment.set_from(Direction::opposite(head_to));
            segment.set_to(path);
            self.body.push_front(segment);

            return (Option::Some(clear_pos), new_pos);
        }
    }

    fn find_path(&self, head_pos: Uvec2, head_to: Direction, food_index: usize) -> Direction {
        let head_index: usize = self.hamilton.get_member(head_pos);
        let tail_index: usize = self.hamilton.get_member(self.peek_tail().get_coordinates());

        let mut best_cont: Direction = Direction::opposite(head_to);
        let mut next_cont: Direction = best_cont;
        let mut best_dist: usize = self.size.x * self.size.y;
        let max_index: usize =  best_dist - 1;
        for d in Direction::ALL {
            if d == Direction::opposite(head_to) { continue; }

            let check_index = match head_pos.add_delta(d, &self.size) {
                Option::Some(v) => self.hamilton.get_member(v),
                Option::None => continue,
            };

            if self.hamilton.is_next(head_index, check_index) {
                next_cont = d;
            }

            if !Hamilton::is_between(head_index, tail_index, check_index) {
                let dist_food: usize;
                if food_index < check_index {
                    dist_food = max_index - check_index + food_index;
                }
                else {
                    dist_food = food_index - check_index;
                }
                if dist_food < best_dist {
                    best_dist = dist_food;
                    best_cont = d;
                }
            }
        }
        if best_cont == Direction::opposite(head_to) {next_cont }
        else { best_cont }
    }

    pub fn peek_head(&self) -> &BodySegment {
        self.body.front().unwrap()
    } 

    pub fn get_length(&self) -> usize {
        self.body.len()
    }

    fn peek_tail(&self) -> &BodySegment {
        self.body.back().unwrap()
    } 
}

#[derive(Clone)]
pub struct BodySegment {
    coordinates: Uvec2,
    from:        Direction,
    to:          Direction,
}

impl BodySegment {
    pub fn new(coordinates: Uvec2, from: Direction, to: Direction) -> Self {
        Self { coordinates: coordinates, from: from, to: to } 
    }

    pub fn get_coordinates(&self) -> Uvec2 {
        self.coordinates
    }

    pub fn get_from(&self) -> Direction {
        self.from
    }

    pub fn get_to(&self) -> Direction {
        self.to
    }

    pub fn set_coordinates(&mut self, coordinates: Uvec2) {
        self.coordinates = coordinates;
    }

    pub fn set_from(&mut self, from: Direction) {
        self.from = from;
    }

    pub fn set_to(&mut self, to: Direction) {
        self.to = to;
    }
}
