use teloxide::prelude::*;
use serde_json::Value;
use crate::trend::escape_html;

pub async fn update_pinned_message(
    bot: &Bot,
    tg_channel: &str,
    records: &[Value],
) -> Result<(), Box<dyn std::error::Error>> {
    let channel_x = tg_channel.to_string();

    let chat = bot.get_chat(channel_x.clone()).await?;
    
    let pinned = if let Some(msg) = chat.pinned_message {
        msg
    } else {
        let text = build_pinned_text(records)?;
        let sent = bot.send_message(channel_x.clone(), text)
            .parse_mode(teloxide::types::ParseMode::Html)
            .await?;

        bot.pin_chat_message(channel_x.clone(), sent.id).await?;
        return Ok(());
    };

    let pinned_id = pinned.id;
    let new_text = build_pinned_text(records)?;

    if let Some(old_text) = pinned.text() {
        if old_text == new_text {
            return Ok(()); // no changes
        }
    }

    let _ = bot.edit_message_text(channel_x.clone(), pinned_id, new_text)
        .parse_mode(teloxide::types::ParseMode::Html)
        .await;

    Ok(())
}

fn build_pinned_text(records: &[Value]) -> Result<String, Box<dyn std::error::Error>> {
    let mut out = String::from("📌 <b>PI NETWORK TOKEN LIST</b>\n\n");

    for (i, token) in records.iter().enumerate() {
        let code = token["asset_code"].as_str().unwrap_or("N/A");
        let issuer = token["asset_issuer"].as_str().unwrap_or("N/A");

        out.push_str(&format!(
            "{}. <b>{}</b>\n   CA: <code>{}</code>\n\n",
            i + 1,
            escape_html(code),
            escape_html(issuer),
        ));
    }

    out.push_str("updated automatically");

    Ok(out)
}
