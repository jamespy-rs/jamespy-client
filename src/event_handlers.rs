use crate::{app::MESSAGES, Error};
use ratatui::prelude::*;
use serenity::all::{Message, MessageUpdateEvent};

pub async fn new_message(
    message: Message,
    guild_name: String,
    channel_name: String,
) -> Result<(), Box<dyn Error>> {
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

    let mut lines: Vec<text::Line> = Vec::new();
    let total_lines = message.content.lines().count();

    for (index, line) in message.content.lines().enumerate() {
        let mut formatted_line = vec![Span::raw(format!("{}", line))];

        if index == 0 {
            formatted_line.insert(
                0,
                Span::styled(
                    format!("[{}] [#{}] ", guild_name, channel_name),
                    Style::default().fg(Color::DarkGray),
                ),
            );
            formatted_line.insert(1, Span::from(format!("{}: ", message.author.name)))
        }

        if index == total_lines - 1 {
            formatted_line.push(Span::styled(
                format!(
                    "{}{}",
                    attachments_fmt.as_deref().unwrap_or(""),
                    embeds_fmt.as_deref().unwrap_or("")
                ),
                Style::default().fg(Color::Cyan),
            ));
        }

        lines.push(text::Line::from(formatted_line));
    }

    let msg = lines;

    // handle bad words
    let mut handle = MESSAGES.lock().unwrap();
    handle.push(msg);
    Ok(())
}

pub async fn message_edit(
    old_if_available: Option<Message>,
    new: Option<Message>,
    event: MessageUpdateEvent,
    guild_name: Option<String>,
    channel_name: Option<String>,
) -> Result<(), Box<dyn Error>> {
    // Currently the guild_names and channel_names will always be a value because its handled within the bot.
    // eventualy I will switch that.
    match (old_if_available, new) {
        (Some(old_message), Some(new_message)) => {
            if new_message.author.bot {
                return Ok(());
            }

            if old_message.content != new_message.content {
                let attachments = new_message.attachments.clone();
                let attachments_fmt: Option<String> = if !attachments.is_empty() {
                    let attachment_names: Vec<String> = attachments
                        .iter()
                        .map(|attachment| attachment.filename.clone())
                        .collect();
                    Some(format!(" <{}>", attachment_names.join(", ")))
                } else {
                    None
                };

                let embeds = new_message.embeds.clone();
                let embeds_fmt: Option<String> = if !embeds.is_empty() {
                    let embed_types: Vec<String> = embeds
                        .iter()
                        .map(|embed| embed.kind.clone().unwrap_or("Unknown Type".to_string()))
                        .collect();

                    Some(format!(" {{{}}}", embed_types.join(", ")))
                } else {
                    None
                };

                let old_content_lines: Vec<&str> = old_message.content.split('\n').collect();
                let new_content_lines: Vec<&str> = new_message.content.split('\n').collect();
                let mut msg = Vec::new();
                let first_line = text::Line::from(vec![
                    Span::styled(
                        format!(
                            "[{}] [#{}] A message by ",
                            guild_name.unwrap(),
                            channel_name.unwrap()
                        ),
                        Style::default().fg(Color::Cyan),
                    ),
                    Span::raw(new_message.author.name.clone()),
                    Span::styled(" was edited:\n", Style::default().fg(Color::Cyan)),
                ]);
                msg.push(first_line);

                for (index, line) in old_content_lines.iter().enumerate() {
                    let prefix = if index == 0 {
                        format!("BEFORE: {}: ", new_message.author.name)
                    } else {
                        String::new()
                    };
                    let line_span = Span::styled(
                        format!("{}{}", prefix, line),
                        Style::default().fg(Color::Cyan),
                    );
                    msg.push(text::Line::from(vec![line_span]));
                }

                for (index, line) in new_content_lines.iter().enumerate() {
                    let prefix = if index == 0 {
                        format!("AFTER: {}: ", new_message.author.name)
                    } else {
                        String::new()
                    };
                    let suffix = if index == new_content_lines.len() - 1 {
                        format!(
                            "{}{}",
                            attachments_fmt.as_deref().unwrap_or(""),
                            embeds_fmt.as_deref().unwrap_or("")
                        )
                    } else {
                        String::new() 
                    };
                    let line_span = Span::styled(
                        format!("{}{}{}", prefix, line, suffix),
                        Style::default().fg(Color::Cyan),
                    );
                    msg.push(text::Line::from(vec![line_span]));
                }

                // Maybe check old embeds and or attachments in the future, requires a big rewrite to this.
                let mut handle = MESSAGES.lock().unwrap();
                handle.push(msg);
            }
        }
        (None, None) => {
            let msg = vec![text::Line::from(vec![Span::styled(
                format!(
                    "A message (ID:{}) was edited but was not in cache",
                    event.id
                ),
                Style::default().fg(Color::Cyan),
            )])];
            let mut handle = MESSAGES.lock().unwrap();
            handle.push(msg);
        }
        _ => {}
    }
    Ok(())
}
