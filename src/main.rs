use std::{io, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen},
    cursor::{Hide, Show},
    style::{Color},
};

mod utility;
mod world;
mod hamilton;
mod snake;
use world::World;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, Hide)?;
    execute!(stdout, EnterAlternateScreen)?;

    let (w, h) = size()?;

    // ============================================ Hyperparameters ============================================
    let sleep_ms: u64 = 5;
    // =========================================================================================================

    // ============================================== Build World ==============================================
    let width:  usize = if w  % 2 == 0 { w  as usize}  else { (w - 1) as usize};
    let height: usize = if h % 2 == 0 { h  as usize} else { (h - 1) as usize};
    assert!(width  >= 2, "width too small!");
    assert!(height >= 2, "height too small!");
    let n_parts: usize = 3;
    let color_snake: Color = Color::Green;
    let color_food:  Color = Color::Yellow;
    let mut world:   World = World::new(width, height, n_parts, color_snake, color_food);
    // =========================================================================================================
    
    // =============================================== Game Loop ===============================================
    loop {
        // simulation step
        world.simulation_step();
        //thread::sleep(Duration::from_millis(10));

        // handle input
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        // sleep
        std::thread::sleep(Duration::from_millis(sleep_ms));
    }
    // =========================================================================================================

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    execute!(stdout, Show)?;
    Ok(())
}
