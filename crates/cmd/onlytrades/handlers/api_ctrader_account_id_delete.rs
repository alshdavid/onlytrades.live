use crate::ctx::Ctx;

pub async fn api_ctrader_account_id_delete(
  req: uhttp::Request,
  res: uhttp::Response,
  Ctx {
    ctrader_account_repository,
    profile,
    ..
  }: Ctx,
) -> uhttp::Result<()> {
  let Some(ctrader_account_id) = req.url_param("id") else {
    res.write_head(uhttp::StatusCode::BAD_REQUEST).await?;
    return Ok(());
  };

  let ctrader_account_id = ctrader_account_id.parse::<i64>()?;

  if ctrader_account_repository
    .find_by_profile_id(&profile.id)
    .await?
    .iter()
    .find(|a| a.account_id == ctrader_account_id)
    .is_none()
  {
    res.write_head(uhttp::StatusCode::UNAUTHORIZED).await?;
    return Ok(());
  }

  ctrader_account_repository
    .delete(&ctrader_account_id)
    .await?;

  res.write_head(uhttp::StatusCode::NO_CONTENT).await?;

  Ok(())
}
