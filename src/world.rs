use crate::{snake::{BodySegment, Snake}, utility::{Direction, Uvec2}};
use std::collections::{HashSet};
use std::io::{stdout};
use crossterm::{
    execute,
    cursor::MoveTo,
    style::{Print, SetForegroundColor, SetBackgroundColor, ResetColor, Color},
};
use rand::{thread_rng, prelude::IteratorRandom};

pub struct World {
    size: Uvec2,
    color_snake: Color,
    color_food: Color,
    color_reset: Color,
    n_steps: u32,
    n_parts: u32,
    empty: HashSet<Uvec2>,
    snake: Snake,
    food_pos: Uvec2,
}

impl World {
    pub fn new(width: usize, height: usize, n_parts: usize, color_snake: Color, color_food: Color, color_reset: Color) -> Self {
        let size: Uvec2 = Uvec2 { x: width, y: height };

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
        // add snake position to data structures
        empty.remove(&start);
        let from: Direction = snake.peek_head().get_from();
        let to: Direction = snake.peek_head().get_to();
        let c1: char = Direction::get_connection(from, to);
        Self::draw_cell(start, c1, color_snake, color_reset);
        // =============================================================================================================

        // ================================================= Set Food ==================================================
        let food_pos: Uvec2 = Self::find_random_empty(&empty);
        let c2: char = Self::get_food_symbol();
        Self::draw_cell(food_pos, c2, color_food, color_reset);
        // =============================================================================================================

        Self {
            size: size,
            color_snake: color_snake, 
            color_food: color_food,
            color_reset: color_reset,
            n_steps: 0, 
            n_parts: n_parts as u32, 
            empty: empty,
            snake: snake, 
            food_pos: food_pos 
        }
    }

    pub fn simulation_step(&mut self) -> GameState {
        self.n_steps += 1;
        // =============================================== Update Snake ================================================
        let (clear_option, new_pos) = self.snake.step(self.food_pos);
        // =============================================================================================================

        // ================================== Handle Collision & Update Empty, Frame ===================================
        if let Some(pos) = clear_option && self.n_steps >= self.n_parts {
            self.empty.insert(pos);
            Self::draw_cell(pos, ' ', self.color_reset, self.color_reset);
        }
        
        if self.snake.get_length() == self.size.x * self.size.y { return GameState::Win; }
        if !self.empty.contains(&new_pos) { return GameState::Loss; }

        self.empty.remove(&new_pos);
        let head: &BodySegment = self.snake.peek_head();
        let c: char = Direction::get_connection(head.get_from(),head.get_to());
        Self::draw_cell(new_pos, c, self.color_snake, self.color_reset);
        // =============================================================================================================

        // ================================================ Handle Food ================================================
        if new_pos == self.food_pos {
            self.food_pos = Self::find_random_empty(&self.empty);
            Self::draw_cell(self.food_pos, Self::get_food_symbol(), self.color_food, self.color_reset);
        }
        // =============================================================================================================

        GameState::Running
    }

    fn find_random_empty(empty: &HashSet<Uvec2>) -> Uvec2 {
        let mut rng = thread_rng();
        *empty.iter().choose(&mut rng).expect("No empty cells")
    }

    fn get_food_symbol() -> char {
        '\u{25CF}' // ●
    }

    fn draw_cell(pos: Uvec2, ch: char, fg: Color, bg: Color) {
        let mut out = stdout();
        execute!(
            out,
            MoveTo(pos.x as u16, pos.y as u16),
            SetForegroundColor(fg),
            SetBackgroundColor(bg),
            Print(ch),
            ResetColor
        ).unwrap();
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GameState {
    Running,
    Win,
    Loss,
    Interrupt,
}