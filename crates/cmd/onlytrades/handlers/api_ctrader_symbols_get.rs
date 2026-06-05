use serde::Deserialize;
use uhttp::*;

use crate::ctx::Ctx;

#[derive(Debug, Deserialize)]
struct Query {
  account_id: Option<i64>,
}

pub async fn api_ctrader_symbols_get(
  req: uhttp::Request,
  mut res: uhttp::Response,
  Ctx {
    ctrader_service,
    profile,
    ..
  }: Ctx,
) -> uhttp::Result<()> {
  let Some(query_str) = req.uri().query() else {
    res.write_head(uhttp::StatusCode::BAD_REQUEST).await?;
    return Ok(());
  };

  let query = serde_urlencoded::from_str::<Query>(query_str)?;

  let Some(account_id) = query.account_id else {
    res.write_head(uhttp::StatusCode::BAD_REQUEST).await?;
    return Ok(());
  };

  todo!();
  // let connection_name = format!("{}:{}", profile.id, account_id);
  // let symbols = ctrader_service
  //   .symbols_list(&connection_name, &profile.id, &account_id)
  //   .await?;

  // let mut symbols = symbols.into_keys().collect::<Vec<String>>();
  // symbols.sort();

  // let body = serde_json::to_vec(&symbols)?;
  // res.write_all(&body).await?;
  // res.header().add("Content-Type", "application/json").await?;
  // res.write_head(StatusCode::OK).await?;
  // Ok(())
}
