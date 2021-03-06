//! Local YYX data

#![warn(clippy::all)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate yyx_utils;

use std::fs;
use std::path::{Path, PathBuf};

use yyx_types::Snapshot;

pub mod result;

use self::result::DataResult;

pub fn init() -> DataResult<()> {
  ensure_data_dir()?;

  Ok(())
}

const LAST_SNAPSHOT_FILE_NAME: &str = "last_snapshot.json";

pub fn save_last_snapshot(snapshot: &Snapshot) -> DataResult<()> {
  use std::io::BufWriter;
  let file = fs::File::create(get_data_path(LAST_SNAPSHOT_FILE_NAME)?)?;
  serde_json::to_writer(BufWriter::new(file), snapshot)?;
  Ok(())
}

pub fn load_last_snapshot() -> DataResult<Option<Snapshot>> {
  use std::io::ErrorKind;
  fs::read(get_data_path(LAST_SNAPSHOT_FILE_NAME)?)
    .map(Some)
    .or_else(|err| {
      if let ErrorKind::NotFound = err.kind() {
        Ok(None)
      } else {
        Err(err.into())
      }
    })
    .and_then(|bytes| {
      if let Some(bytes) = bytes {
        serde_json::from_reader(&bytes as &[u8]).map_err(Into::into)
      } else {
        Ok(None)
      }
    })
}

fn ensure_data_dir() -> DataResult<()> {
  fs::create_dir_all("data").map_err(|err| {
    error!("Create data dir error: {}", err);
    err
  })?;
  Ok(())
}

fn get_data_path<T: AsRef<Path>>(path: T) -> DataResult<PathBuf> {
  ensure_data_dir()?;
  Ok(Path::new("data").join(path))
}
