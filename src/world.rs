use crate::{snake::Snake, utility::Uvec2};
use std::collections::VecDeque;

use ratatui::{
    buffer::{Cell, Buffer}, layout::{Rect}, widgets::Widget, style::{Color, Style}
};

pub struct World {
    size: Uvec2,
    game_world: GameWorld,
    color_snake: Color,
    color_food: Color,
}

impl Widget for &World {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let width = self.frame.frame_size.x;
        let height = self.frame.frame_size.y;

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let cell = &self.frame.frame[idx];

                if let Some(dst) = buf.cell_mut((area.x + x as u16, area.y + y as u16)) {
                    *dst = cell.clone();
                }
            }
        }
    }
}

impl World {
    pub fn new(width: usize, height: usize, color_snake: Color, color_food: Color) -> Self {
        Self {
            size: Uvec2 { x: width, y: height },
            game_world: GameWorld::new(size),
            color_snake: color_snake, 
            color_food: color_food,
        }
    }

    pub fn get_size(&self) -> Uvec2 {
        self.size
    }

    fn construct_frame(&self) -> Vec<Cell> {

    }
}

struct GameWorld {
    n_steps: u32,
    size: Uvec2,
    snake: Snake,
    food_coordinates: Uvec2,
    empty_cells: VecDeque<Uvec2>,
}

impl GameWorld {
    pub fn new(size: Uvec2) -> Self {
        // ============================================== Build Snake ==============================================
        let n_parts: usize = 3;
        if n_parts < 1 { panic!("Initial Snake Length too short!"); }
        let start_x: usize = 1;
        let start: Uvec2 = Uvec2 { x: start_x, y: size.y / 2 };
        let mut snake: Snake = Snake::new(start, n_parts, size);
        // =========================================================================================================

        // ============================================== Build Empty ==============================================
        let empty_cells: VecDeque<Uvec2> = VecDeque::new();
        // =========================================================================================================
    }

    pub fn find_random_empty() -> Uvec2 {
        todo!();
    }
}