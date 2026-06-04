use super::CTraderSocketClient;
use super::messages;

pub struct CTraderSubscribeSpotsOptions {
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub account_id: i64,
  /// Unique identifier of the Symbol in cTrader platform.
  pub symbol_id: Vec<i64>,
  /// If TRUE you will also receive the timestamp in ProtoOASpotEvent.
  pub subscribe_to_spot_timestamp: Option<bool>,
}

impl CTraderSocketClient {
  pub async fn subscribe_spots(
    &self,
    options: CTraderSubscribeSpotsOptions,
  ) -> anyhow::Result<()> {
    let mut _rx = self
      .send_and_subscribe(
        messages::ProtoOaPayloadType::ProtoOaSubscribeSpotsReq,
        messages::ProtoOaSubscribeSpotsReq {
          payload_type: None,
          ctid_trader_account_id: options.account_id,
          symbol_id: options.symbol_id,
          subscribe_to_spot_timestamp: options.subscribe_to_spot_timestamp,
        },
      )
      .await?;

    // while let Some(result) = rx.recv().await {
    //   match result {
    //     Err(err) => return Err(err.into()),
    //     Ok(msg) => {
    //       dbg!(&msg);
    //     } // match msg.payload_type {

    //       //   2126 => {
    //       //     let ev =
    //       //       ExecutionEvent::try_from(msg.try_decode_body::<messages::ProtoOaExecutionEvent>()?)?;
    //       //     match ev.execution_type {
    //       //       ExecutionType::OrderFilled => {
    //       //         // return Ok(ev);
    //       //       }
    //       //       _ => continue,
    //       //     }
    //       //   }
    //       //   _ => {
    //       //     anyhow::bail!("Invalid message type for request")
    //       //   }
    //       // },
    //   };
    // }

    Ok(())
  }
}
