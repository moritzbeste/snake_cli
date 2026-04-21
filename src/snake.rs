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
        // ============================================= Build Snake ===============================================
        let mut body: VecDeque<BodySegment> = VecDeque::new();
        let facing: Direction = Direction::Right;

        for _i in 0..n_parts {
            let part: BodySegment = BodySegment::new(start, Direction::opposite(facing), facing);
            body.push_back(part);
        }
        // =========================================================================================================

        // ============================================ Build Hamilton =============================================
        let mut hamilton = Hamilton::new(size);
        hamilton.build();
        // =========================================================================================================

        Self { size: size, body: body, hamilton: hamilton }
    }

    pub fn peek_head(&self) -> &BodySegment {
        self.body.front().unwrap()
    } 

    pub fn peek_tail(&self) -> &BodySegment {
        self.body.back().unwrap()
    } 

    pub fn get_body(&self) -> &VecDeque<BodySegment> {
        &self.body
    }

    pub fn step(&mut self, food_coordinates: Uvec2) -> (Option<Uvec2>, Uvec2) {
        let (head_pos, head_to) = {
            let head = self.peek_head();
            (head.get_coordinates(), head.get_to())
        };
        let path: Direction = self.find_path(head_pos, head_to, self.hamilton.get_member(food_coordinates));
        let new_pos: Uvec2 = head_pos.add_delta(path, &self.size).unwrap();
        {
            let head = self.body.front_mut().unwrap();
            head.set_to(path);
        }
        if new_pos == food_coordinates {
            let growth: BodySegment = BodySegment::new(new_pos, Direction::opposite(path), path); 
            self.body.push_front(growth);
            return (Option::None, new_pos);
        }
        else {
            let mut segment: BodySegment = self.body.pop_back().unwrap();
            let clear_pos: Uvec2 = segment.get_coordinates();
            segment.set_coordinates(new_pos);
            segment.set_from(Direction::opposite(path));
            segment.set_to(path);
            self.body.push_front(segment);

            return (Option::Some(clear_pos), new_pos);
        }
    }

    fn find_path(&self, head_pos: Uvec2, head_to: Direction, food_index: usize) -> Direction {
        let head_index: usize = self.hamilton.get_member(head_pos);
        let tail_index: usize = self.hamilton.get_member(self.peek_tail().get_coordinates());
        let len_board: usize = self.size.x * self.size.y;

        let mut best_cont: Direction = Direction::opposite(head_to);
        let mut next_cont: Direction = best_cont;
        let mut best_dist: usize = self.size.x * self.size.y;
        let max_index: usize =  best_dist - 1;
        for d in Direction::ALL {
            if d == Direction::opposite(head_to) { continue; }

            let curr_index: usize = match head_pos.add_delta(d, &self.size) {
                Option::None => continue,
                Option::Some(v) => self.hamilton.get_member(v),
            };

            if curr_index == (head_index + 1) % len_board {
                next_cont = d;
            }

            if !Self::is_between(head_index, tail_index, curr_index) {
                let dist_food: usize;
                if food_index < curr_index {
                    dist_food = max_index - curr_index + food_index;
                }
                else {
                    dist_food = food_index - curr_index;
                }
                if dist_food < best_dist {
                    best_dist = dist_food;
                    best_cont = d;
                }
            }
        }
        if best_cont == Direction::opposite(head_to) { next_cont }
        else { best_cont }
    }

    pub fn is_between(bound1: usize, bound2: usize, index: usize) -> bool {
        if bound1 > bound2 {
            index >= bound2 && index <= bound1
        }
        else {
            index >= bound2 || index <= bound1
        }
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


