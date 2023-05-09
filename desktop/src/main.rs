use aery_core::{Client, Match, Summoner};
use futures::future;
use futures::stream::{self, StreamExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = dotenv::var("RGAPI_KEY")?;
    let client = Client::new(api_key);

    let summoner = Summoner::from_name(&client, "synxtrak").await?;

    println!("Summoner: {}", summoner.name());

    println!("Matches (0..10):");
    stream::iter(summoner.matches(&client, 0..10).await?)
        .map(|id| Match::from_id(&client, id))
        .buffered(10)
        .filter_map(|game| future::ready(game.ok()))
        .for_each(|game| {
            let player = game
                .participant(summoner.puuid())
                .expect("Participant missing");

            let result = if player.win { "Win" } else { "Lose" };

            println!(
                "[{}] {}: {}/{}/{}",
                result, player.champion_name, player.kills, player.deaths, player.assists
            );

            future::ready(())
        })
        .await;

    Ok(())
}
