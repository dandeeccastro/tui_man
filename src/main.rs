extern crate tuirealm;

use tuirealm::application::PollStrategy;
use tuirealm::{AttrValue, Attribute, Update};
use tuirealm::terminal::TerminalBridge;
use tuirealm::tui::layout::{Constraint, Direction, Layout};
use tuirealm::tui::widgets::{Block, Borders};

use std::{thread, time::Duration};

fn main() {
    let mut terminal = TerminalBridge::new().expect("Deu não meu cria");
    terminal.enter_alternate_screen();
    terminal.enable_raw_mode();

    terminal
        .raw_mut()
        .draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints(
                    [
                    Constraint::Percentage(60),
                    Constraint::Percentage(40)
                    ]
                    .as_ref()
                    )
                .split(f.size());

            f.render_widget(
                Block::default()
                .title("SINGLE REQUEST VIEW")
                .borders(Borders::ALL)
                , chunks[0]);

            f.render_widget(
                Block::default()
                .title("REQUEST LOG VIEW")
                .borders(Borders::ALL)
                , chunks[1]);
        })
    .is_ok();

    thread::sleep(Duration::from_millis(2000));

    terminal.leave_alternate_screen();
    terminal.disable_raw_mode();
    terminal.clear_screen();
}

