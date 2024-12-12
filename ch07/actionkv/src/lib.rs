use byteorder::{BigEndian as Endian, ReadBytesExt, WriteBytesExt};
use crc;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::{
  collections::HashMap,
  fs::OpenOptions,
  io::{self, BufReader, Read, Seek, SeekFrom},
  path::Path,
};

type ByteString = Vec<u8>;

type ByteStr = [u8];

const CRC32: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValuePair {
  pub key: ByteString,
  pub value: ByteString,
}

#[derive(Debug)]
pub struct ActionKV {
  file: File,
  pub index: HashMap<ByteString, u64>,
}

impl ActionKV {
  pub fn open(path: &Path) -> io::Result<Self> {
    let file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .append(true)
      .open(path)?;
    let index = HashMap::new();
    Ok(ActionKV { file, index })
  }

  pub fn load(&mut self) -> io::Result<()> {
    let mut file = BufReader::new(&mut self.file);
    loop {
      let position = file.seek(SeekFrom::Current(0))?;
      let maybe_kv = ActionKV::process_record(&mut file);
      let kv = match maybe_kv {
        Ok(kv) => kv,
        Err(err) => match err.kind() {
          io::ErrorKind::UnexpectedEof => break,
          _ => return Err(err),
        },
      };
      self.index.insert(kv.key, position);
    }

    Ok(())
  }

  fn process_record<R: Read>(file: &mut R) -> io::Result<KeyValuePair> {
    let saved_checksum = file.read_u32::<Endian>()?;
    let key_len = file.read_u32::<Endian>()?;
    let value_len = file.read_u32::<Endian>()?;

    let data_len = key_len + value_len;
    let mut data = ByteString::with_capacity(data_len as usize);
    {
      file.by_ref().take(data_len as u64).read_to_end(&mut data)?;
    }
    debug_assert_eq!(data.len(), data_len as usize);

    let checksum = CRC32.checksum(&data);
    if checksum != saved_checksum {
      panic!(
        "data corruption detected ({:08x} != {:08x})",
        saved_checksum, checksum
      );
    }
    let value = data.split_off(key_len as usize);
    let key = data;
    Ok(KeyValuePair { key, value })
  }

  pub fn insert(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
    let position = self.insert_but_ignore_index(key, value)?;
    self.index.insert(key.to_vec(), position);
    Ok(())
  }

  pub fn insert_but_ignore_index(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<u64> {
    let mut file = BufWriter::new(&mut self.file);
    let key_len = key.len();
    let value_len = value.len();
    let mut tmp = ByteString::with_capacity(key_len + value_len);
    for byte in key {
      tmp.push(*byte);
    }
    for byte in value {
      tmp.push(*byte);
    }
    let checksum = CRC32.checksum(&tmp);

    let next_byte = SeekFrom::End(0);
    let current_position = file.seek(SeekFrom::Current(0));
    let _ = file.seek(next_byte);
    let _ = file.write_u32::<Endian>(checksum);
    let _ = file.write_u32::<Endian>(key_len as u32);
    let _ = file.write_u32::<Endian>(value_len as u32);
    file.write_all(&tmp)?;
    io::Result::Ok(current_position)?
  }

  pub fn seek_to_end(&mut self) -> io::Result<u64> { self.file.seek(SeekFrom::End(0)) }

  pub fn get(&mut self, key: &ByteStr) -> io::Result<Option<ByteString>> {
    let position = match self.index.get(key) {
      None => return Ok(None),
      Some(position) => *position,
    };

    let kv = self.get_at(position)?;
    Ok(Some(kv.value))
  }

  pub fn get_at(&mut self, position: u64) -> io::Result<KeyValuePair> {
    let mut file = BufReader::new(&mut self.file);
    file.seek(SeekFrom::Start(position))?;
    let kv = ActionKV::process_record(&mut file);
    io::Result::Ok(kv)?
  }

  pub fn find(&mut self, target: &ByteStr) -> io::Result<Option<(u64, ByteString)>> {
    let mut file = BufReader::new(&mut self.file);
    let mut found: Option<(u64, ByteString)> = None;
    loop {
      let position = file.seek(SeekFrom::Current(0))?;
      let optional_kv = ActionKV::process_record(&mut file);
      let kv = match optional_kv {
        Ok(kv) => kv,
        Err(err) => match err.kind() {
          io::ErrorKind::UnexpectedEof => break,
          _ => return Err(err),
        },
      };
      if kv.key == target {
        found = Some((position, kv.value));
      }
    }
    Ok(found)
  }

  #[inline]
  pub fn update(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
    self.insert(key, value)
  }

  #[inline]
  pub fn delete(&mut self, key: &ByteStr) -> io::Result<()> { self.insert(key, b"") }
}
