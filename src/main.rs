use reqwest::Client;
use teloxide::prelude::*;

use pi_trending::trend::{ trend_fn };

#[tokio::main]
async fn main(){

    dotenvy::dotenv().ok();

    let bot = Bot::from_env(); // Uses TELOXIDE_TOKEN
    let client = Client::new();
    let url = "https://api.testnet.minepi.com/assets?limit=50&order=desc";
    
    let res  = trend_fn(&bot, &client, &url).await.unwrap();
    eprintln!("{:?}", res);
}
