use crate::utility::Uvec2;

use ratatui::{
    buffer::{Cell, Buffer}, layout::{Rect}, widgets::Widget, style::{Color, Style}
};

pub struct World {
    size: Uvec2,
    frame: Frame,
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
            size: Uvec2 { x: width - 2, y: height - 2 },
            frame: Frame::new(width, height),
            game_world: GameWorld::new(width - 2, height - 2),
            color_snake: color_snake, 
            color_food: color_food,
        }
    }

    pub fn write(&mut self, pos: Uvec2, c: char, element: Element) {
        let color = match element {
            Element::Snake => self.color_snake,
            Element::Food => self.color_food,
            Element::Empty => Color::Black,
        };
        self.frame.write_frame(pos, c, color);
        self.game_world.set(pos, element);
    }

    pub fn get_size(&self) -> Uvec2 {
        self.size
    }
}

struct Frame {
    frame_size: Uvec2,
    frame: Vec<Cell>,
}


impl Frame {
    pub fn new(width: usize, height: usize) -> Self {
        Self { 
            frame_size: Uvec2 { x: width, y: height }, 
            frame: Self::build_frame(width, height), 
        }
    }

    pub fn write_frame(&mut self, pos: Uvec2, c: char, color: Color) {
        let idx: usize = (pos.y + 1) * self.frame_size.x + pos.x + 1;
        let cell = &mut self.frame[idx];
        cell.set_char(c);
        cell.set_style(Style::default().fg(color));
    } 

    fn build_frame(width: usize, height: usize) -> Vec<Cell> {
        let mut frame : Vec<Cell> = vec![Cell::default(); width * height];
        // putting in corners
        frame[0].set_char('\u{2554}');                    // ╔
        frame[width - 1].set_char('\u{2557}');            // ╗
        frame[(height - 1) * width].set_char('\u{255A}'); // ╚
        frame[height * width - 1].set_char('\u{255D}');   // ╝
        
        // iterate column wise
        for i in 1..height - 1 {
            // left edge
            frame[i * width].set_char('\u{2551}');           // ║
            // right edge
            frame[(i + 1) * width - 1].set_char('\u{2551}'); // ║
        }
        // iterate row wise
        let offset = (height - 1) * width;
        for i in 1..width - 1 {
            // top edge
            frame[i].set_char('\u{2550}'); // ═
            // bottom edge
            frame[offset + i].set_char('\u{2550}'); // ═
        }
        frame
    }
}

struct GameWorld {
    width: usize,
    board: Vec<Element>,
    food_coordinates: Uvec2,
    empty_cells: Vec<Uvec2>,
}

impl GameWorld {
    pub fn new(width: usize, height: usize) -> Self {
        let board: Vec<Element> = vec![Element::Empty; width * height];
        Self { width: width, board: board }
    }

    pub fn get(&self, pos: Uvec2) -> Element {
        self.board[pos.y * self.width + pos.x]
    }

    pub fn set(&mut self, pos: Uvec2, element: Element) {
        self.board[pos.y * self.width + pos.x] = element;
    }

    pub fn find_random_empty() -> Uvec2 {
        todo!();
    }
}

#[derive(Clone, Copy)]
pub enum Element {
    Empty,
    Snake,
    Food,
}