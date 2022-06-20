use std::env;
use std::io::{BufReader, BufWriter, ErrorKind, Read};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use stream_gears::downloader::httpflv::map_parse_err;
use stream_gears::flv_parser::{aac_audio_packet_header, avc_video_packet_header, CodecId, header, script_data, SoundFormat, tag_data, tag_header, TagData, TagHeader};
use stream_gears::flv_writer::{self, FlvTag, TagDataHeader, write_tag_header};
use stream_gears::error::Error;


fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let flv_file = std::fs::File::open(file_name)?;
    let buf_reader = BufReader::new(flv_file);
    let mut reader = Reader::new(buf_reader);

    let mut script_tag_count = 0;
    let mut audio_tag_count = 0;
    let mut video_tag_count = 0;
    let mut tag_count = 0;
    let mut err_count = 0;
    let flv_header = reader.read_frame(9)?;
    let file = std::fs::File::create(format!("{file_name}.json"))?;
    let mut writer = BufWriter::new(file);
    // Vec::clear()
    let (_, header) = map_parse_err(header(&flv_header), "flv header")?;
    flv_writer::to_json(&mut writer, &header)?;
    loop {
        let previous_tag_size = reader.read_frame(4)?;

        let t_header = reader.read_frame(11)?;
        if t_header.is_empty() { break; }
        let tag_header = match map_parse_err(tag_header(&t_header), "tag header") {
            Ok((_, tag_header)) => {tag_header}
            Err(e) => {
                println!("{e}");
                break;
            }
        };
        tag_count += 1;
        let bytes = reader.read_frame(tag_header.data_size as usize)?;
        let (i, flv_tag_data) = match map_parse_err(tag_data(tag_header.tag_type, tag_header.data_size as usize)(&bytes), "tag data") {
            Ok((i, flv_tag_data)) => (i, flv_tag_data),
            Err(e) => {
                println!("{e}");
                break;
            }
        };

        let flv_tag = match flv_tag_data {
            TagData::Audio(audio_data) => {
                audio_tag_count += 1;

                let packet_type = if audio_data.sound_format == SoundFormat::AAC {
                    let (_, packet_header) = aac_audio_packet_header(audio_data.sound_data).unwrap();
                    Some(packet_header.packet_type)
                } else { None };
                let flv_tag = FlvTag {
                    header: tag_header,
                    data: TagDataHeader::Audio{
                        sound_format: audio_data.sound_format,
                        sound_rate: audio_data.sound_rate,
                        sound_size: audio_data.sound_size,
                        sound_type: audio_data.sound_type,
                        packet_type
                    }
                };
                flv_tag
            }
            TagData::Video(video_data) => {
                video_tag_count += 1;

                let (packet_type, composition_time) = if CodecId::H264 == video_data.codec_id {
                    let (_, avc_video_header) = avc_video_packet_header(video_data.video_data).unwrap();
                    (Some(avc_video_header.packet_type), Some(avc_video_header.composition_time))
                } else { (None, None) };
                let flv_tag = FlvTag {
                    header: tag_header,
                    data: TagDataHeader::Video {
                        frame_type: video_data.frame_type,
                        codec_id: video_data.codec_id,
                        packet_type,
                        composition_time,
                    }
                };
                flv_tag
            }
            TagData::Script => {
                script_tag_count += 1;

                let (_, tag_data) = script_data(i).unwrap();
                let flv_tag = FlvTag {
                    header: tag_header,
                    data: TagDataHeader::Script(tag_data)
                };
                flv_tag
            }
        };
        flv_writer::to_json(&mut writer, &flv_tag)?;
    }
    println!("tag count: {tag_count}");
    println!("audio tag count: {audio_tag_count}");
    println!("script tag count: {script_tag_count}");
    println!("video tag count: {video_tag_count}");
    Ok(())
}

pub struct Reader<T>{
    read: T,
    buffer: BytesMut,
}

impl<T: Read> Reader<T> {
    fn new(read: T) -> Reader<T> {
        Reader {
            read,
            buffer: BytesMut::with_capacity(8 * 1024)
        }
    }

    fn read_frame(&mut self, chunk_size: usize) -> std::io::Result<Bytes> {
        let mut buf = [0u8; 8 * 1024];
        loop {
            if chunk_size <= self.buffer.len() {
                let bytes = Bytes::copy_from_slice(&self.buffer[..chunk_size]);
                self.buffer.advance(chunk_size as usize);
                return Ok(bytes)
            }
            // BytesMut::with_capacity(0).deref_mut()
            // tokio::fs::File::open("").read()
            // self.read_buf.
            let n = match self.read.read(&mut buf) {
                Ok(n) => n,
                Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            };
            if n == 0 {
                return Ok(self.buffer.split().freeze())
            }
            self.buffer.put_slice(&buf[..n]);
        }
    }
}