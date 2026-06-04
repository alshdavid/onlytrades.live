use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Holiday {
  /// Unique ID of holiday.
  pub holiday_id: i64,
  /// Name of holiday.
  pub name: String,
  /// Description of holiday.
  pub description: Option<String>,
  /// Timezone used for holiday.
  pub schedule_time_zone: String,
  /// Amount of days from 1st Jan 1970, multiply it by 86400000 to get Unix time in milliseconds.
  pub holiday_date: i64,
  /// If TRUE, then the holiday happens each year.
  pub is_recurring: bool,
  /// Amount of seconds from 00:00:00 of the holiday day when holiday actually starts.
  pub start_second: Option<i32>,
  /// Amount of seconds from 00:00:00 of the holiday day when holiday actually finishes.
  pub end_second: Option<i32>,
}

impl From<kit_ctrader_proto::ProtoOaHoliday> for Holiday {
  fn from(holiday: kit_ctrader_proto::ProtoOaHoliday) -> Self {
    Holiday {
      holiday_id: holiday.holiday_id,
      name: holiday.name,
      description: holiday.description,
      schedule_time_zone: holiday.schedule_time_zone,
      holiday_date: holiday.holiday_date,
      is_recurring: holiday.is_recurring,
      start_second: holiday.start_second,
      end_second: holiday.end_second,
    }
  }
}
