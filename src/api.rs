mod stuff;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use stuff::*;

const ROOT: &str = "http://0.0.0.0:3000";

const SNAPSHOT_HEIGHT: u32 = 826600;

type Result<T> = std::result::Result<T, anyhow::Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let mut ret = BTreeMap::<String, Stats>::new();

    // let first_block = 767430;
    // let snapshot_block = 826600;

    let mut page = 0;
    loop {
        let is_more = process_inscriptions_page(page, &mut ret).await?;
        if !is_more {
            break;
        }
        page += 1;
    }

    // println!("ret: {:?}", ret);

    count_addresses(&ret);

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

    let is_more = block.more;

    for insc in block.inscriptions {
        let i = get_inscription(&insc).await?;
        // println!("=> HEIGHT: {:?}", i.genesis_height);
        let skip = i.genesis_height > SNAPSHOT_HEIGHT;

        if !skip {
            if let Some(addy) = i.address {
                if let Some(mime) = i.content_type {
                    mutate_ret(ret, &addy, &mime);
                }
            };
        }
    }
    Ok(is_more)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InscriptionJson {
    genesis_height: u32,
    address: Option<String>,
    content_type: Option<String>,
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

/*

curl --insecure https://ordstuff.info/Downloads/indexes/0.15.x/0.15.x%20uses%20the%20same%20index%20files%20as%200.14.x/index-without-sats-824298-0.14.1.redb.gz -o index15.redb.gz

*/
