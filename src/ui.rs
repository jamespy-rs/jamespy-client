use ratatui::{prelude::*, widgets::*};

use crate::app::{App, MESSAGES};

pub fn draw(f: &mut Frame, app: &mut App) {
    // a lot of the customisation code is bad, but will improve.
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.size());
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| text::Line::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();

    if app.show_tabs {
        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title(app.title))
            .highlight_style(Style::default().fg(Color::Yellow))
            .select(app.tabs.index);
        f.render_widget(tabs, chunks[0]);
    }

    // so it shuts up for now.
    // improve this code later.
    #[allow(clippy::single_match)]
    match app.tabs.index {
        0 => {
            if app.show_tabs {
                draw_first_tab(f, app, chunks[1]);
            } else {
                draw_first_tab(f, app, f.size());
            }
        }
        _ => {}
    }
}

fn draw_first_tab(f: &mut Frame, app: &mut App, area: Rect) {
    draw_events(f, app, area);
}

// the events on the first page.
// I wonder if I can make it so it only redraws messages
// if they have actually changed or the window has updated?
fn draw_events(f: &mut Frame, app: &mut App, area: Rect, ) {
    let messages = MESSAGES.lock().unwrap();
    let text: Vec<Line<'_>> = messages
        .iter()
        .flat_map(|lines| lines.iter())
        .cloned()
        .collect();

    let border_width: u16 = if app.logs_border { 2 } else { 0 };
    let mut new_text: Vec<Line<'_>> = Vec::new();
    let max_width = area.width - border_width;

    for line in text.clone() {
        if line.width() <= max_width.into() {
            new_text.push(line)
        } else {
            let funky = split_line(line.spans, max_width.into());
            for fun in funky {
                new_text.push(fun)
            }
        }
    }

    app.vertical_scroll_state = app.vertical_scroll_state.content_length(new_text.len());

    let logs_height = area.height - border_width;

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

    let mut paragraph = Paragraph::new(new_text).scroll((app.vertical_scroll as u16, 0));

    if app.logs_border {
        paragraph = paragraph.block(block);
    }

    f.render_widget(paragraph, area);
}

// I should probably try and optimise this.
fn split_line(spans: Vec<Span<'_>>, max_width: usize) -> Vec<Line<'_>> {
    let mut lines: Vec<Line<'_>> = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;

    let mut spans_to_push: Vec<Span> = Vec::new();

    for span in spans.clone() {
        let span_chars: Vec<char> = span.content.chars().collect();

        for &c in span_chars.iter() {
            if current_width < max_width {
                current_line.push(c);
                current_width += 1;
            } else {
                // line is full so clear it.
                let span = Span::styled(current_line.clone(), span.style);
                spans_to_push.push(span);
                lines.push(Line::from(spans_to_push.clone()));
                current_line.clear();
                spans_to_push.clear();
                // add the character to the line.
                current_line.push(c);
                current_width = 1;
            }
        }
        if !current_line.is_empty() {
            // is not empty, but isn't full either.
            // We have reached the end of the span.
            let span = Span::styled(current_line.clone(), span.style);
            current_line.clear();
            spans_to_push.push(span);
        }
    }
    let line = Line::from(spans_to_push);
    lines.push(line);

    lines
}
