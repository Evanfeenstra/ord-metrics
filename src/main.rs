mod stuff;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use stuff::*;

type Result<T> = std::result::Result<T, anyhow::Error>;

#[tokio::main]

async fn main() -> Result<()> {
    let mut ret = BTreeMap::<String, Stats>::new();

    let args: Vec<String> = std::env::args().collect();
    let filepath = &args[1];
    println!("FILEDPATH: {:?}", filepath);

    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split("\t");
        let _num = parts.next().unwrap();
        let _height = parts.next().unwrap();
        let address = parts.next().unwrap();
        let mime = parts.next().unwrap();
        mutate_ret(&mut ret, address, mime);
    }

    count_addresses(&ret);

    Ok(())
}
