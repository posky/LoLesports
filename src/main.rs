use lolesports::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = get_api().await.expect("API is not available");

    let rows = cargoquery(
        &api,
        "ScoreboardGames=SG, Tournaments=T",
        "T.Name, SG.DateTime_UTC, SG.Team1, SG.Team2",
        None,
        Some("SG.OverviewPage=T.OverviewPage"),
    )
    .await
    .expect("failed to get response");

    println!("{:?}", rows);

    let rows = get_leagues(&api, None)
        .await
        .expect("failed to get response");
    println!();
    println!("{:?}", rows);

    Ok(())
}
