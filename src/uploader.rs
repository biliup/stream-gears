use anyhow::{Context, Result};
use biliup::client::Client;
use biliup::line::{self, Probe};
use biliup::video::{BiliBili, Studio, Video};
use biliup::VideoFile;
use futures::StreamExt;
use pyo3::pyclass;
use serde_json::Value;
use std::path::PathBuf;
use std::time::Instant;
use tracing::info;

#[pyclass]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum UploadLine {
    Bda2,
    Ws,
    Qn,
    Kodo,
    Cos,
    CosInternal,
}

async fn upload_vid(
    client: &Client,
    video_path: Vec<PathBuf>,
    line: Option<UploadLine>,
    limit: usize,
) -> Result<Vec<Video>> {
    let mut videos = Vec::new();
    let line = match line {
        Some(UploadLine::Kodo) => line::kodo(),
        Some(UploadLine::Bda2) => line::bda2(),
        Some(UploadLine::Ws) => line::ws(),
        Some(UploadLine::Qn) => line::qn(),
        Some(UploadLine::Cos) => line::cos(),
        Some(UploadLine::CosInternal) => line::cos_internal(),
        None => Probe::probe().await.unwrap_or_default(),
    };
    // let line = line::kodo();
    for video_path in video_path {
        println!("{:?}", video_path.canonicalize()?.to_str());
        info!("{line:?}");
        let video_file = VideoFile::new(&video_path)?;
        let total_size = video_file.total_size;
        let file_name = video_file.file_name.clone();
        let uploader = line.to_uploader(video_file);

        let instant = Instant::now();

        let video = uploader
            .upload(&client, limit, |vs| {
                vs.map(|vs| {
                    let chunk = vs?;
                    let len = chunk.len();
                    Ok((chunk, len))
                })
            })
            .await?;
        let t = instant.elapsed().as_millis();
        info!(
            "Upload completed: {file_name} => cost {:.2}s, {:.2} MB/s.",
            t as f64 / 1000.,
            total_size as f64 / 1000. / t as f64
        );
        videos.push(video);
    }
    return Ok(videos);
}

pub async fn upload(
    video_path: Vec<PathBuf>,
    cookie_file: PathBuf,
    line: Option<UploadLine>,
    limit: usize,
    title: String,
    tid: u16,
    tag: String,
    copyright: u8,
    source: String,
    desc: String,
    dynamic: String,
    cover: String,
    dtime: Option<u32>,
) -> Result<Value> {
    let client: Client = Default::default();
    let file = std::fs::File::options()
        .read(true)
        .write(true)
        .open(&cookie_file);
    let login_info = client
        .login_by_cookies(file.with_context(|| cookie_file.to_str().unwrap().to_string())?)
        .await?;
    let videos = upload_vid(&client, video_path, line, limit).await?;
    let mut studio: Studio = Studio::builder()
        .desc(desc)
        .dtime(dtime)
        .copyright(copyright)
        .cover(cover)
        .dynamic(dynamic)
        .source(source)
        .tag(tag)
        .tid(tid)
        .title(title)
        .videos(videos)
        .build();
    if !studio.cover.is_empty() {
        let url = BiliBili::new(&login_info, &client)
            .cover_up(
                &std::fs::read(&studio.cover)
                    .with_context(|| format!("cover: {}", studio.cover))?,
            )
            .await?;
        println!("{url}");
        studio.cover = url;
    }
    Ok(studio.submit(&login_info).await?)
    // Ok(videos)
}

pub async fn edit(
    aid: u64,
    video_path: Vec<PathBuf>,
    cookie_file: PathBuf,
    line: Option<UploadLine>,
    limit: usize,
    title: String,
    tid: u16,
    tag: String,
    copyright: u8,
    source: String,
    desc: String,
    dynamic: String,
    cover: String,
    dtime: Option<u32>,
) -> Result<Value> {
    let client: Client = Default::default();
    let login_info = client
        .login_by_web_cookies(&sess_data, &bili_jct)
        .await?;
    let videos = upload_vid(&client, video_path, line, limit).await?;
    let mut studio: Studio = Studio::builder()
        .aid(aid)
        .desc(desc)
        .dtime(dtime)
        .copyright(copyright)
        .cover(cover)
        .dynamic(dynamic)
        .source(source)
        .tag(tag)
        .tid(tid)
        .title(title)
        .videos(videos)
        .build();
    if !studio.cover.is_empty() {
        let url = BiliBili::new(&login_info, &client)
            .cover_up(
                &std::fs::read(&studio.cover)
                    .with_context(|| format!("cover: {}", studio.cover))?,
            )
            .await?;
        println!("{url}");
        studio.cover = url;
    }
    Ok(studio.edit(&login_info).await?)
    // Ok(videos)
}

pub async fn upload_web_cookies(
    video_path: Vec<PathBuf>,
    sess_data: String,
    bili_jct: String,
    line: Option<UploadLine>,
    limit: usize,
    title: String,
    tid: u16,
    tag: String,
    copyright: u8,
    source: String,
    desc: String,
    dynamic: String,
    cover: String,
    dtime: Option<u32>,
) -> Result<Value> {
    let client: Client = Default::default();
    let login_info = client
        .login_by_web_cookies(&sess_data, &bili_jct)
        .await?;
    let videos = upload_vid(&client, video_path, line, limit).await?;
    let mut studio: Studio = Studio::builder()
        .desc(desc)
        .dtime(dtime)
        .copyright(copyright)
        .cover(cover)
        .dynamic(dynamic)
        .source(source)
        .tag(tag)
        .tid(tid)
        .title(title)
        .videos(videos)
        .build();
    if !studio.cover.is_empty() {
        let url = BiliBili::new(&login_info, &client)
            .cover_up(
                &std::fs::read(&studio.cover)
                    .with_context(|| format!("cover: {}", studio.cover))?,
            )
            .await?;
        println!("{url}");
        studio.cover = url;
    }
    Ok(studio.submit(&login_info).await?)
    // Ok(videos)
}

pub async fn edit_web_cookies(
    aid: u64,
    video_path: Vec<PathBuf>,
    sess_data: String,
    bili_jct: String,
    line: Option<UploadLine>,
    limit: usize,
    title: String,
    tid: u16,
    tag: String,
    copyright: u8,
    source: String,
    desc: String,
    dynamic: String,
    cover: String,
    dtime: Option<u32>,
) -> Result<Value> {
    let client: Client = Default::default();
    let login_info = client
        .login_by_web_cookies(&sess_data, &bili_jct)
        .await?;
    let videos = upload_vid(&client, video_path, line, limit).await?;
    let mut studio: Studio = Studio::builder()
        .aid(aid)
        .desc(desc)
        .dtime(dtime)
        .copyright(copyright)
        .cover(cover)
        .dynamic(dynamic)
        .source(source)
        .tag(tag)
        .tid(tid)
        .title(title)
        .videos(videos)
        .build();
    if !studio.cover.is_empty() {
        let url = BiliBili::new(&login_info, &client)
            .cover_up(
                &std::fs::read(&studio.cover)
                    .with_context(|| format!("cover: {}", studio.cover))?,
            )
            .await?;
        println!("{url}");
        studio.cover = url;
    }
    Ok(studio.edit(&login_info).await?)
    // Ok(videos)
}