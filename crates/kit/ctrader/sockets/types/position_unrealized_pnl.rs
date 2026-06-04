use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PositionUnrealizedPnL {
  /// The position ID.
  pub position_id: i64,
  /// The gross unrealized PnL of the position denoted in the account deposit currency.
  pub gross_unrealized_pn_l: i64,
  /// The net unrealized PnL of the position denoted in the account deposit currency. It does not include potential closing commission.
  pub net_unrealized_pn_l: i64,
}

impl From<kit_ctrader_proto::ProtoOaPositionUnrealizedPnL> for PositionUnrealizedPnL {
  fn from(pnl: kit_ctrader_proto::ProtoOaPositionUnrealizedPnL) -> Self {
    PositionUnrealizedPnL {
      position_id: pnl.position_id,
      gross_unrealized_pn_l: pnl.gross_unrealized_pn_l,
      net_unrealized_pn_l: pnl.net_unrealized_pn_l,
    }
  }
}
