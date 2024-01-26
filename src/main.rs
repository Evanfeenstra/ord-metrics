use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const ROOT: &str = "http://0.0.0.0:3000";

const SNAPSHOT_HEIGHT: u32 = 826600;

type Result<T> = std::result::Result<T, anyhow::Error>;

#[derive(Serialize, Deserialize, Debug)]
struct Stats {
    any: u64,
    cool: u64,
    boring: u64,
}

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

fn count_addresses(ret: &BTreeMap<String, Stats>) {
    /*
    -Holds 2 or more inscriptions of any file type
    -Holds 3 or more inscriptions of any file type
    -Holds 4 or more inscriptions of any file type
    -Holds 5 or more inscriptions of any file type
    -Holds 2 or more inscriptions that are not the file types text/plain, text/plain;charset=utf-8, or application/json
    -Holds 3 or more inscriptions that are not the file types text/plain, text/plain;charset=utf-8, or application/json
    -Holds 4 or more inscriptions that are not the file types text/plain, text/plain;charset=utf-8, or application/json
    -Holds 5 or more inscriptions that are not the file types text/plain, text/plain;charset=utf-8, or application/json
    -Holds 10 or more inscriptions that are either text/plain, text/plain;charset=utf-8, or application/json
    -Holds 25 or more inscriptions that are either text/plain, text/plain;charset=utf-8, or application/json
    -Holds 50 or more inscriptions that are either text/plain, text/plain;charset=utf-8, or application/json
    */
    let mut count_2_or_more = 0;
    let mut count_3_or_more = 0;
    let mut count_4_or_more = 0;
    let mut count_5_or_more = 0;
    let mut count_2_or_more_cool = 0;
    let mut count_3_or_more_cool = 0;
    let mut count_4_or_more_cool = 0;
    let mut count_5_or_more_cool = 0;
    let mut count_10_or_more_boring = 0;
    let mut count_25_or_more_boring = 0;
    let mut count_50_or_more_boring = 0;

    for (_addy, stats) in ret {
        //
        if stats.any >= 2 {
            count_2_or_more += 1;
        }
        if stats.any >= 3 {
            count_3_or_more += 1;
        }
        if stats.any >= 4 {
            count_4_or_more += 1;
        }
        if stats.any >= 5 {
            count_5_or_more += 1;
        }
        //
        if stats.cool >= 2 {
            count_2_or_more_cool += 1;
        }
        if stats.cool >= 3 {
            count_3_or_more_cool += 1;
        }
        if stats.cool >= 4 {
            count_4_or_more_cool += 1;
        }
        if stats.cool >= 5 {
            count_5_or_more_cool += 1;
        }
        //
        if stats.boring >= 10 {
            count_10_or_more_boring += 1;
        }
        if stats.boring >= 25 {
            count_25_or_more_boring += 1;
        }
        if stats.boring >= 50 {
            count_50_or_more_boring += 1;
        }
    }

    println!(
        "2 or more inscriptions of any file type: {}",
        count_2_or_more
    );
    println!(
        "3 or more inscriptions of any file type: {}",
        count_3_or_more
    );
    println!(
        "4 or more inscriptions of any file type: {}",
        count_4_or_more
    );
    println!(
        "5 or more inscriptions of any file type: {}",
        count_5_or_more
    );
    println!(
        "2 or more inscriptions that are not the file types text/plain, text/plain;charset=utf-8, or application/json: {}",
        count_2_or_more_cool
    );
    println!(
        "3 or more inscriptions that are not the file types text/plain, text/plain;charset=utf-8, or application/json: {}",
        count_3_or_more_cool
    );
    println!(
        "4 or more inscriptions that are not the file types text/plain, text/plain;charset=utf-8, or application/json: {}",
        count_4_or_more_cool
    );
    println!(
        "5 or more inscriptions that are not the file types text/plain, text/plain;charset=utf-8, or application/json: {}",
        count_5_or_more_cool
    );
    println!(
        "10 or more inscriptions that are either text/plain, text/plain;charset=utf-8, or application/json: {}",
        count_10_or_more_boring
    );
    println!(
        "25 or more inscriptions that are either text/plain, text/plain;charset=utf-8, or application/json: {}",
        count_25_or_more_boring
    );
    println!(
        "50 or more inscriptions that are either text/plain, text/plain;charset=utf-8, or application/json: {}",
        count_50_or_more_boring
    );
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
                    match ret.get_mut(&addy) {
                        Some(r) => {
                            r.any += 1;
                            match is_boring(&mime) {
                                true => r.boring += 1,
                                false => r.cool += 1,
                            };
                        }
                        None => {
                            let mut sta = Stats {
                                any: 1,
                                cool: 0,
                                boring: 0,
                            };
                            match is_boring(&mime) {
                                true => sta.boring += 1,
                                false => sta.cool += 1,
                            };
                            ret.insert(addy.clone(), sta);
                        }
                    }
                }
            }
        }
    }
    Ok(is_more)
}

fn is_boring(mime: &str) -> bool {
    mime.contains("text/plain") || mime.contains("application/json")
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
