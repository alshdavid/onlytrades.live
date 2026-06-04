#![deny(unused_crate_dependencies)]
mod brotli;
mod gzip;

pub use self::brotli::*;
pub use self::gzip::*;
