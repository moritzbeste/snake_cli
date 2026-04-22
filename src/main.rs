use std::{io, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
    cursor::{Hide, Show},
    style::{Color},
};

mod utility;
mod world;
mod hamilton;
mod snake;
use world::{World, GameState};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, Hide)?;
    execute!(stdout, EnterAlternateScreen)?;
    execute!(stdout, Clear(ClearType::All)).unwrap();

    let (w, h) = size()?;

    // ============================================ Hyperparameters ============================================
    let sleep_ms: u64 = 1;
    let n_parts: usize = 3;
    let color_snake: Color = Color::Green;
    let color_food:  Color = Color::Yellow;
    let color_reset: Color = Color::Reset;
    let auto_replay: bool = false;
    // =========================================================================================================

    // ============================================== Build World ==============================================
    let width:  usize = if w  % 2 == 0 { w  as usize}  else { (w - 1) as usize};
    let height: usize = if h % 2 == 0 { h  as usize} else { (h - 1) as usize};
    assert!(width  >= 2, "width too small!");
    assert!(height >= 2, "height too small!");
    let mut world: World = World::new(width, height, n_parts, color_snake, color_food, color_reset);
    // =========================================================================================================
    
    // =============================================== Game Loop ===============================================
    let final_state: GameState;
    loop {
        // simulation step
        let state: GameState = world.simulation_step();
        if state != GameState::Running {
            execute!(stdout, Clear(ClearType::All)).unwrap();
            if auto_replay {
                world = World::new(width, height, n_parts, color_snake, color_food, color_reset);
            }
            else {
                final_state = state;
                break;
            }
        }

        // handle input
        if event::poll(Duration::from_millis(sleep_ms))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    final_state = GameState::Interrupt;
                    break;
                }
            }
        }
    }
    // =========================================================================================================

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    execute!(stdout, Show)?;
    println!("Game ended by {:?}.", final_state);
    Ok(())
}
