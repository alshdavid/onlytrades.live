use sha2::Digest;
use sha2::Sha256;

pub fn sha256(data: &[u8]) -> String {
  let mut hasher = Sha256::new();
  hasher.update(data);
  let result = hasher.finalize();
  result.iter().map(|b| format!("{:02x}", b)).collect()
}
