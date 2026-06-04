use kit_ctrader_proto::ProtoMessage;
use kit_ctrader_proto::{self as ct};
use prost::Message;

use super::types;
use crate::connection_proto::PayloadTypeExt;

pub enum CTraderRequestType {
  ApplicationAuthReq(Box<types::ApplicationAuthReq>),
  AccountAuthReq(Box<types::AccountAuthReq>),
  VersionReq(Box<types::VersionReq>),
  NewOrderReq(Box<types::NewOrderReq>),
  CancelOrderReq(Box<types::CancelOrderReq>),
  AmendOrderReq(Box<types::AmendOrderReq>),
  AmendPositionSltpReq(Box<types::AmendPositionSltpReq>),
  ClosePositionReq(Box<types::ClosePositionReq>),
  AssetListReq(Box<types::AssetListReq>),
  SymbolsListReq(Box<types::SymbolsListReq>),
  SymbolByIdReq(Box<types::SymbolByIdReq>),
  SymbolsForConversionReq(Box<types::SymbolsForConversionReq>),
  TraderReq(Box<types::TraderReq>),
  ReconcileReq(Box<types::ReconcileReq>),
  SubscribeSpotsReq(Box<types::SubscribeSpotsReq>),
  UnsubscribeSpotsReq(Box<types::UnsubscribeSpotsReq>),
  DealListReq(Box<types::DealListReq>),
  SubscribeLiveTrendbarReq(Box<types::SubscribeLiveTrendbarReq>),
  UnsubscribeLiveTrendbarReq(Box<types::UnsubscribeLiveTrendbarReq>),
  GetTrendbarsReq(Box<types::GetTrendbarsReq>),
  ExpectedMarginReq(Box<types::ExpectedMarginReq>),
  CashFlowHistoryListReq(Box<types::CashFlowHistoryListReq>),
  GetTickDataReq(Box<types::GetTickDataReq>),
  GetAccountListByAccessTokenReq(Box<types::GetAccountListByAccessTokenReq>),
  GetCtidProfileByTokenReq(Box<types::GetCtidProfileByTokenReq>),
  AssetClassListReq(Box<types::AssetClassListReq>),
  SubscribeDepthQuotesReq(Box<types::SubscribeDepthQuotesReq>),
  UnsubscribeDepthQuotesReq(Box<types::UnsubscribeDepthQuotesReq>),
  SymbolCategoryListReq(Box<types::SymbolCategoryListReq>),
  AccountLogoutReq(Box<types::AccountLogoutReq>),
  MarginCallListReq(Box<types::MarginCallListReq>),
  MarginCallUpdateReq(Box<types::MarginCallUpdateReq>),
  RefreshTokenReq(Box<types::RefreshTokenReq>),
  OrderListReq(Box<types::OrderListReq>),
  GetDynamicLeverageByIdReq(Box<types::GetDynamicLeverageByIdReq>),
  DealListByPositionIdReq(Box<types::DealListByPositionIdReq>),
  OrderDetailsReq(Box<types::OrderDetailsReq>),
  OrderListByPositionIdReq(Box<types::OrderListByPositionIdReq>),
  DealOffsetListReq(Box<types::DealOffsetListReq>),
  GetPositionUnrealizedPnLReq(Box<types::GetPositionUnrealizedPnLReq>),
}

impl Into<ProtoMessage> for CTraderRequestType {
  fn into(self) -> ProtoMessage {
    match self {
      Self::ApplicationAuthReq(req) => ProtoMessage {
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
      Self::AccountAuthReq(req) => ProtoMessage {
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
      Self::VersionReq(req) => ProtoMessage {
        payload_type: ct::ProtoOaPayloadType::ProtoOaVersionReq.as_u32(),
        payload: Some(ct::ProtoOaVersionReq { payload_type: None }.encode_to_vec()),
        client_msg_id: req.client_msg_id,
      },
      Self::NewOrderReq(req) => ProtoMessage {
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
      Self::CancelOrderReq(req) => ProtoMessage {
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
      Self::AmendOrderReq(req) => ProtoMessage {
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
      Self::AmendPositionSltpReq(req) => ProtoMessage {
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
      Self::ClosePositionReq(req) => ProtoMessage {
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
      Self::AssetListReq(req) => ProtoMessage {
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
      Self::SymbolsListReq(req) => ProtoMessage {
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
      Self::SymbolByIdReq(req) => ProtoMessage {
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
      Self::SymbolsForConversionReq(req) => ProtoMessage {
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
      Self::TraderReq(req) => ProtoMessage {
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
      Self::ReconcileReq(req) => ProtoMessage {
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
      Self::SubscribeSpotsReq(req) => ProtoMessage {
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
      Self::UnsubscribeSpotsReq(req) => ProtoMessage {
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
      Self::DealListReq(req) => ProtoMessage {
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
      Self::SubscribeLiveTrendbarReq(req) => ProtoMessage {
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
      Self::UnsubscribeLiveTrendbarReq(req) => ProtoMessage {
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
      Self::GetTrendbarsReq(req) => ProtoMessage {
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
      Self::ExpectedMarginReq(req) => ProtoMessage {
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
      Self::CashFlowHistoryListReq(req) => ProtoMessage {
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
      Self::GetTickDataReq(req) => ProtoMessage {
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
      Self::GetAccountListByAccessTokenReq(req) => ProtoMessage {
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
      Self::GetCtidProfileByTokenReq(req) => ProtoMessage {
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
      Self::AssetClassListReq(req) => ProtoMessage {
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
      Self::SubscribeDepthQuotesReq(req) => ProtoMessage {
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
      Self::UnsubscribeDepthQuotesReq(req) => ProtoMessage {
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
      Self::SymbolCategoryListReq(req) => ProtoMessage {
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
      Self::AccountLogoutReq(req) => ProtoMessage {
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
      Self::MarginCallListReq(req) => ProtoMessage {
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
      Self::MarginCallUpdateReq(req) => ProtoMessage {
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
      Self::RefreshTokenReq(req) => ProtoMessage {
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
      Self::OrderListReq(req) => ProtoMessage {
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
      Self::GetDynamicLeverageByIdReq(req) => ProtoMessage {
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
      Self::DealListByPositionIdReq(req) => ProtoMessage {
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
      Self::OrderDetailsReq(req) => ProtoMessage {
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
      Self::OrderListByPositionIdReq(req) => ProtoMessage {
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
      Self::DealOffsetListReq(req) => ProtoMessage {
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
      Self::GetPositionUnrealizedPnLReq(req) => ProtoMessage {
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
