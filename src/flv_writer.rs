use crate::flv_parser::{
    AACPacketType, AVCPacketType, CodecId, FrameType, ScriptData, SoundFormat, SoundRate,
    SoundSize, SoundType, TagHeader,
};
use byteorder::{BigEndian, WriteBytesExt};
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};

const FLV_HEADER: [u8; 9] = [
    0x46, // 'F'
    0x4c, //'L'
    0x56, //'V'
    0x01, //version
    0x05, //00000101  audio tag  and video tag
    0x00, 0x00, 0x00, 0x09, //flv header size
]; // 9

pub fn create_flv_file(file_name: &str) -> std::io::Result<impl Write> {
    let out = File::create(format!("{file_name}.flv")).expect("Unable to create flv file.");
    let mut buf_writer = BufWriter::new(out);
    buf_writer.write(&FLV_HEADER)?;
    write_previous_tag_size(&mut buf_writer, 0)?;
    Ok(buf_writer)
}

pub fn write_tag(
    out: &mut impl Write,
    tag_header: &TagHeader,
    body: &[u8],
    previous_tag_size: &[u8],
) -> std::io::Result<usize> {
    write_tag_header(out, tag_header)?;
    out.write(body)?;
    out.write(previous_tag_size)
}

pub fn write_tag_header(writer: &mut impl Write, tag_header: &TagHeader) -> std::io::Result<()> {
    writer.write_u8(tag_header.tag_type as u8)?;
    writer.write_u24::<BigEndian>(tag_header.data_size)?;
    writer.write_u24::<BigEndian>(tag_header.timestamp & 0xffffff)?;
    let timestamp_ext = (tag_header.timestamp >> 24 & 0xff) as u8;
    writer.write_u8(timestamp_ext)?;
    writer.write_u24::<BigEndian>(tag_header.stream_id)
}

pub fn write_previous_tag_size(
    writer: &mut impl Write,
    previous_tag_size: u32,
) -> std::io::Result<usize> {
    writer.write(&previous_tag_size.to_be_bytes())
}

#[derive(Debug, PartialEq, Serialize)]
pub struct FlvTag<'a> {
    pub header: TagHeader,
    pub data: TagDataHeader<'a>,
}

pub fn to_json<T: ?Sized + Serialize>(mut writer: impl Write, t: &T) -> std::io::Result<usize> {
    serde_json::to_writer(&mut writer, t)?;
    writer.write("\n".as_ref())
}

#[derive(Debug, PartialEq, Serialize)]
pub enum TagDataHeader<'a> {
    Audio {
        sound_format: SoundFormat,
        sound_rate: SoundRate,
        sound_size: SoundSize,
        sound_type: SoundType,
        packet_type: Option<AACPacketType>,
    },
    Video {
        frame_type: FrameType,
        codec_id: CodecId,
        packet_type: Option<AVCPacketType>,
        composition_time: Option<i32>,
    },
    Script(ScriptData<'a>),
}
