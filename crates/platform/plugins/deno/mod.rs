use std::sync::Arc;

use kit_ctrader::CTraderSubscribeSpotsOptions;
use kit_ctrader::ProtoMessageParse;
use kit_ctrader::SpotEvent;
use kit_ctrader::messages;
use platform_ctrader_service::CTraderService;
use platform_process::DenoIPCMessage;
use platform_process::DenoInstance;
use platform_process::DenoProcessOptions;
use serde::Deserialize;
use uuid::Uuid;

pub static PLUGIN_CODE: &str = include_str!("plugin.ts");

pub struct DenoPlugin {
  handle_read: tokio::task::JoinHandle<anyhow::Result<()>>,
  handle_write: tokio::task::JoinHandle<anyhow::Result<()>>,
  pub instance: DenoInstance,
}

impl std::fmt::Debug for DenoPlugin {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.debug_struct("DenoPlugin").finish()
  }
}

impl DenoPlugin {
  pub async fn new(
    handler: &str,
    options: DenoProcessOptions,
    profile_id: &Uuid,
    account_id: &i64,
    ctrader_service: &Arc<CTraderService>,
  ) -> anyhow::Result<Self> {
    let code = format!(
      "
      {}
      {}
      try {{
        await handler(new Context(globalThis.conn));
      }} catch (err) {{
        console.error(err) 
      }}
    ",
      PLUGIN_CODE, handler
    );

    let mut instance = DenoInstance::new(options, &code).await?;

    let connection_name = Arc::new(format!("{}:{}:{}", profile_id, account_id, instance.id));
    let mut plugin_rx = instance.reader()?;
    let plugin_tx = instance.writer();

    let handle_read: tokio::task::JoinHandle<anyhow::Result<()>> = tokio::spawn({
      let ctrader_service = Arc::clone(ctrader_service);
      let profile_id = *profile_id;
      let account_id = *account_id;
      let plugin_tx = plugin_tx.clone();
      let connection_name = connection_name.clone();

      async move {
        let mut sub = ctrader_service
          .subscribe(&connection_name, &profile_id, &account_id)
          .await?;

        while let Some(Ok(msg)) = sub.recv().await {
          if msg.payload_type == 2131 {
            let payload = msg.try_decode_body::<messages::ProtoOaSpotEvent>()?;
            let payload = SpotEvent::try_from(payload)?;
            let response = serde_json::to_value(&payload)?;
            let msg = DenoIPCMessage("spot_event".to_string(), Some(response), None);
            let msg = serde_json::to_vec(&msg)?;
            let _ = plugin_tx.send(msg);
          }
        }

        Ok(())
      }
    });

    let handle_write: tokio::task::JoinHandle<anyhow::Result<()>> = tokio::spawn({
      let ctrader_service = Arc::clone(ctrader_service);
      let profile_id = *profile_id;
      let account_id = *account_id;
      let connection_name = connection_name.clone();

      async move {
        while let Some(bytes) = plugin_rx.recv().await {
          let DenoIPCMessage(event_name, body, id) =
            serde_json::from_slice::<DenoIPCMessage>(&bytes)?;

          match event_name.as_str() {
            "symbols_list" => {
              let result = ctrader_service
                .symbols_list(&connection_name, &profile_id, &account_id)
                .await?;

              let response = serde_json::to_value(&result)?;
              let msg = DenoIPCMessage("".to_string(), Some(response), id);
              let msg = serde_json::to_vec(&msg)?;
              let _ = plugin_tx.send(msg);
            }
            "subscribe_spots" => {
              #[derive(Debug, Deserialize)]
              struct Response {
                symbols: Vec<i64>,
              }

              let body = serde_json::from_value::<Response>(body.unwrap()).unwrap();

              ctrader_service
                .subscribe_spots(
                  &connection_name,
                  &profile_id,
                  &account_id,
                  CTraderSubscribeSpotsOptions {
                    account_id,
                    symbol_id: body.symbols,
                    subscribe_to_spot_timestamp: None,
                  },
                )
                .await?;

              let msg = DenoIPCMessage("".to_string(), None, id);
              let msg = serde_json::to_vec(&msg)?;
              let _ = plugin_tx.send(msg);
            }
            _ => {
              eprintln!("Unknown event")
            }
          }
        }

        Ok(())
      }
    });

    Ok(Self {
      handle_read,
      handle_write,
      instance,
    })
  }
}

impl Drop for DenoPlugin {
  fn drop(&mut self) {
    self.handle_read.abort();
    self.handle_write.abort();
  }
}
