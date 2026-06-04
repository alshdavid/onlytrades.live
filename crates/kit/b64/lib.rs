#![deny(unused_crate_dependencies)]
pub mod serde;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;

/// Takes a raw byte slice and encodes it into a Base64 string.
pub fn encode(input: &[u8]) -> String {
  STANDARD.encode(input)
}

/// Takes a Base64-encoded string and decodes it back into its raw binary bytes.
pub fn decode(input: &str) -> anyhow::Result<Vec<u8>> {
  let decoded_bytes = STANDARD.decode(input)?;
  Ok(decoded_bytes)
}
