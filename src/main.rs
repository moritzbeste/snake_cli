use std::{io, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    Terminal, backend::CrosstermBackend, style::Color
};
mod utility;
mod world;
mod hamilton;
mod snake;
use utility::Uvec2;
use world::World;
use snake::Snake;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let size = terminal.size()?;

    // ============================================== Build Frame ==============================================
    let width: usize = if size.width % 2 == 0 { size.width  as usize} else { (size.width - 1) as usize};
    let height: usize = if size.height % 2 == 0 { size.height  as usize} else { (size.height - 1) as usize};
    assert!(width >= 4, "width too small!");
    assert!(height >= 4, "height too small!");
    let color_snake: Color = Color::Green;
    let color_food: Color = Color::Yellow;
    let mut world: World = World::new(width, height, color_snake, color_food);
    // =========================================================================================================

    // ============================================== Build Snake ==============================================
    let n_parts: usize = 3;
    if n_parts < 1 { panic!("Initial Snake Length too short!"); }
    let start_x: usize = 1;
    let start: Uvec2 = Uvec2 { x: start_x, y: world.get_size().y / 2 };
    let mut snake: Snake = Snake::new(start, n_parts, &mut world);
    // =========================================================================================================
    
    // =============================================== Game Loop ===============================================
    loop {
        // draw frame
        terminal.draw(|f| {
            let size = f.area();
            f.render_widget(&world, size);
        })?;

        // update snake
        // TODO

        // handle input
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }
    // =========================================================================================================

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
