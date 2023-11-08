use crate::{app::MESSAGES, Error};
use ratatui::prelude::*;
use serenity::all::Message;

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
    let mut handle = MESSAGES.lock().unwrap();
    handle.push(msg);
    Ok(())
}
