use std::collections::HashMap;

use kit_ctrader_proto::*;

use super::CTraderSocketClient;

pub struct CTraderSymbolsListOptions {
  pub account_id: i64,
}

impl CTraderSocketClient {
  pub async fn symbols_list(
    &self,
    options: CTraderSymbolsListOptions,
  ) -> anyhow::Result<HashMap<String, i64>> {
    let result = self
      .send_and_receive_oneshot::<ProtoOaSymbolsListRes>(
        ProtoOaPayloadType::ProtoOaSymbolsListReq,
        ProtoOaSymbolsListReq {
          payload_type: None,
          ctid_trader_account_id: options.account_id,
          include_archived_symbols: None,
        },
      )
      .await?;

    let mut map = HashMap::<String, i64>::new();

    for symbol in result.symbol {
      map.insert(symbol.symbol_name().to_string(), symbol.symbol_id);
    }

    Ok(map)
  }
}
