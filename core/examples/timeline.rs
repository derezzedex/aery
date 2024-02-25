use aery_core::{Client, Game, Queue, Summoner};
use futures::future;
use futures::stream::{self, StreamExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = dotenv::var("RGAPI_KEY")?;
    let client = Client::new(api_key);

    let summoner = Summoner::from_name(client.clone(), "synxtrak".to_string()).await?;

    println!("Summoner: {}", summoner.name());

    let range = 0..10;
    let queue = Queue::RankedSolo;
    println!("{queue:?} matches ({range:?}):");
    stream::iter(summoner.matches(&client, range, queue).await?)
        .map(|id| Game::from_id(&client, id))
        .buffered(10)
        .filter_map(|game| future::ready(game.ok()))
        .for_each(|game| {
            let player = game
                .participant(summoner.puuid())
                .expect("Participant missing");

            let result = if player.result.won() { "Win" } else { "Lose" };

            println!(
                "[{}] {}: {}/{}/{}",
                result,
                player.champion.identifier().unwrap(),
                player.stats.kills,
                player.stats.deaths,
                player.stats.assists
            );

            future::ready(())
        })
        .await;

    Ok(())
}
