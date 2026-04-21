use crate::{snake::{BodySegment, Snake}, utility::{Direction, Uvec2}};
use std::collections::{HashSet, VecDeque};

use rand::{thread_rng, prelude::IteratorRandom};
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
        let frame = self.construct_frame();
        let width = self.size.x;
        let height = self.size.y;

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let cell: &Cell = &frame[idx];

                if let Some(dst) = buf.cell_mut((area.x + x as u16, area.y + y as u16)) {
                    *dst = cell.clone();
                }
            }
        }
    }
}

impl World {
    pub fn new(width: usize, height: usize, n_parts: usize, color_snake: Color, color_food: Color) -> Self {
        let size: Uvec2 = Uvec2 { x: width, y: height };
        Self {
            size: size,
            game_world: GameWorld::new(size, n_parts),
            color_snake: color_snake, 
            color_food: color_food,
        }
    }

    pub fn get_size(&self) -> Uvec2 {
        self.size
    }

    pub fn simulation_step(&mut self) {
        self.game_world.simulation_step();
    }

    fn construct_frame(&self) -> Vec<Cell> {
        let mut frame : Vec<Cell> = vec![Cell::default(); self.size.x * self.size.y];

        // ================================================ Draw Snake =================================================
        let snake: &VecDeque<BodySegment> = self.game_world.get_snake().get_body();
        for segment in snake {
            let v: Uvec2 = segment.get_coordinates();
            let index: usize = v.y * self.size.x + v.x;
            frame[index].set_char(Direction::get_connection(segment.get_from(), segment.get_to()));
            frame[index].set_style(Style::default().fg(self.color_snake));
        }
        // =============================================================================================================

        // ================================================= Draw Food =================================================
        let food: &Uvec2 = self.game_world.get_food();
        let food_index: usize = food.y * self.size.x + food.x;
        frame[food_index].set_char('\u{25CF}'); // ●
        frame[food_index].set_style(Style::default().fg(self.color_food));
        // =============================================================================================================

        frame 
    }
}

struct GameWorld {
    n_steps: u32,
    n_parts: u32,
    empty: HashSet<Uvec2>,
    snake: Snake,
    food_coordinates: Uvec2,
}

impl GameWorld {
    pub fn new(size: Uvec2, n_parts: usize) -> Self {
        // ================================================ Build Empty ================================================
        let mut empty = HashSet::new();

        for x in 0..size.x {
            for y in 0..size.y {
                empty.insert(Uvec2 { x: x, y: y });
            }
        }
        // =============================================================================================================

        // ================================================ Build Snake ================================================
        if n_parts < 1 { panic!("Initial Snake Length too short!"); }
        let start_x: usize = 1;
        let start: Uvec2 = Uvec2 { x: start_x, y: size.y / 2 };
        let snake: Snake = Snake::new(start, n_parts, size);
        empty.remove(&start);
        // =============================================================================================================

        // ================================================= Set Food ==================================================
        let food_coordinates: Uvec2 = Self::find_random_empty(&empty);
        // =============================================================================================================

        Self { 
            n_steps: 0, 
            n_parts: n_parts as u32, 
            empty: empty,
            snake: snake, 
            food_coordinates: food_coordinates 
        }
    }

    pub fn simulation_step(&mut self) {
        self.n_steps += 1;
        // =============================================== Update Snake ================================================
        let (clear_option, new_pos) = self.snake.step(self.food_coordinates);
        // =============================================================================================================

        // ============================================= Handle Collision ==============================================
        if !self.empty.contains(&new_pos) {
            panic!("Collision!");
        }
        // =============================================================================================================

        // =============================================== Update Empty ================================================
        self.empty.remove(&new_pos);
        if let Some(pos) = clear_option && self.n_steps >= self.n_parts {
            self.empty.insert(pos);
        }
        // =============================================================================================================

        // ================================================ Handle Food ================================================
        if new_pos == self.food_coordinates {
            self.food_coordinates = Self::find_random_empty(&self.empty);
        }
        // =============================================================================================================
    }

    fn find_random_empty(empty: &HashSet<Uvec2>) -> Uvec2 {
        let mut rng = thread_rng();
        *empty.iter().choose(&mut rng).expect("No empty cells")
    }

    fn get_snake(&self) -> &Snake {
        &self.snake
    }

    fn get_food(&self) -> &Uvec2 {
        &self.food_coordinates
    }
}
