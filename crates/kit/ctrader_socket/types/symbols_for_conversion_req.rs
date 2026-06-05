use kit_ctrader_proto::ProtoOaSymbolsForConversionReq;

/// * Request for getting a conversion chain between two assets that consists of several symbols. Use when no direct quote is available.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct SymbolsForConversionReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// The ID of the firs asset in the conversation chain. e.g.: for EUR/USD the firstAssetId is EUR ID and lastAssetId is USD ID.
  pub first_asset_id: i64,
  /// The ID of the last asset in the conversation chain. e.g.: for EUR/USD the firstAssetId is EUR ID and lastAssetId is USD ID.
  pub last_asset_id: i64,
}

impl From<ProtoOaSymbolsForConversionReq> for SymbolsForConversionReq {
  fn from(value: ProtoOaSymbolsForConversionReq) -> Self {
    SymbolsForConversionReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      first_asset_id: value.first_asset_id,
      last_asset_id: value.last_asset_id,
    }
  }
}
