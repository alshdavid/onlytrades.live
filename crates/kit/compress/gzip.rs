use std::io::Write;

use flate2::Compression;
use flate2::write::GzEncoder;

pub fn gzip(input: &[u8]) -> Vec<u8> {
  // Compression::default() is 6. You can use Compression::best() for 9.
  let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
  encoder
    .write_all(input)
    .expect("Failed to write to Gzip encoder");
  encoder.finish().expect("Failed to finish Gzip encoding")
}
