use kit_ctrader_proto::ProtoOaGetAccountListByAccessTokenRes;
use num_enum::TryFromPrimitive;

use super::ClientPermissionScope;
use super::CtidTraderAccount;

/// * Response to the ProtoOAGetAccountListByAccessTokenReq request.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct GetAccountListByAccessTokenRes {
  pub client_msg_id: Option<String>,
  /// The Access Token issued for providing access to the Trader's Account.
  pub access_token: String,
  /// SCOPE_VIEW, SCOPE_TRADE.
  pub permission_scope: Option<ClientPermissionScope>,
  /// The list of the accounts.
  pub ctid_trader_account: Vec<CtidTraderAccount>,
}

impl TryFrom<ProtoOaGetAccountListByAccessTokenRes> for GetAccountListByAccessTokenRes {
  type Error = anyhow::Error;

  fn try_from(value: ProtoOaGetAccountListByAccessTokenRes) -> Result<Self, Self::Error> {
    Ok(GetAccountListByAccessTokenRes {
      client_msg_id: None,
      access_token: value.access_token,
      permission_scope: match value.permission_scope {
        Some(permission_scope) => {
          Some(ClientPermissionScope::try_from_primitive(permission_scope)?)
        }
        None => None,
      },
      ctid_trader_account: value
        .ctid_trader_account
        .into_iter()
        .map(CtidTraderAccount::from)
        .collect(),
    })
  }
}
