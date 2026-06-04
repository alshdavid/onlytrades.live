use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::LazyLock;

use anyhow::Context;
use include_dir::Dir;
use include_dir::include_dir;

pub static CLIENT_FILES: Dir = include_dir!("$CARGO_MANIFEST_DIR/../../../dist");

pub static CLIENT_FILES_BR: LazyLock<HashMap<PathBuf, Vec<u8>>> = LazyLock::new(|| {
  let mut stash = HashMap::<PathBuf, Vec<u8>>::new();

  for (_path, dest) in client_manifest().unwrap() {
    let contents = CLIENT_FILES.get_file(&dest).unwrap().contents().to_vec();
    let contents_compressed = kit_compress::brotli(&contents);
    stash.insert(dest, contents_compressed);
  }

  stash
});

pub static CLIENT_FILES_GZ: LazyLock<HashMap<PathBuf, Vec<u8>>> = LazyLock::new(|| {
  let mut stash = HashMap::<PathBuf, Vec<u8>>::new();

  for (_path, dest) in client_manifest().unwrap() {
    let contents = CLIENT_FILES.get_file(&dest).unwrap().contents().to_vec();
    let contents_compressed: Vec<u8> = kit_compress::gzip(&contents);
    stash.insert(dest, contents_compressed);
  }

  stash
});

pub fn client_manifest() -> anyhow::Result<HashMap<String, PathBuf>> {
  let client_manifest = CLIENT_FILES
    .get_file("client_manifest.json")
    .context("Missing client manifest")?;
  Ok(serde_json::from_slice(client_manifest.contents())?)
}
