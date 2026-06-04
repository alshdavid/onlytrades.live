use super::CTraderSocketClient;
use super::ProtoMessageParse;
use kit_ctrader_proto::*;
use super::types::*;

pub struct CTraderClosePositionOptions {
  pub account_id: i64,
  pub position_id: i64,
  pub volume: i64,
}

impl CTraderSocketClient {
  pub async fn close_position(
    &self,
    options: CTraderClosePositionOptions,
  ) -> anyhow::Result<ExecutionEvent> {
    let mut rx = self
      .send_and_subscribe(
        ProtoOaPayloadType::ProtoOaClosePositionReq,
        ProtoOaClosePositionReq {
          payload_type: None,
          ctid_trader_account_id: options.account_id,
          position_id: options.position_id,
          volume: options.volume,
        },
      )
      .await?;

    while let Some(result) = rx.recv().await {
      match result {
        Err(err) => return Err(err.into()),
        Ok(msg) => match msg.payload_type {
          2126 => {
            let ev =
              ExecutionEvent::try_from(msg.try_decode_body::<ProtoOaExecutionEvent>()?)?;
            match ev.execution_type {
              ExecutionType::OrderFilled => {
                return Ok(ev);
              }
              _ => continue,
            }
          }
          _ => {
            anyhow::bail!("Invalid message type for request")
          }
        },
      };
    }

    anyhow::bail!("request terminated early")
  }
}
