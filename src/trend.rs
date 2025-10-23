use reqwest::Client;
use serde_json::Value;
use rand::{seq::SliceRandom, Rng};
use tokio::time::{sleep, Duration};
use teloxide::prelude::*;


pub async fn trend_fn(bot: &Bot, client: &Client, url: &'static str)  -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let resp = client.get(url).send().await?.text().await?;
        let json: Value = serde_json::from_str(&resp)?;
        //println!("{json:?}");

        if let Some(records) = json["_embedded"]["records"].as_array() {
            let mut rng = rand::thread_rng();

            if let Some(random_token) = records.choose(&mut rng) {
                let code = escape_html(random_token["asset_code"].as_str().unwrap_or("N/A"));
                let issuer = escape_html(random_token["asset_issuer"].as_str().unwrap_or("N/A"));

                // Random emoji line
                let emoji_line = "🟠".repeat(rng.gen_range(3..=10));

                // HTML message
                let msg = format!(
                    "🚀 <b>PI TRENDING</b>\n\
                    <b>{}</b> <i>Buy!</i>\n\
                    {}\n\n\
                    💰 Name: <b>{}</b>\n\
                    ⛓️‍💥 CA: <i><code>{}</code></i>\n\n\
                    💸 Market Cap: <b>$0.0</b>\n\
                    ~ ",
                    code, emoji_line, code, issuer
                );

                // Send to channel
                bot.send_message("@pi_trending".to_string(), msg)
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await?;
            }
        }

        sleep(Duration::from_secs(10)).await; // check every 10 seconds
    }
}

// Escape <, >, & to avoid Telegram HTML parse errors
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
}