use ratatui::{prelude::*, widgets::*};

use crate::app::{App, MESSAGES};

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.size());
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| text::Line::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);
    f.render_widget(tabs, chunks[0]);

    match app.tabs.index {
        0 => draw_first_tab(f, app, chunks[1]),
        _ => {}
    };
}

fn draw_first_tab(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .split(area);
    draw_events(f, app, chunks[0]);
}

// the events on the first page.
fn draw_events(f: &mut Frame, _app: &mut App, area: Rect) {
    // Lock the mutex to access the vector of strings
    let messages = MESSAGES.lock().unwrap();
    let text: Vec<text::Line> = messages
        .clone()
        .into_iter()
        .map(text::Line::from)
        .collect();

    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Events",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));

    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}
