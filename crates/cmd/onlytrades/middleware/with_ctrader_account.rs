use platform_models::CtraderAccountModel;
use platform_repository_turso::CtraderAccountRepository;
use uuid::Uuid;

pub async fn with_ctrader_account(
  _req: &uhttp::Request,
  res: &uhttp::Response,
  profile_id: &Uuid,
  account_id: &i64,
  ctrader_account_repository: &CtraderAccountRepository,
) -> anyhow::Result<Option<CtraderAccountModel>> {
  let accounts = ctrader_account_repository
    .find_by_profile_id(profile_id)
    .await?;

  // Validate that the account belongs to the profile
  let Some(account) = accounts.into_iter().find(|a| &a.account_id == account_id) else {
    res.write_head(uhttp::StatusCode::FORBIDDEN).await?;
    return Ok(None);
  };

  Ok(Some(account))
}
