use libsql::Connection;
use libsql::Row;
use libsql::params;
use platform_models::CtraderAccountModel;
use uuid::Uuid;

pub struct CtraderAccountRepository {
  db: Connection,
}

impl CtraderAccountRepository {
  pub fn new(db: &Connection) -> Self {
    Self { db: db.clone() }
  }

  pub async fn init(&self) -> anyhow::Result<()> {
    let sql = r#"
      CREATE TABLE IF NOT EXISTS ctrader_accounts (
          account_id            INTEGER PRIMARY KEY,  -- i64
          profile_id            TEXT NOT NULL,
          account_number        INTEGER NOT NULL,     -- u64 -> INTEGER
          live                  INTEGER NOT NULL,     -- bool -> INTEGER (1 or 0)
          broker_name           TEXT NOT NULL,
          broker_title          TEXT NOT NULL,
          deposit_currency      TEXT NOT NULL,
          trader_account_type   TEXT NOT NULL,
          leverage              INTEGER NOT NULL,     -- u32 -> INTEGER
          leverage_in_cents     INTEGER NOT NULL,     -- u64 -> INTEGER
          deleted               INTEGER NOT NULL,     -- bool -> INTEGER (1 or 0)
          account_status        TEXT NOT NULL,
          swap_free             INTEGER NOT NULL,     -- bool -> INTEGER (1 or 0)
          money_digits          INTEGER NOT NULL,     -- u32 -> INTEGER
          FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE
      );
    "#;

    self.db.execute(sql, ()).await?;

    Ok(())
  }

  /// Creates or updates a cTrader account.
  pub async fn upsert(
    &self,
    account: &CtraderAccountModel,
    profile_id: &Uuid,
  ) -> anyhow::Result<()> {
    let sql = r#"
      INSERT INTO ctrader_accounts (
        account_id, profile_id, account_number, live,
        broker_name, broker_title, deposit_currency, trader_account_type,
        leverage, leverage_in_cents, deleted, account_status,
        swap_free, money_digits
      ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
      ON CONFLICT(account_id) DO UPDATE SET
        account_number = EXCLUDED.account_number,
        live = EXCLUDED.live,
        broker_name = EXCLUDED.broker_name,
        broker_title = EXCLUDED.broker_title,
        deposit_currency = EXCLUDED.deposit_currency,
        trader_account_type = EXCLUDED.trader_account_type,
        leverage = EXCLUDED.leverage,
        leverage_in_cents = EXCLUDED.leverage_in_cents,
        deleted = EXCLUDED.deleted,
        account_status = EXCLUDED.account_status,
        swap_free = EXCLUDED.swap_free,
        money_digits = EXCLUDED.money_digits;
    "#;

    self
      .db
      .execute(
        sql,
        params![
          account.account_id, // i64 -> String/TEXT
          profile_id.to_string(),
          account.account_number as i64, // u64 -> Safe cast for SQLite INTEGER
          if account.live { 1 } else { 0 },
          account.broker_name.clone(),
          account.broker_title.clone(),
          account.deposit_currency.clone(),
          account.trader_account_type.clone(),
          account.leverage as i64,
          account.leverage_in_cents as i64,
          if account.deleted { 1 } else { 0 },
          account.account_status.clone(),
          if account.swap_free { 1 } else { 0 },
          account.money_digits as i64,
        ],
      )
      .await?;

    Ok(())
  }

  /// Finds all ctrader accounts linked to a single profile.
  pub async fn find_by_profile_id(
    &self,
    profile_id: &Uuid,
  ) -> anyhow::Result<Vec<CtraderAccountModel>> {
    let sql = "SELECT * FROM ctrader_accounts WHERE profile_id = ?1;";
    let mut rows = self.db.query(sql, params![profile_id.to_string()]).await?;

    let mut accounts = Vec::new();
    while let Some(row) = rows.next().await? {
      accounts.push(self.map_row(row)?);
    }

    Ok(accounts)
  }

  /// Performs a soft-delete (or hard delete if preferred) on an account.
  pub async fn delete(
    &self,
    account_id: &i64,
  ) -> anyhow::Result<()> {
    let sql = "DELETE FROM ctrader_accounts WHERE account_id = ?1;";
    self
      .db
      .execute(sql, params![account_id.to_string()])
      .await?;
    Ok(())
  }

  /// Helper to map a database row to the CtraderAccountModel struct.
  fn map_row(
    &self,
    row: Row,
  ) -> anyhow::Result<CtraderAccountModel> {
    Ok(CtraderAccountModel {
      account_id: row.get(0)?,
      account_number: {
        let account_number_raw: i64 = row.get(2)?;
        account_number_raw as u64
      },
      live: {
        let live_raw: i64 = row.get(3)?;
        live_raw != 0
      },
      broker_name: row.get(4)?,
      broker_title: row.get(5)?,
      deposit_currency: row.get(6)?,
      trader_account_type: row.get(7)?,
      leverage: {
        let leverage_raw: i64 = row.get(8)?;
        leverage_raw as u32
      },
      leverage_in_cents: {
        let leverage_in_cents_raw: i64 = row.get(9)?;
        leverage_in_cents_raw as u64
      },
      deleted: {
        let deleted_raw: i64 = row.get(10)?;
        deleted_raw != 0
      },
      account_status: row.get(11)?,
      swap_free: {
        let swap_free_raw: i64 = row.get(12)?;
        swap_free_raw != 0
      },
      money_digits: {
        let money_digits_raw: i64 = row.get(13)?;
        money_digits_raw as u32
      },
    })
  }
}
