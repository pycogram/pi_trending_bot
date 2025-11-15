use reqwest::Client;
use teloxide::Bot;

use pi_trending::trend::{ trend_fn };

#[tokio::main]
async fn main(){

    dotenvy::dotenv().ok();

    let bot = Bot::from_env(); // Uses TELOXIDE_TOKEN
    let client = Client::new();
    let url = "https://api.testnet.minepi.com/assets?limit=10&order=desc";
    let tg_channel = "@pi_trending";
    
    let res  = trend_fn(&bot, &client, &url, &tg_channel).await.unwrap();
    eprintln!("{:?}", res);

}
