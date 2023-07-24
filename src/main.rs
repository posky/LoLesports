use lolesports::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = get_api().await.expect("API is not available");

    let tournaments = get_tournaments(&api, Some("T.Year=2023"))
        .await
        .unwrap()
        .unwrap();
    println!("{:?}", tournaments);

    let row = &tournaments[0];
    println!("\n\n{:?}", row);

    Ok(())
}
