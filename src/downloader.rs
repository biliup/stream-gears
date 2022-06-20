use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use nom::{Err, IResult};
use reqwest::header::{ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, HeaderMap, HeaderName, HeaderValue, InvalidHeaderValue, REFERER, USER_AGENT};
use reqwest::blocking::Response;
use crate::flv_parser::header;

pub mod httpflv;
mod hls;

pub fn download(url: &str, headers: HeaderMap, file_name: &str, segment: Segment) -> anyhow::Result<()> {
    let mut response = get_response(url, &headers)?;
    let buf = &mut [0u8;9];
    response.read_exact(buf)?;
    // let content = response.read(buf)?;
    match header(buf) {
        Ok((i, header)) => {
            println!("header: {header:#?}");
            httpflv::download(url, &headers, file_name, segment)?;
        }
        Err(Err::Incomplete(needed)) => {
            println!("needed: {needed:?}")
        }
        Err(e) => {
            println!("{e}");
            hls::download(url, &headers, file_name)?;
        }
    }
    Ok(())
}

pub fn construct_headers(hash_map: HashMap<String, String>) -> HeaderMap {
    let mut headers = HeaderMap::new();
    for (key, value) in hash_map.iter() {
        headers.insert(HeaderName::from_str(key).unwrap(), HeaderValue::from_str(value).unwrap());
    }
    headers
}

pub fn get_response(url: &str, headers: &HeaderMap) -> anyhow::Result<Response> {
    let mut resp =  retry(|| {
        reqwest::blocking::Client::new()
            .get(url)
            .header(ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
            .header(ACCEPT_ENCODING, "gzip, deflate")
            .header(ACCEPT_LANGUAGE, "zh-CN,zh;q=0.8,en-US;q=0.5,en;q=0.3")
            .header(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64; rv:38.0) Gecko/20100101 Firefox/38.0 Iceweasel/38.2.1")
            .headers(headers.clone())
            .send()
    })?;
    resp.error_for_status_ref()?;
    Ok(resp)
}

fn retry<O, E: std::fmt::Display>(mut f: impl FnMut()-> Result<O, E>) -> Result<O, E> {
    let mut retries = 3;
    loop {
        match f() {
            Err(e) if retries > 0 => {
                retries -= 1;
                println!(
                    "Retry attempt #{}. Sleeping 500ms before the next attempt. {e}",
                    3 - retries,
                );
                sleep(Duration::from_millis(500));
            }
            res => break res,
        }
    }
}


pub enum Segment{
    Time(Duration),
    Size(u64)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use anyhow::Result;
    use crate::downloader::{download, Segment};

    #[test]
    fn it_works() -> Result<()> {
        // download(
        //     "new_test",
        //     // Segment::Time(Duration::from_secs(30))
        //     Segment::Size(20 * 1024 * 1024)
        // )?;
        Ok(())
    }
}