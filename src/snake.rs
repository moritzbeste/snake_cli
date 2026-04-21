use std::collections::VecDeque;

use crate::hamilton::Hamilton;
use crate::utility::{Uvec2, Direction};
use crate::world::{World};

pub struct Snake {
    body: VecDeque<BodyPart>,
    facing: Direction,
    hamilton: Hamilton,
}

impl Snake {
    pub fn new(start: Uvec2, n_parts: usize, size: Uvec2) -> Self {        
        // ============================================= Build Snake ===============================================
        let mut body: VecDeque<BodyPart> = VecDeque::new();
        let facing: Direction = Direction::Right;

        for _i in 0..n_parts {
            let part: BodyPart = BodyPart::new(start);
            body.push_back(part);
        }
        // =========================================================================================================

        // ============================================ Build Hamilton =============================================
        let mut hamilton = Hamilton::new(size);
        hamilton.build();
        // =========================================================================================================

        Self { body: body, facing: facing, hamilton }
    }

    #[allow(dead_code)]
    pub fn get_tail(&self) -> BodyPart {
        match self.body.front() {
            Option::None => panic!("Getting tail returned None!"),
            Some(f) => f.clone(),
        }
    } 

    #[allow(dead_code)]
    pub fn get_head(&self) -> BodyPart {
        match self.body.back() {
            Option::None => panic!("Getting head returned None!"),
            Some(f) => f.clone(),
        }
    } 
}

#[derive(Clone)]
pub struct BodyPart {
    coordinates: Uvec2,
}

impl BodyPart {
    pub fn new(coordinates: Uvec2) -> Self {
        Self { coordinates: coordinates } 
    }
}


