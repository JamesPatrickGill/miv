use std::io;

use clap::Parser;
use crossterm::cursor::SetCursorStyle;
use crossterm::execute;
use log::info;
use miv::app::{App, AppResult, InputMode};
use miv::cli::Cli;
use miv::event::Event;
use miv::input_handling::InputStack;
use miv::tui::Tui;
use miv::utils::initialize_logging;

fn main() -> AppResult<()> {
    // Start logging
    initialize_logging()?;
    info!("Starting miv");

    // Parse cli args
    let args = Cli::parse();

    // Stack for input handling
    let mut input_stack = InputStack::new();

    // Create an application.
    let mut app = App::new(args.filename);
    let mut tui = Tui::new()?;
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        let commands = match tui.events.next()? {
            Event::Tick => {
                app.tick();
                vec![]
            }
            Event::Key(key_event) => input_stack
                .handle_key_event(key_event, &app.mode)
                .unwrap_or_default(),
            Event::Mouse(_) => vec![],
            Event::Resize(_, _) => vec![],
        };

        app.execute(commands)?;

        match app.mode {
            InputMode::Normal => {
                execute!(io::stdout(), SetCursorStyle::BlinkingBlock).unwrap();
            }
            InputMode::Insert | InputMode::Command => {
                execute!(io::stdout(), SetCursorStyle::BlinkingBar).unwrap();
            }
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
