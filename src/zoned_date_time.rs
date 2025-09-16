// We still base on time::OffsetDateTime for storage, until tz support is added.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZonedDateTime(pub time::OffsetDateTime);

impl From<time::OffsetDateTime> for ZonedDateTime { fn from(t: time::OffsetDateTime) -> Self { ZonedDateTime(t) } }
impl From<ZonedDateTime> for time::OffsetDateTime { fn from(w: ZonedDateTime) -> Self { w.0 } }
impl ZonedDateTime {
    pub fn inner(&self) -> &time::OffsetDateTime { &self.0 }

    // java.time.ZonedDateTime.now() - UTC only placeholder
    pub fn now_utc() -> Self { ZonedDateTime(time::OffsetDateTime::now_utc()) }

    // java.time.ZonedDateTime.of(LocalDateTime, ZoneId)
    pub fn of(ldt: crate::LocalDateTime, zone: crate::ZoneId) -> Self {
        // Only UTC supported explicitly; other ids behave as UTC for now
        let _ = zone; // unused currently; placeholder for future tz logic
        let pdt: time::PrimitiveDateTime = ldt.into();
        ZonedDateTime(pdt.assume_utc())
    }
}
