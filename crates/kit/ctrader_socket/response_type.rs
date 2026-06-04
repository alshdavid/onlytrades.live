use kit_ctrader_proto::{self as ct};

use super::types;
use crate::connection_proto::ProtoError;
use crate::connection_proto::ProtoMessageParse;

pub enum CTraderResponseType {
  ExecutionEvent(Box<types::ExecutionEvent>),
  ReconcileRes(types::ReconcileRes),
  ApplicationAuthRes(types::ApplicationAuthRes),
  AccountAuthRes(types::AccountAuthRes),
  VersionRes(types::VersionRes),
  ErrorRes(types::ErrorRes),
  SpotEvent(types::SpotEvent),
  AssetListRes(types::AssetListRes),
  SymbolsListRes(types::SymbolsListRes),
  SymbolByIdRes(types::SymbolByIdRes),
  SymbolsForConversionRes(types::SymbolsForConversionRes),
  TraderRes(types::TraderRes),
  SubscribeSpotsRes(types::SubscribeSpotsRes),
  UnsubscribeSpotsRes(types::UnsubscribeSpotsRes),
  DealListRes(types::DealListRes),
  SubscribeLiveTrendbarRes(types::SubscribeLiveTrendbarRes),
  UnsubscribeLiveTrendbarRes(types::UnsubscribeLiveTrendbarRes),
  GetTrendbarsRes(types::GetTrendbarsRes),
  ExpectedMarginRes(types::ExpectedMarginRes),
  CashFlowHistoryListRes(types::CashFlowHistoryListRes),
  GetTickDataRes(types::GetTickDataRes),
  GetAccountListByAccessTokenRes(types::GetAccountListByAccessTokenRes),
  GetCtidProfileByTokenRes(types::GetCtidProfileByTokenRes),
  AssetClassListRes(types::AssetClassListRes),
  SubscribeDepthQuotesRes(types::SubscribeDepthQuotesRes),
  UnsubscribeDepthQuotesRes(types::UnsubscribeDepthQuotesRes),
  SymbolCategoryListRes(types::SymbolCategoryListRes),
  AccountLogoutRes(types::AccountLogoutRes),
  MarginCallListRes(types::MarginCallListRes),
  MarginCallUpdateRes(types::MarginCallUpdateRes),
  RefreshTokenRes(types::RefreshTokenRes),
  OrderListRes(types::OrderListRes),
  GetDynamicLeverageByIdRes(types::GetDynamicLeverageByIdRes),
  DealListByPositionIdRes(types::DealListByPositionIdRes),
  OrderDetailsRes(types::OrderDetailsRes),
  OrderListByPositionIdRes(types::OrderListByPositionIdRes),
  DealOffsetListRes(types::DealOffsetListRes),
  GetPositionUnrealizedPnLRes(types::GetPositionUnrealizedPnLRes),
}

impl TryFrom<ct::ProtoMessage> for CTraderResponseType {
  type Error = ProtoError;

  fn try_from(value: ct::ProtoMessage) -> Result<Self, Self::Error> {
    let map_err = |_err: anyhow::Error| {
      ProtoError::PayloadParseError(value.client_msg_id.clone(), value.payload_type)
    };

    match value.try_payload_type() {
      // Events (no client_msg_id)
      Ok(ct::ProtoOaPayloadType::ProtoOaExecutionEvent) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaExecutionEvent>()?;
        let res = types::ExecutionEvent::try_from(proto_res).map_err(map_err)?;
        Ok(CTraderResponseType::ExecutionEvent(Box::new(res)))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaSpotEvent) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaSpotEvent>()?;
        let res = types::SpotEvent::try_from(proto_res).map_err(map_err)?;
        Ok(CTraderResponseType::SpotEvent(res))
      }
      // Responses with client_msg_id and From conversion
      Ok(ct::ProtoOaPayloadType::ProtoOaApplicationAuthRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaApplicationAuthRes>()?;
        let mut res = types::ApplicationAuthRes::from(proto_res);
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::ApplicationAuthRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaAccountAuthRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaAccountAuthRes>()?;
        let mut res = types::AccountAuthRes::from(proto_res);
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::AccountAuthRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaVersionRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaVersionRes>()?;
        let mut res = types::VersionRes::from(proto_res);
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::VersionRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaErrorRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaErrorRes>()?;
        let mut res = types::ErrorRes::from(proto_res);
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::ErrorRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaAssetListRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaAssetListRes>()?;
        let mut res = types::AssetListRes::from(proto_res);
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::AssetListRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaSymbolsListRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaSymbolsListRes>()?;
        let mut res = types::SymbolsListRes::from(proto_res);
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::SymbolsListRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaSymbolsForConversionRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaSymbolsForConversionRes>()?;
        let mut res = types::SymbolsForConversionRes::from(proto_res);
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::SymbolsForConversionRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaTraderRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaTraderRes>()?;
        let mut res = types::TraderRes::from(proto_res);
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::TraderRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaSubscribeSpotsRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaSubscribeSpotsRes>()?;
        let mut res = types::SubscribeSpotsRes::from(proto_res);
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::SubscribeSpotsRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaUnsubscribeSpotsRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaUnsubscribeSpotsRes>()?;
        let mut res = types::UnsubscribeSpotsRes::from(proto_res);
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::UnsubscribeSpotsRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaExpectedMarginRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaExpectedMarginRes>()?;
        let mut res = types::ExpectedMarginRes::from(proto_res);
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::ExpectedMarginRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaSubscribeLiveTrendbarRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaSubscribeLiveTrendbarRes>()?;
        let mut res = types::SubscribeLiveTrendbarRes::from(proto_res);
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::SubscribeLiveTrendbarRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaUnsubscribeLiveTrendbarRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaUnsubscribeLiveTrendbarRes>()?;
        let mut res = types::UnsubscribeLiveTrendbarRes::from(proto_res);
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::UnsubscribeLiveTrendbarRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaSubscribeDepthQuotesRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaSubscribeDepthQuotesRes>()?;
        let mut res = types::SubscribeDepthQuotesRes::from(proto_res);
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::SubscribeDepthQuotesRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaUnsubscribeDepthQuotesRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaUnsubscribeDepthQuotesRes>()?;
        let mut res = types::UnsubscribeDepthQuotesRes::from(proto_res);
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::UnsubscribeDepthQuotesRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaAccountLogoutRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaAccountLogoutRes>()?;
        let mut res = types::AccountLogoutRes::from(proto_res);
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::AccountLogoutRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaMarginCallListRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaMarginCallListRes>()?;
        let mut res = types::MarginCallListRes::from(proto_res);
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::MarginCallListRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaMarginCallUpdateRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaMarginCallUpdateRes>()?;
        let mut res = types::MarginCallUpdateRes::from(proto_res);
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::MarginCallUpdateRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaRefreshTokenRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaRefreshTokenRes>()?;
        let mut res = types::RefreshTokenRes::from(proto_res);
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::RefreshTokenRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaDealOffsetListRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaDealOffsetListRes>()?;
        let mut res = types::DealOffsetListRes::from(proto_res);
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::DealOffsetListRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaAssetClassListRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaAssetClassListRes>()?;
        let mut res = types::AssetClassListRes::from(proto_res);
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::AssetClassListRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaGetCtidProfileByTokenRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaGetCtidProfileByTokenRes>()?;
        let mut res = types::GetCtidProfileByTokenRes::from(proto_res);
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::GetCtidProfileByTokenRes(res))
      }
      // Payload type names that differ from proto struct names (From conversion)
      Ok(ct::ProtoOaPayloadType::ProtoOaGetTickdataRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaGetTickDataRes>()?;
        let mut res = types::GetTickDataRes::from(proto_res);
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::GetTickDataRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaSymbolCategoryRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaSymbolCategoryListRes>()?;
        let mut res = types::SymbolCategoryListRes::from(proto_res);
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::SymbolCategoryListRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaGetDynamicLeverageRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaGetDynamicLeverageByIdRes>()?;
        let mut res = types::GetDynamicLeverageByIdRes::from(proto_res);
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::GetDynamicLeverageByIdRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaGetPositionUnrealizedPnlRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaGetPositionUnrealizedPnLRes>()?;
        let mut res = types::GetPositionUnrealizedPnLRes::from(proto_res);
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::GetPositionUnrealizedPnLRes(res))
      }
      // Responses with client_msg_id and TryFrom conversion
      Ok(ct::ProtoOaPayloadType::ProtoOaReconcileRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaReconcileRes>()?;
        let mut res = types::ReconcileRes::try_from(proto_res).map_err(map_err)?;
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::ReconcileRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaSymbolByIdRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaSymbolByIdRes>()?;
        let mut res = types::SymbolByIdRes::try_from(proto_res).map_err(map_err)?;
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::SymbolByIdRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaDealListRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaDealListRes>()?;
        let mut res = types::DealListRes::try_from(proto_res).map_err(map_err)?;
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::DealListRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaGetTrendbarsRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaGetTrendbarsRes>()?;
        let mut res = types::GetTrendbarsRes::try_from(proto_res).map_err(map_err)?;
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::GetTrendbarsRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaCashFlowHistoryListRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaCashFlowHistoryListRes>()?;
        let mut res = types::CashFlowHistoryListRes::try_from(proto_res).map_err(map_err)?;
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::CashFlowHistoryListRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaGetAccountsByAccessTokenRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaGetAccountListByAccessTokenRes>()?;
        let mut res =
          types::GetAccountListByAccessTokenRes::try_from(proto_res).map_err(map_err)?;
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::GetAccountListByAccessTokenRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaOrderListRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaOrderListRes>()?;
        let mut res = types::OrderListRes::try_from(proto_res).map_err(map_err)?;
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::OrderListRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaDealListByPositionIdRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaDealListByPositionIdRes>()?;
        let mut res = types::DealListByPositionIdRes::try_from(proto_res).map_err(map_err)?;
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::DealListByPositionIdRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaOrderDetailsRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaOrderDetailsRes>()?;
        let mut res = types::OrderDetailsRes::try_from(proto_res).map_err(map_err)?;
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::OrderDetailsRes(res))
      }
      Ok(ct::ProtoOaPayloadType::ProtoOaOrderListByPositionIdRes) => {
        let proto_res = value.try_decode_payload::<ct::ProtoOaOrderListByPositionIdRes>()?;
        let mut res = types::OrderListByPositionIdRes::try_from(proto_res).map_err(map_err)?;
        res.client_msg_id = value.client_msg_id;
        Ok(CTraderResponseType::OrderListByPositionIdRes(res))
      }
      Ok(payload_type) => Err(ProtoError::UnknownPayloadType(
        value.client_msg_id,
        payload_type as i32,
      )),
      Err(err) => Err(err),
    }
  }
}
