use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    pub any: u64,
    pub cool: u64,
    pub boring: u64,
}

pub fn is_boring(mime: &str) -> bool {
    mime.contains("text/plain") || mime.contains("application/json")
}

pub fn mutate_ret(ret: &mut BTreeMap<String, Stats>, addy: &str, mime: &str) {
    match ret.get_mut(addy) {
        Some(r) => {
            r.any += 1;
            match is_boring(mime) {
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
            match is_boring(mime) {
                true => sta.boring += 1,
                false => sta.cool += 1,
            };
            ret.insert(addy.to_string(), sta);
        }
    }
}

pub fn count_addresses(ret: &BTreeMap<String, Stats>) {
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
