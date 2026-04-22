use crate::{snake::{BodySegment, Snake}, utility::{Direction, Uvec2}};
use std::collections::{HashSet};

use rand::{thread_rng, prelude::IteratorRandom};
use ratatui::{
    buffer::{Cell, Buffer}, layout::{Rect}, widgets::Widget, style::{Color, Style}
};

pub struct World {
    size: Uvec2,
    frame: Vec<Cell>,
    color_snake: Color,
    color_food: Color,
    n_steps: u32,
    n_parts: u32,
    empty: HashSet<Uvec2>,
    snake: Snake,
    food_coordinates: Uvec2,
}

impl Widget for &World {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let width = self.size.x;
        let height = self.size.y;

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let cell: &Cell = &self.frame[idx];

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

        // ================================================ Build Frame ================================================
        let mut frame : Vec<Cell> = vec![Cell::default(); size.x * size.y];
        // =============================================================================================================

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
        let facing: Direction = Direction::Right;
        let snake: Snake = Snake::new(start, n_parts, size, facing);
        // add snake position to data structures
        empty.remove(&start);
        let start_index: usize = start.y * size.x + start.x;
        frame[start_index].set_char(Direction::get_connection(Direction::opposite(facing), facing));
        frame[start_index].set_style(Style::default().fg(color_snake));
        // =============================================================================================================

        // ================================================= Set Food ==================================================
        let food_coordinates: Uvec2 = Self::find_random_empty(&empty);
        let food_index: usize = food_coordinates.y * size.x + food_coordinates.x;
        frame[food_index].set_char(Self::get_food_symbol());
        frame[food_index].set_style(Style::default().fg(color_food));
        // =============================================================================================================

        Self {
            size: size,
            frame: frame,
            color_snake: color_snake, 
            color_food: color_food,
            n_steps: 0, 
            n_parts: n_parts as u32, 
            empty: empty,
            snake: snake, 
            food_coordinates: food_coordinates 
        }
    }

    pub fn get_size(&self) -> Uvec2 {
        self.size
    }

    pub fn simulation_step(&mut self) {
        self.n_steps += 1;
        // =============================================== Update Snake ================================================
        let (clear_option, new_pos) = self.snake.step(self.food_coordinates);
        // =============================================================================================================

        // ================================== Handle Collision & Update Empty, Frame ===================================
        if let Some(pos) = clear_option && self.n_steps >= self.n_parts {
            self.empty.insert(pos);
            self.update_frame(pos, ' ', Color::Reset);
        }
        if !self.empty.contains(&new_pos) {
            panic!("Collision!");
        }
        self.empty.remove(&new_pos);
        let head: &BodySegment = self.snake.peek_head();
        let c1: char = Direction::get_connection(head.get_from(),head.get_to());
        self.update_frame(new_pos, c1, self.color_snake);
        if let Some(second) = self.snake.peek_second() {
            let c2: char = Direction::get_connection(second.get_from(),second.get_to());
            self.update_frame(second.get_coordinates(), c2, self.color_snake);
        }
        // =============================================================================================================

        // ================================================ Handle Food ================================================
        if new_pos == self.food_coordinates {
            self.food_coordinates = Self::find_random_empty(&self.empty);
            self.update_frame(self.food_coordinates, Self::get_food_symbol(), self.color_food);
        }
        // =============================================================================================================
    }

    fn update_frame(&mut self, pos: Uvec2, c: char, color: Color) {
        let index: usize = pos.y * self.size.x + pos.x;
        self.frame[index].set_char(c);
        self.frame[index].set_style(Style::default().fg(color));
    }

    fn find_random_empty(empty: &HashSet<Uvec2>) -> Uvec2 {
        let mut rng = thread_rng();
        *empty.iter().choose(&mut rng).expect("No empty cells")
    }

    fn get_food_symbol() -> char {
        '\u{25CF}' // ●
    }
}
