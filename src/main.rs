use lolesports::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = get_api().await.expect("API is not available");

    let leagues = get_leagues(&api, None)
        .await
        .expect("failed to get response")
        .unwrap();
    println!("{:?}", leagues);

    let row = &leagues[0];

    println!("\n\n{:?}", row);

    Ok(())
}
