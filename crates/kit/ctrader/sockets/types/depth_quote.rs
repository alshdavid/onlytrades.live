use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DepthQuote {
  /// Quote ID.
  pub id: u64,
  /// Quote size in cents.
  pub size: u64,
  /// Bid price for bid quotes.
  pub bid: Option<u64>,
  /// Ask price for ask quotes.
  pub ask: Option<u64>,
}

impl From<super::super::messages::ProtoOaDepthQuote> for DepthQuote {
  fn from(quote: super::super::messages::ProtoOaDepthQuote) -> Self {
    DepthQuote {
      id: quote.id,
      size: quote.size,
      bid: quote.bid,
      ask: quote.ask,
    }
  }
}
