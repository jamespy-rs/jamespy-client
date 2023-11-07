use std::{
    error::Error,
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use tokio::runtime;

use crate::{app::App, ui};
use std::sync::mpsc::Receiver;

pub fn run(tick_rate: Duration, receiver: Receiver<String>) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    thread::spawn(move || {
        let rt = runtime::Runtime::new().expect("Failed to create Tokio runtime.");
        rt.block_on(process_messages(receiver));
    });

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new("jamespy client (ip-address)");
    let res = run_app(&mut terminal, app, tick_rate);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Mouse(key) = event::read()? {
                match key.kind {
                    MouseEventKind::ScrollUp => todo!(),
                    MouseEventKind::ScrollDown => todo!(),
                    _ => {}
                }
            }
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(c) => app.on_key(c),
                        KeyCode::Left => app.on_left(),
                        KeyCode::Right => app.on_right(),
                        _ => {}
                    }
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
        if app.should_quit {
            return Ok(());
        }
    }
}

use crate::app::MESSAGES;

async fn process_messages(receiver: mpsc::Receiver<String>) {
    while let Ok(message) = receiver.recv() {
        let mut handle = MESSAGES.lock().unwrap();
        handle.push(message)
        // how do I get this over to where it draws?
        // I just want to add messages to app.messages...
    }
}
