use std::{io, thread, time::Duration};

use tui::{
    backend::{Backend,CrosstermBackend},
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Frame,
    Terminal
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> Result<(), io::Error> {

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(ui)?;

    thread::sleep(Duration::from_millis(5000));

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor();

    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(60),
                Constraint::Percentage(40)
            ].as_ref()
        )
        .split(f.size());

    let single_request_menu = Block::default()
        .title("REQUEST")
        .borders(Borders::ALL);

    f.render_widget(single_request_menu, chunks[0]);

    let request_log_menu = Block::default()
        .title("LOG")
        .borders(Borders::ALL);
    
    f.render_widget(request_log_menu, chunks[1]);
}
