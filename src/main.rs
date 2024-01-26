use std::any;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const ROOT: &str = "http://0.0.0.0:3000";

type Result<T> = std::result::Result<T, anyhow::Error>;

#[derive(Serialize, Deserialize, Debug)]
struct Stats {
    any: u64,
    img: u64,
    boring: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    // let id1 = "6fb976ab49dcec017f1e201e84395983204ae1a7c2abf7ced0a85d692e442799i0";
    // let url1 = format!("inscription/{}", id1);
    // let resp = get(&url1).await?;
    // println!("{}", resp);

    let mut ret = BTreeMap::<String, Stats>::new();

    // let first_block = 767430;
    // let end_block = first_block + 100;
    // let snapshot_block = 826600;

    let mut page = 0;
    loop {
        let is_more = process_inscriptions_page(page, &mut ret).await?;
        if !is_more || page > 10 {
            break;
        }
        page += 1;
    }

    println!("ret: {:?}", ret);

    Ok(())
}

async fn process_inscriptions_page(page: u64, ret: &mut BTreeMap<String, Stats>) -> Result<bool> {
    let url = format!("inscriptions/{}", page);
    let resp = get(&url).await?;
    let block: InscriptionsJson = serde_json::from_str(&resp)?;

    println!(
        "=> page {} has {} inscriptions",
        page,
        block.inscriptions.len()
    );
    for insc in block.inscriptions {
        let i = get_inscription(&insc).await?;
        if let Some(addy) = i.address {
            match ret.get_mut(&addy) {
                Some(r) => {
                    r.any += 1;
                }
                None => {
                    ret.insert(
                        addy.clone(),
                        Stats {
                            any: 1,
                            img: 0,
                            boring: 0,
                        },
                    );
                }
            }
        }
    }
    Ok(block.more)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InscriptionJson {
    address: Option<String>,
    content_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InscriptionsJson {
    inscriptions: Vec<String>,
    more: bool,
    page_index: u64,
}

async fn get_inscription(id: &str) -> Result<InscriptionJson> {
    let url = format!("inscription/{}", id);
    let resp = get(&url).await?;
    Ok(serde_json::from_str(&resp)?)
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
