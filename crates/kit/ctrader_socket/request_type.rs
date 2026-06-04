use kit_ctrader_proto::ProtoMessage;
use kit_ctrader_proto::{self as ct};
use prost::Message;

use super::types;
use crate::connection_proto::PayloadTypeExt;

pub enum CTraderRequestType {
  ApplicationAuthReq(types::ApplicationAuthReq),
  AccountAuthReq(types::AccountAuthReq),
  VersionReq(types::VersionReq),
  NewOrderReq(types::NewOrderReq),
  CancelOrderReq(types::CancelOrderReq),
  AmendOrderReq(types::AmendOrderReq),
  AmendPositionSltpReq(types::AmendPositionSltpReq),
  ClosePositionReq(types::ClosePositionReq),
  AssetListReq(types::AssetListReq),
  SymbolsListReq(types::SymbolsListReq),
  SymbolByIdReq(types::SymbolByIdReq),
  SymbolsForConversionReq(types::SymbolsForConversionReq),
  TraderReq(types::TraderReq),
  ReconcileReq(types::ReconcileReq),
  SubscribeSpotsReq(types::SubscribeSpotsReq),
  UnsubscribeSpotsReq(types::UnsubscribeSpotsReq),
  DealListReq(types::DealListReq),
  SubscribeLiveTrendbarReq(types::SubscribeLiveTrendbarReq),
  UnsubscribeLiveTrendbarReq(types::UnsubscribeLiveTrendbarReq),
  GetTrendbarsReq(types::GetTrendbarsReq),
  ExpectedMarginReq(types::ExpectedMarginReq),
  CashFlowHistoryListReq(types::CashFlowHistoryListReq),
  GetTickDataReq(types::GetTickDataReq),
  GetAccountListByAccessTokenReq(types::GetAccountListByAccessTokenReq),
  GetCtidProfileByTokenReq(types::GetCtidProfileByTokenReq),
  AssetClassListReq(types::AssetClassListReq),
  SubscribeDepthQuotesReq(types::SubscribeDepthQuotesReq),
  UnsubscribeDepthQuotesReq(types::UnsubscribeDepthQuotesReq),
  SymbolCategoryListReq(types::SymbolCategoryListReq),
  AccountLogoutReq(types::AccountLogoutReq),
  MarginCallListReq(types::MarginCallListReq),
  MarginCallUpdateReq(types::MarginCallUpdateReq),
  RefreshTokenReq(types::RefreshTokenReq),
  OrderListReq(types::OrderListReq),
  GetDynamicLeverageByIdReq(types::GetDynamicLeverageByIdReq),
  DealListByPositionIdReq(types::DealListByPositionIdReq),
  OrderDetailsReq(types::OrderDetailsReq),
  OrderListByPositionIdReq(types::OrderListByPositionIdReq),
  DealOffsetListReq(types::DealOffsetListReq),
  GetPositionUnrealizedPnLReq(types::GetPositionUnrealizedPnLReq),
}

impl From<CTraderRequestType> for ProtoMessage {
  fn from(val: CTraderRequestType) -> Self {
    match val {
      CTraderRequestType::ApplicationAuthReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaApplicationAuthReq.as_u32(),
        payload: Some(
          ct::ProtoOaApplicationAuthReq {
            payload_type: None,
            client_id: req.client_id,
            client_secret: req.client_secret,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::AccountAuthReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaAccountAuthReq.as_u32(),
        payload: Some(
          ct::ProtoOaAccountAuthReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            access_token: req.access_token,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::VersionReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaVersionReq.as_u32(),
        payload: Some(ct::ProtoOaVersionReq { payload_type: None }.encode_to_vec()),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::NewOrderReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaNewOrderReq.as_u32(),
        payload: Some(
          ct::ProtoOaNewOrderReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            symbol_id: req.symbol_id,
            order_type: req.order_type.into(),
            trade_side: req.trade_side.into(),
            volume: req.volume,
            limit_price: req.limit_price,
            stop_price: req.stop_price,
            time_in_force: req.time_in_force.map(|v| v.into()),
            expiration_timestamp: req.expiration_timestamp,
            stop_loss: req.stop_loss,
            take_profit: req.take_profit,
            comment: req.comment,
            base_slippage_price: req.base_slippage_price,
            slippage_in_points: req.slippage_in_points,
            label: req.label,
            position_id: req.position_id,
            client_order_id: req.client_order_id,
            relative_stop_loss: req.relative_stop_loss,
            relative_take_profit: req.relative_take_profit,
            guaranteed_stop_loss: req.guaranteed_stop_loss,
            trailing_stop_loss: req.trailing_stop_loss,
            stop_trigger_method: req.stop_trigger_method.map(|v| v.into()),
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::CancelOrderReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaCancelOrderReq.as_u32(),
        payload: Some(
          ct::ProtoOaCancelOrderReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            order_id: req.order_id,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::AmendOrderReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaAmendOrderReq.as_u32(),
        payload: Some(
          ct::ProtoOaAmendOrderReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            order_id: req.order_id,
            volume: req.volume,
            limit_price: req.limit_price,
            stop_price: req.stop_price,
            expiration_timestamp: req.expiration_timestamp,
            stop_loss: req.stop_loss,
            take_profit: req.take_profit,
            slippage_in_points: req.slippage_in_points,
            relative_stop_loss: req.relative_stop_loss,
            relative_take_profit: req.relative_take_profit,
            guaranteed_stop_loss: req.guaranteed_stop_loss,
            trailing_stop_loss: req.trailing_stop_loss,
            stop_trigger_method: req.stop_trigger_method.map(|v| v.into()),
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::AmendPositionSltpReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaAmendPositionSltpReq.as_u32(),
        payload: Some(
          ct::ProtoOaAmendPositionSltpReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            position_id: req.position_id,
            stop_loss: req.stop_loss,
            take_profit: req.take_profit,
            guaranteed_stop_loss: req.guaranteed_stop_loss,
            trailing_stop_loss: req.trailing_stop_loss,
            stop_loss_trigger_method: req.stop_loss_trigger_method.map(|v| v.into()),
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::ClosePositionReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaClosePositionReq.as_u32(),
        payload: Some(
          ct::ProtoOaClosePositionReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            position_id: req.position_id,
            volume: req.volume,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::AssetListReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaAssetListReq.as_u32(),
        payload: Some(
          ct::ProtoOaAssetListReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::SymbolsListReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaSymbolsListReq.as_u32(),
        payload: Some(
          ct::ProtoOaSymbolsListReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            include_archived_symbols: req.include_archived_symbols,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::SymbolByIdReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaSymbolByIdReq.as_u32(),
        payload: Some(
          ct::ProtoOaSymbolByIdReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            symbol_id: req.symbol_id,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::SymbolsForConversionReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaSymbolsForConversionReq.as_u32(),
        payload: Some(
          ct::ProtoOaSymbolsForConversionReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            first_asset_id: req.first_asset_id,
            last_asset_id: req.last_asset_id,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::TraderReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaTraderReq.as_u32(),
        payload: Some(
          ct::ProtoOaTraderReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::ReconcileReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaReconcileReq.as_u32(),
        payload: Some(
          ct::ProtoOaReconcileReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            return_protection_orders: req.return_protection_orders,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::SubscribeSpotsReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaSubscribeSpotsReq.as_u32(),
        payload: Some(
          ct::ProtoOaSubscribeSpotsReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            symbol_id: req.symbol_id,
            subscribe_to_spot_timestamp: req.subscribe_to_spot_timestamp,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::UnsubscribeSpotsReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaUnsubscribeSpotsReq.as_u32(),
        payload: Some(
          ct::ProtoOaUnsubscribeSpotsReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            symbol_id: req.symbol_id,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::DealListReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaDealListReq.as_u32(),
        payload: Some(
          ct::ProtoOaDealListReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            from_timestamp: req.from_timestamp,
            to_timestamp: req.to_timestamp,
            max_rows: req.max_rows,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::SubscribeLiveTrendbarReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaSubscribeLiveTrendbarReq.as_u32(),
        payload: Some(
          ct::ProtoOaSubscribeLiveTrendbarReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            period: req.period.into(),
            symbol_id: req.symbol_id,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::UnsubscribeLiveTrendbarReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaUnsubscribeLiveTrendbarReq.as_u32(),
        payload: Some(
          ct::ProtoOaUnsubscribeLiveTrendbarReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            period: req.period.into(),
            symbol_id: req.symbol_id,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::GetTrendbarsReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaGetTrendbarsReq.as_u32(),
        payload: Some(
          ct::ProtoOaGetTrendbarsReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            from_timestamp: req.from_timestamp,
            to_timestamp: req.to_timestamp,
            period: req.period.into(),
            symbol_id: req.symbol_id,
            count: req.count,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::ExpectedMarginReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaExpectedMarginReq.as_u32(),
        payload: Some(
          ct::ProtoOaExpectedMarginReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            symbol_id: req.symbol_id,
            volume: req.volume,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::CashFlowHistoryListReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaCashFlowHistoryListReq.as_u32(),
        payload: Some(
          ct::ProtoOaCashFlowHistoryListReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            from_timestamp: req.from_timestamp,
            to_timestamp: req.to_timestamp,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::GetTickDataReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaGetTickdataReq.as_u32(),
        payload: Some(
          ct::ProtoOaGetTickDataReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            symbol_id: req.symbol_id,
            r#type: req.r#type.into(),
            from_timestamp: req.from_timestamp,
            to_timestamp: req.to_timestamp,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::GetAccountListByAccessTokenReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaGetAccountsByAccessTokenReq.as_u32(),
        payload: Some(
          ct::ProtoOaGetAccountListByAccessTokenReq {
            payload_type: None,
            access_token: req.access_token,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::GetCtidProfileByTokenReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaGetCtidProfileByTokenReq.as_u32(),
        payload: Some(
          ct::ProtoOaGetCtidProfileByTokenReq {
            payload_type: None,
            access_token: req.access_token,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::AssetClassListReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaAssetClassListReq.as_u32(),
        payload: Some(
          ct::ProtoOaAssetClassListReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::SubscribeDepthQuotesReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaSubscribeDepthQuotesReq.as_u32(),
        payload: Some(
          ct::ProtoOaSubscribeDepthQuotesReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            symbol_id: req.symbol_id,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::UnsubscribeDepthQuotesReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaUnsubscribeDepthQuotesReq.as_u32(),
        payload: Some(
          ct::ProtoOaUnsubscribeDepthQuotesReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            symbol_id: req.symbol_id,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::SymbolCategoryListReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaSymbolCategoryReq.as_u32(),
        payload: Some(
          ct::ProtoOaSymbolCategoryListReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::AccountLogoutReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaAccountLogoutReq.as_u32(),
        payload: Some(
          ct::ProtoOaAccountLogoutReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::MarginCallListReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaMarginCallListReq.as_u32(),
        payload: Some(
          ct::ProtoOaMarginCallListReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::MarginCallUpdateReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaMarginCallUpdateReq.as_u32(),
        payload: Some(
          ct::ProtoOaMarginCallUpdateReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            margin_call: ct::ProtoOaMarginCall {
              margin_call_type: req.margin_call.margin_call_type,
              margin_level_threshold: req.margin_call.margin_level_threshold,
              utc_last_update_timestamp: req.margin_call.utc_last_update_timestamp,
            },
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::RefreshTokenReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaRefreshTokenReq.as_u32(),
        payload: Some(
          ct::ProtoOaRefreshTokenReq {
            payload_type: None,
            refresh_token: req.refresh_token,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::OrderListReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaOrderListReq.as_u32(),
        payload: Some(
          ct::ProtoOaOrderListReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            from_timestamp: req.from_timestamp,
            to_timestamp: req.to_timestamp,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::GetDynamicLeverageByIdReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaGetDynamicLeverageReq.as_u32(),
        payload: Some(
          ct::ProtoOaGetDynamicLeverageByIdReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            leverage_id: req.leverage_id,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::DealListByPositionIdReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaDealListByPositionIdReq.as_u32(),
        payload: Some(
          ct::ProtoOaDealListByPositionIdReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            position_id: req.position_id,
            from_timestamp: req.from_timestamp,
            to_timestamp: req.to_timestamp,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::OrderDetailsReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaOrderDetailsReq.as_u32(),
        payload: Some(
          ct::ProtoOaOrderDetailsReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            order_id: req.order_id,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::OrderListByPositionIdReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaOrderListByPositionIdReq.as_u32(),
        payload: Some(
          ct::ProtoOaOrderListByPositionIdReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            position_id: req.position_id,
            from_timestamp: req.from_timestamp,
            to_timestamp: req.to_timestamp,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::DealOffsetListReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaDealOffsetListReq.as_u32(),
        payload: Some(
          ct::ProtoOaDealOffsetListReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
            deal_id: req.deal_id,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
      CTraderRequestType::GetPositionUnrealizedPnLReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaGetPositionUnrealizedPnlReq.as_u32(),
        payload: Some(
          ct::ProtoOaGetPositionUnrealizedPnLReq {
            payload_type: None,
            ctid_trader_account_id: req.ctid_trader_account_id,
          }
          .encode_to_vec(),
        ),
        client_msg_id: req.client_msg_id,
      },
    }
  }
}
