use std::collections::HashSet;

use chrono::DateTime;
use chrono::Utc;
use libsql::Connection;
use libsql::Row;
use libsql::params;
use platform_models::CtraderAccountModel;
use platform_models::CtraderTokenModel;
use platform_models::IdentityModel;
use platform_models::ProfileDataViewModel;
use platform_models::ProfilePermission;
use uuid::Uuid;

/// Read Only view of a complete profile
pub struct ProfileDataView {
  db: Connection,
}

impl ProfileDataView {
  pub fn new(db: &Connection) -> Self {
    Self { db: db.clone() }
  }

  #[deprecated]
  #[allow(unused)]
  /// Finds a profile by its GUID using an isolated read transaction.
  pub async fn find_by_id(
    &self,
    profile_id: &Uuid,
  ) -> anyhow::Result<Option<ProfileDataViewModel>> {
    let profile_id_str = profile_id.to_string();

    // Open a transaction to lock in a point-in-time data snapshot
    let tx = self.db.transaction().await?;

    let sql = "SELECT id, email, created_at FROM profiles WHERE id = ?1;";
    let mut rows = tx.query(sql, params![profile_id_str.clone()]).await?;

    if let Some(row) = rows.next().await? {
      let mut view_model = self.map_row(row)?;

      // Pass the transaction reference down to fetch child collections safely
      self.populate_relations_tx(&tx, &mut view_model).await?;

      // Commit closes the snapshot cleanly
      tx.commit().await?;
      Ok(Some(view_model))
    } else {
      tx.commit().await?;
      Ok(None)
    }
  }

  #[deprecated]
  #[allow(unused)]
  /// Finds a profile by email using an isolated read transaction.
  pub async fn find_by_email(
    &self,
    email: &str,
  ) -> anyhow::Result<Option<ProfileDataViewModel>> {
    let tx = self.db.transaction().await?;

    let sql = "SELECT id, email, created_at FROM profiles WHERE email = ?1;";
    let mut rows = tx.query(sql, params![email]).await?;

    if let Some(row) = rows.next().await? {
      let mut view_model = self.map_row(row)?;

      self.populate_relations_tx(&tx, &mut view_model).await?;

      tx.commit().await?;
      Ok(Some(view_model))
    } else {
      tx.commit().await?;
      Ok(None)
    }
  }

  /// Finds a profile using an external identity string (e.g., Auth0 sub payload).
  pub async fn find_by_identity(
    &self,
    identity_sub: &str,
  ) -> anyhow::Result<Option<ProfileDataViewModel>> {
    // Open a transaction to lock in a point-in-time snapshot across lookups
    let tx = self.db.transaction().await?;

    // Step 1: Resolve the profile_id linked to this identity sub
    let lookup_sql = "SELECT profile_id FROM identities WHERE sub = ?1;";
    let mut lookup_rows = tx.query(lookup_sql, params![identity_sub]).await?;

    if let Some(lookup_row) = lookup_rows.next().await? {
      let profile_id_str: String = lookup_row.get(0)?;

      // Step 2: Fetch the primary profile record using the found profile_id
      let profile_sql = "SELECT id, email, created_at FROM profiles WHERE id = ?1;";
      let mut profile_rows = tx.query(profile_sql, params![profile_id_str]).await?;

      if let Some(profile_row) = profile_rows.next().await? {
        let mut view_model = self.map_row(profile_row)?;

        // Step 3: Hydrate all related vectors safely under the same transaction
        self.populate_relations_tx(&tx, &mut view_model).await?;

        tx.commit().await?;
        return Ok(Some(view_model));
      }
    }

    // Explicitly close the transaction if no matching identity or profile was found
    tx.commit().await?;
    Ok(None)
  }

  /// Maps a database row back into our clean ProfileModel boilerplate fields.
  fn map_row(
    &self,
    row: Row,
  ) -> anyhow::Result<ProfileDataViewModel> {
    let id_str: String = row.get(0)?;
    let email: String = row.get(1)?;

    let created_at: DateTime<Utc> = match row.get::<String>(2) {
      Ok(s) => s.parse::<DateTime<Utc>>().unwrap_or_else(|_| Utc::now()),
      Err(_) => Utc::now(),
    };

    Ok(ProfileDataViewModel {
      id: Uuid::parse_str(&id_str)?,
      email,
      created_at,
      identities: Vec::new(),
      ctrader_accounts: Vec::new(),
      ctrader_tokens: None,
      permissions: HashSet::new(),
    })
  }

  /// Internal strategy executing isolated sequential reads on the active transaction handle.
  async fn populate_relations_tx(
    &self,
    tx: &libsql::Transaction,
    profile: &mut ProfileDataViewModel,
  ) -> anyhow::Result<()> {
    let profile_id_str = profile.id.to_string();

    // 1. Fetch Identities
    let mut identity_rows = tx
      .query(
        "SELECT sub, provider, last_login FROM identities WHERE profile_id = ?1;",
        params![profile_id_str.clone()],
      )
      .await?;

    while let Some(row) = identity_rows.next().await? {
      let last_login: DateTime<Utc> = match row.get::<String>(2) {
        Ok(s) => s.parse::<DateTime<Utc>>().unwrap_or_else(|_| Utc::now()),
        Err(_) => Utc::now(),
      };
      profile.identities.push(IdentityModel {
        sub: row.get(0)?,
        profile_id: profile.id,
        provider: row.get(1)?,
        last_login,
      });
    }

    // 2. Fetch cTrader Accounts
    let mut account_rows = tx
      .query(
        r#"SELECT 
            account_id, account_number, live, broker_name, broker_title, 
            deposit_currency, trader_account_type, leverage, leverage_in_cents, 
            deleted, account_status, swap_free, money_digits 
           FROM ctrader_accounts WHERE profile_id = ?1;"#,
        params![profile_id_str.clone()],
      )
      .await?;

    while let Some(row) = account_rows.next().await? {
      profile.ctrader_accounts.push(CtraderAccountModel {
        account_id: row.get(0)?,
        account_number: row.get::<i64>(1)? as u64,
        live: row.get::<i64>(2)? != 0,
        broker_name: row.get(3)?,
        broker_title: row.get(4)?,
        deposit_currency: row.get(5)?,
        trader_account_type: row.get(6)?,
        leverage: row.get::<i64>(7)? as u32,
        leverage_in_cents: row.get::<i64>(8)? as u64,
        deleted: row.get::<i64>(9)? != 0,
        account_status: row.get(10)?,
        swap_free: row.get::<i64>(11)? != 0,
        money_digits: row.get::<i64>(12)? as u32,
      });
    }

    // 3. Fetch cTrader Tokens
    let mut token_rows = tx.query(
        "SELECT token_id, refresh_token, access_token, access_token_expires_at FROM ctrader_tokens WHERE profile_id = ?1;",
        params![profile_id_str.clone()]
    ).await?;

    if let Some(row) = token_rows.next().await? {
      let expires_at_unix: i64 = row.get(3)?;
      let token_id_str: String = row.get(0)?;
      profile.ctrader_tokens = Some(CtraderTokenModel {
        token_id: Uuid::parse_str(&token_id_str).unwrap_or_default(),
        refresh_token: row.get(1)?,
        access_token: row.get(2)?,
        access_token_expires_at: DateTime::from_timestamp(expires_at_unix, 0)
          .unwrap_or_else(Utc::now),
      });
    }

    // 4. Fetch Permissions
    let mut perm_rows = tx
      .query(
        "SELECT role FROM profile_permissions WHERE profile_id = ?1;",
        params![profile_id_str],
      )
      .await?;

    while let Some(row) = perm_rows.next().await? {
      profile
        .permissions
        .insert(ProfilePermission::try_from(row.get::<String>(0)?)?);
    }

    Ok(())
  }
}
