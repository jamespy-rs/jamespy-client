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

use crate::{app::App, ui, WebSocketEvent};
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
        if let Ok(event) = serde_json::from_str::<WebSocketEvent>(&message) {
            match event {
                WebSocketEvent::NewMessage {
                    message,
                    guild_name,
                    channel_name,
                } => {
                    let attachments = message.attachments.clone();
                    let attachments_fmt: Option<String> = if !attachments.is_empty() {
                        let attachment_names: Vec<String> = attachments
                            .iter()
                            .map(|attachment| attachment.filename.clone())
                            .collect();
                        Some(format!(" <{}>", attachment_names.join(", ")))
                    } else {
                        None
                    };

                    let embeds = message.embeds.clone();
                    let embeds_fmt: Option<String> = if !embeds.is_empty() {
                        let embed_types: Vec<String> = embeds
                            .iter()
                            .map(|embed| embed.kind.clone().unwrap_or("Unknown Type".to_string()))
                            .collect();

                        Some(format!(" {{{}}}", embed_types.join(", ")))
                    } else {
                        None
                    };

                    let msg = text::Line::from(vec![
                        Span::styled(
                            format!("[{}] [#{}] ", guild_name, channel_name),
                            Style::default().fg(Color::DarkGray),
                        ),
                        Span::raw(format!("{}: {}", message.author.name, message.content)),
                        Span::styled(
                            format!(
                                "{}{}",
                                attachments_fmt.as_deref().unwrap_or(""),
                                embeds_fmt.as_deref().unwrap_or("")
                            ),
                            Style::default().fg(Color::Cyan),
                        ),
                    ]);
                    // handle bad words
                    // handle attachments
                    let mut handle = MESSAGES.lock().unwrap();
                    handle.push(msg)
                }
            }
        }
    }
}
