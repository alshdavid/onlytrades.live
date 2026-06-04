use serde::Deserialize;
use uhttp::*;
use uuid::Uuid;

use crate::ctx::Ctx;

#[derive(Debug, Deserialize)]
pub struct WebhookPostRequest {
  /// ID for trigger
  pub trigger_id: Uuid,
  /// "{{order_id}" | "Close entry(s) order {{order_id}}"
  pub order_id: String,
  /// "buy" | "sell"
  pub action: String,
  /// "XAUUSD"
  pub ticker: String,
  /// number - exact number of units
  /// e.g.
  ///   "XAUUSD": 10000     = 1 Lot
  ///   "US500":  100       = 1 Lot
  ///   "EURUSD": 10000000  = 1 Lot
  pub position_size: String,
  #[allow(unused)]
  /// ISO Formatted date "2026-05-18T07:46:26Z"
  pub timestamp: String,
}

pub async fn webhooks_post(
  mut req: uhttp::Request,
  res: uhttp::Response,
  ctx: Ctx,
) -> uhttp::Result<()> {
  // WRITE RESPONSE EARLY as ACK. Use Logs to determine result
  res.write_head(StatusCode::NO_CONTENT).await?;

  let Ok(body) = uhttp::body::json::<WebhookPostRequest>(&mut req.body()).await else {
    ctx
      .log_service
      .info("triggers", "ERROR: Unable to parse body");
    return Ok(());
  };

  let Some(trigger) = ctx.trigger_repository.get_by_id(&body.trigger_id).await? else {
    ctx.log_service.info(
      "triggers",
      &format!("ERROR: Trigger does not exist {}", body.trigger_id),
    );
    return Ok(());
  };

  match trigger.platform.as_str() {
    "trading-view" => {
      super::webhooks_post_trading_view::webhooks_post_trading_view(req, ctx, body, trigger).await
    }
    trigger_platform => {
      ctx.log_service.info(
        "triggers",
        &format!("ERROR: Unknown trigger platform {}", trigger_platform),
      );
      Ok(())
    }
  }
}
