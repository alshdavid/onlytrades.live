use std::fmt::Display;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub enum ProfilePermission {
  Admin,
}

impl TryFrom<String> for ProfilePermission {
  type Error = anyhow::Error;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    match value.as_str() {
      "admin" => Ok(Self::Admin),
      _ => anyhow::bail!("Invalid string"),
    }
  }
}

impl Display for ProfilePermission {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    match self {
      ProfilePermission::Admin => write!(f, "admin"),
    }
  }
}
