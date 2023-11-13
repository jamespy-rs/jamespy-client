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
fn draw_events(f: &mut Frame, app: &mut App, area: Rect) {
    let messages = MESSAGES.lock().unwrap();
    let text: Vec<Line<'_>> = messages
        .iter()
        .flat_map(|lines| lines.iter())
        .cloned()
        .collect();

    let mut new_text: Vec<Line<'_>> = Vec::new();
    let max_width = area.width - 2;

    for line in text.clone() {
        if line.width() <= max_width.into() {
            new_text.push(line)
        } else {
            let funky = split_line(line.spans, max_width);
            for fun in funky {
                new_text.push(fun)
            }
        }
    }

    app.vertical_scroll_state = app.vertical_scroll_state.content_length(new_text.len());

    let logs_height = area.height - 2; // border.

    app.vertical_scroll = if new_text.len() > logs_height as usize {
        new_text.len() - logs_height as usize
    } else {
        0
    };

    app.vertical_scroll_state.position(new_text.len());

    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Events",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));

    let paragraph = Paragraph::new(new_text)
        .block(block)
        .scroll((app.vertical_scroll as u16, 0));

    f.render_widget(paragraph, area);
}

fn split_line(spans: Vec<Span<'_>>, max_width: u16) -> Vec<Line> {
    let mut lines: Vec<Line<'_>> = Vec::new();
    // This is ONE message.
    // it is just too long to display without split.
    for span in spans {
        let chunks: Vec<String> = span.content
        .chars()
        .collect::<Vec<char>>()
        .chunks(max_width.into())
        .map(|chunk| chunk.iter().collect())
        .collect();
        // the span split only after words, not on words, or its just splitting when it shouldn't be.

        for chunk in chunks {
            let line = Line::from(Span::styled(chunk, span.style));
            lines.push(line)

        }
    }    
    lines
}
//
/*
Line::from(vec![
    Span::styled(span.content, span.style)
]); */

fn split_string(input: &str, limit: usize) -> Vec<String> {
    input.chars().collect::<Vec<_>>()
        .chunks(limit)
        .map(|chunk| chunk.iter().collect())
        .collect()
}
