#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZoneOffset(pub time::UtcOffset);

impl From<time::UtcOffset> for ZoneOffset { fn from(t: time::UtcOffset) -> Self { ZoneOffset(t) } }
impl From<ZoneOffset> for time::UtcOffset { fn from(w: ZoneOffset) -> Self { w.0 } }
impl ZoneOffset {
    pub fn inner(&self) -> &time::UtcOffset { &self.0 }

    // java.time.ZoneOffset.ofHours
    pub fn of_hours(hours: i8) -> Self {
        ZoneOffset(time::UtcOffset::from_hms(hours, 0, 0).expect("invalid offset hours"))
    }
    // java.time.ZoneOffset.ofHoursMinutes
    pub fn of_hours_minutes(hours: i8, minutes: i8) -> Self {
        ZoneOffset(time::UtcOffset::from_hms(hours, minutes, 0).expect("invalid offset"))
    }
}
