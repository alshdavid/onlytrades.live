use kit_ctrader::CTraderClosePositionOptions;
use kit_ctrader::CTraderNewOrderOptions;
use kit_ctrader::CTraderReconcileOptions;
use kit_ctrader::{self};
use platform_models::TriggerModel;
use uhttp::Request;

use crate::ctx::Ctx;

#[derive(Debug)]
enum OrderType {
  Open(String),
  Close(String),
}

impl OrderType {
  fn parse_order_id(message: &str) -> Self {
    if let Some((_, order_id)) = message.split_once("Close entry(s) order ") {
      OrderType::Close(order_id.to_string())
    } else {
      OrderType::Open(message.to_string())
    }
  }
}

pub async fn webhooks_post_trading_view(
  req: Request,
  Ctx {
    ctrader_service,
    log_service,
    ..
  }: Ctx,
  body: super::WebhookPostRequest,
  TriggerModel {
    profile_id,
    ctrader_account_id,
    ..
  }: TriggerModel,
) -> anyhow::Result<()> {
  let Some(header) = req.headers().get("X-Forwarded-For") else {
    return Ok(());
  };

  let header = header.to_str()?;

  // IP is spoofed
  if header.contains(",") {
    return Ok(());
  }

  // Request is not from TradingView
  if header != "52.89.214.238"
    && header != "34.212.75.30"
    && header != "54.218.53.128"
    && header != "52.32.178.7"
  {
    return Ok(());
  }

  let position_size = body.position_size.parse::<i64>()?;

  let Some(trade_side) = kit_ctrader::TradeSide::from_str_name(&body.action) else {
    log_service.info(
      &format!("{}:trigger:{}", profile_id, body.trigger_id),
      &format!("ERROR: action must be 'buy' or 'sell', got {}", body.action),
    );
    return Ok(());
  };

  let connection_name = format!("{}:{}", profile_id, ctrader_account_id);

  let Some(symbol_id) = ctrader_service
    .symbols_list(&connection_name, &profile_id, &ctrader_account_id)
    .await?
    .get(&body.ticker)
    .cloned()
  else {
    log_service.info(
      &format!("{}:trigger:{}", profile_id, body.trigger_id),
      &format!("ERROR: Cannot find ticker {}", body.ticker),
    );
    return Ok(());
  };

  match OrderType::parse_order_id(&body.order_id) {
    OrderType::Open(order_id) => {
      ctrader_service
        .new_order(
          &connection_name,
          &profile_id,
          &ctrader_account_id,
          CTraderNewOrderOptions {
            ctid_trader_account_id: ctrader_account_id,
            symbol_id,
            order_type: kit_ctrader::OrderType::Market,
            trade_side,
            volume: position_size,
            limit_price: None,
            stop_price: None,
            time_in_force: Some(kit_ctrader::TimeInForce::ImmediateOrCancel),
            expiration_timestamp: None,
            stop_loss: None,
            take_profit: None,
            comment: Some(order_id),
            base_slippage_price: None,
            slippage_in_points: None,
            label: None,
            position_id: None,
            client_order_id: None,
            relative_stop_loss: None,
            relative_take_profit: None,
            guaranteed_stop_loss: None,
            trailing_stop_loss: None,
            stop_trigger_method: None,
          },
        )
        .await?;

      log_service.info(
        &format!("{}:trigger:{}", profile_id, body.trigger_id),
        &format!("OPEN {} x {}", body.ticker, position_size),
      );
      Ok(())
    }
    OrderType::Close(order_id) => {
      let reconciled = ctrader_service
        .reconcile(
          &connection_name,
          &profile_id,
          &ctrader_account_id,
          CTraderReconcileOptions {
            account_id: ctrader_account_id,
          },
        )
        .await?;

      let Some(position) = ('block: {
        for position in reconciled.position {
          if let Some(comment) = &position.trade_data.comment
            && comment == &order_id
          {
            break 'block Some(position);
          }
        }
        None
      }) else {
        log_service.info(
          &format!("{}:trigger:{}", profile_id, body.trigger_id),
          &format!("Cannot find order {}", order_id),
        );
        return Ok(());
      };

      ctrader_service
        .close_position(
          &connection_name,
          &profile_id,
          &ctrader_account_id,
          CTraderClosePositionOptions {
            account_id: ctrader_account_id,
            position_id: position.position_id,
            volume: position_size,
          },
        )
        .await?;

      log_service.info(
        &format!("{}:trigger:{}", profile_id, body.trigger_id),
        &format!("CLOSE {} x {}", body.ticker, position_size),
      );
      Ok(())
    }
  }
}
