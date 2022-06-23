use crate::downloader::httpflv::Connection;
use crate::flv_parser::header;
use nom::{Err, IResult};
use reqwest::blocking::Response;
use reqwest::header::{
    HeaderMap, HeaderName, HeaderValue, InvalidHeaderValue, ACCEPT, ACCEPT_ENCODING,
    ACCEPT_LANGUAGE, CONNECTION, REFERER, USER_AGENT,
};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Read};
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

mod hls;
pub mod httpflv;

pub fn download(
    url: &str,
    headers: HeaderMap,
    file_name: &str,
    segment: Segment,
) -> anyhow::Result<()> {
    let mut response = get_response(url, &headers)?;
    let buf = &mut [0u8; 9];
    response.read_exact(buf)?;
    // let out = File::create(format!("{}.flv", file_name)).expect("Unable to create file.");
    // let mut writer = BufWriter::new(out);
    // let mut buf = [0u8; 8 * 1024];
    // response.copy_to(&mut writer)?;
    // io::copy(&mut resp, &mut out).expect("Unable to copy the content.");
    match header(buf) {
        Ok((i, header)) => {
            println!("header: {header:#?}");
            println!("{}", response.status());
            let mut connection = Connection::new(response);
            println!("Downloading {}...", url);
            httpflv::download(connection, file_name, segment);
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
        headers.insert(
            HeaderName::from_str(key).unwrap(),
            HeaderValue::from_str(value).unwrap(),
        );
    }
    headers
}

pub fn get_response(url: &str, headers: &HeaderMap) -> anyhow::Result<Response> {
    let mut resp = retry(|| {
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

fn retry<O, E: std::fmt::Display>(mut f: impl FnMut() -> Result<O, E>) -> Result<O, E> {
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

pub enum Segment {
    Time(Duration),
    Size(u64),
}

#[cfg(test)]
mod tests {
    use crate::downloader::{download, Segment};
    use anyhow::Result;
    use reqwest::header::HeaderMap;
    use std::time::Duration;

    #[test]
    fn it_works() -> Result<()> {
        tracing_subscriber::fmt::init();
        download(
            "url",
            HeaderMap::new(),
            "testdouyu",
            Segment::Size(2000 * 1024 * 1024),
            // Segment::Time(Duration::from_secs(30))
        )?;
        Ok(())
    }
}
