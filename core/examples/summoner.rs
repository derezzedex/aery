use aery_core::{Client, GameMatch, Summoner};
use futures::future;
use futures::stream::{self, StreamExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = dotenv::var("RGAPI_KEY")?;
    let client = Client::new(api_key);

    let name = String::from("shiny abra");
    println!("Searching for: {name}");
    println!();

    match Summoner::from_name(&client, &name).await {
        Ok(summoner) => {
            println!("Name: {}", summoner.name());
            println!("Level: {}", summoner.level());
            println!("Icon: {}", summoner.icon_id());
            println!();

            // leagues
            match summoner.leagues(&client).await {
                Ok(leagues) => {
                    for league in leagues {
                        println!("League: {:?}", league.queue_kind());
                        println!("Rank: {:?} {:?}", league.tier(), league.division());
                        println!("Points: {}", league.points());
                        println!("{}W {}L", league.wins(), league.losses());

                        let range = 0..10;
                        let queue = league.queue_kind();
                        println!("Matches:");
                        stream::iter(summoner.matches(&client, range, queue).await?)
                            .map(|id| GameMatch::from_id(&client, id))
                            .buffered(10)
                            .filter_map(|game| future::ready(game.ok()))
                            .for_each(|game| {
                                let player = game
                                    .participant(summoner.puuid())
                                    .expect("Participant missing");

                                let result = if player.win { "Win" } else { "Lose" };

                                println!(
                                    "[{}] {}: {}/{}/{}",
                                    result,
                                    player.champion_name,
                                    player.kills,
                                    player.deaths,
                                    player.assists
                                );

                                future::ready(())
                            })
                            .await;
                        println!();
                    }
                }
                Err(error) => {
                    println!("Error: {error}")
                }
            }
        }
        Err(error) => {
            println!("Error: {error}");
        }
    }

    Ok(())
}
