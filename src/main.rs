use std::{io, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
mod utility;
mod world;
mod hamilton;
use world::World;
use hamilton::Hamilton;

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
    let mut world: World = World::new(width, height);
    // =========================================================================================================

    // ============================================ Build Hamilton =============================================
    let mut hamilton = Hamilton::new(&world);
    hamilton.build();
    // =========================================================================================================
    
    // ================================================ IO Loop ================================================
    loop {
    terminal.draw(|f| {
        let size = f.area();
        f.render_widget(&world, size);
    })?;

    if event::poll(Duration::from_millis(10))? {
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }
    // =========================================================================================================
}

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
