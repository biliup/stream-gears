use anyhow::Result;
use m3u8_rs::Playlist;
use reqwest::header::HeaderMap;
use reqwest::Url;
use std::fs::File;
use std::io::Write;

use tracing::{debug, warn};

pub fn download(url: &str, headers: &HeaderMap, file_name: &str) -> Result<()> {
    println!("Downloading {}...", url);
    let resp = super::get_response(url, headers)?;
    println!("{}", resp.status());
    // let mut resp = resp.bytes_stream();
    let bytes = resp.bytes()?;
    File::create(format!("{file_name}.ts"))?;
    let mut media_url = Url::parse(url)?;
    let mut pl = match m3u8_rs::parse_playlist(&bytes) {
        Ok((_i, Playlist::MasterPlaylist(pl))) => {
            println!("Master playlist:\n{:#?}", pl);
            media_url = media_url.join(&pl.variants[0].uri)?;
            println!("media url: {media_url}");
            let resp = super::get_response(media_url.as_str(), headers)?;
            let bs = resp.bytes()?;
            // println!("{:?}", bs);
            if let Ok((_, pl)) = m3u8_rs::parse_media_playlist(&bs) {
                pl
            } else {
                let mut file = File::create("test.fmp4")?;
                file.write(&bs)?;
                panic!("Unable to parse the content.")
            }
        }
        Ok((_i, Playlist::MediaPlaylist(pl))) => {
            println!("Media playlist:\n{:#?}", pl);
            println!("index {}", pl.media_sequence);
            pl
        }
        Err(e) => panic!("Parsing error: \n{}", e),
    };
    let mut previous_last_segment = 0;
    loop {
        if pl.segments.is_empty() {
            println!("Segments array is empty - stream finished");
            break;
        }
        let mut seq = pl.media_sequence;
        for segment in &pl.segments {
            if seq > previous_last_segment {
                if (previous_last_segment > 0) && (seq > (previous_last_segment + 1)) {
                    println!("SEGMENT INFO SKIPPED");
                    warn!("SEGMENT INFO SKIPPED");
                }
                println!("Downloading segment...");
                debug!("Yield segment");
                download_to_file(media_url.join(&segment.uri)?, file_name, headers)?;
                previous_last_segment = seq;
            }
            seq += 1;
        }
        let resp = super::get_response(media_url.as_str(), headers)?;
        let bs = resp.bytes()?;
        if let Ok((_, playlist)) = m3u8_rs::parse_media_playlist(&bs) {
            pl = playlist;
        }
    }
    println!("Done...");
    Ok(())
}

fn download_to_file(url: Url, file_name: &str, headers: &HeaderMap) -> Result<()> {
    println!("url: {url}");
    let mut response = super::get_response(url.as_str(), headers)?;
    let mut out = File::options()
        .append(true)
        .open(format!("{file_name}.ts"))?;
    response.copy_to(&mut out)?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use anyhow::Result;
    use reqwest::Url;

    #[test]
    fn test_url() -> Result<()> {
        let url = Url::parse("h://host.path/to/remote/resource.m3u8")?;
        let scheme = url.scheme();
        let new_url = url.join("http://path.host/remote/resource.ts")?;
        println!("{url}, {scheme}");
        println!("{new_url}, {scheme}");
        Ok(())
    }

    #[test]
    fn it_works() -> Result<()> {
        // download(
        //     "test.ts")?;
        Ok(())
    }
}
