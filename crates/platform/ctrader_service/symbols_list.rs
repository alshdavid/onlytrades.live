// use std::collections::HashMap;

// use kit_ctrader::CTraderSymbolsListOptions;
// use uuid::Uuid;

// use super::CTraderService;

// impl CTraderService {
//   pub async fn symbols_list(
//     &self,
//     connection_name: &str,
//     profile_id: &Uuid,
//     account_id: &i64,
//   ) -> anyhow::Result<HashMap<String, i64>> {
//     let (conn, account) = self
//       .get_or_connect_socket(connection_name, profile_id, account_id)
//       .await?;

//     conn
//       .symbols_list(CTraderSymbolsListOptions {
//         account_id: account.account_id,
//       })
//       .await
//   }
// }
