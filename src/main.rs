const ROOT: &str = "http://0.0.0.0:3000";

type Result<T> = std::result::Result<T, anyhow::Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let id1 = "6fb976ab49dcec017f1e201e84395983204ae1a7c2abf7ced0a85d692e442799i0";
    let url1 = format!("inscriptions/{}", id1);
    let resp = get(&url1).await?;
    println!("{}", resp);

    let first_block = 767430;
    let snapshot_block = 826600;
    let url = format!("inscriptions/block/{}", first_block);
    let resp2 = get(&url).await?;
    println!("{}", resp2);

    Ok(())
}

async fn get(path: &str) -> Result<String> {
    let url = format!("{}/{}", ROOT, path);
    Ok(reqwest::get(url).await?.text().await?)
}
