use serde::{Deserialize, Serialize};

const ROOT: &str = "http://0.0.0.0:3000";

type Result<T> = std::result::Result<T, anyhow::Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let id1 = "6fb976ab49dcec017f1e201e84395983204ae1a7c2abf7ced0a85d692e442799i0";
    let url1 = format!("inscription/{}", id1);
    let resp = get(&url1).await?;
    println!("{}", resp);

    let first_block = 767430;
    let end_block = first_block + 100;
    let snapshot_block = 826600;

    for bn in first_block..end_block {
        get_inscriptions_in_block(bn).await?;
    }

    for page in 0..100 {
        get_inscriptions_page(page).await?;
    }

    Ok(())
}

async fn get_inscriptions_page(page: u64) -> Result<()> {
    let url = format!("inscriptions/{}", page);
    let resp = get(&url).await?;
    let block: InscriptionsJson = serde_json::from_str(&resp)?;
    if block.more {
        println!("=======> page {} has more inscriptions", page);
    }
    println!(
        "=> page {} has {} inscriptions",
        page,
        block.inscriptions.len()
    );
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InscriptionsJson {
    inscriptions: Vec<String>,
    more: bool,
    page_index: u64,
}
async fn get_inscriptions_in_block(bn: u64) -> Result<()> {
    let url = format!("inscriptions/block/{}", bn);
    let resp = get(&url).await?;
    let block: InscriptionsJson = serde_json::from_str(&resp)?;
    if block.more {
        println!("=======> block {} has more inscriptions", bn);
    }
    println!(
        "=> block {} has {} inscriptions",
        bn,
        block.inscriptions.len()
    );
    Ok(())
}

async fn get(path: &str) -> Result<String> {
    let url = format!("{}/{}", ROOT, path);
    let client = reqwest::Client::new();
    Ok(client
        .get(url)
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .await?
        .text()
        .await?)
}
