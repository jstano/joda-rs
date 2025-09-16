#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OffsetDateTime(pub time::OffsetDateTime);

impl From<time::OffsetDateTime> for OffsetDateTime { fn from(t: time::OffsetDateTime) -> Self { OffsetDateTime(t) } }
impl From<OffsetDateTime> for time::OffsetDateTime { fn from(w: OffsetDateTime) -> Self { w.0 } }
impl OffsetDateTime {
    pub fn inner(&self) -> &time::OffsetDateTime { &self.0 }

    // java.time.OffsetDateTime.now()
    pub fn now_utc() -> Self { OffsetDateTime(time::OffsetDateTime::now_utc()) }

    // java.time.OffsetDateTime.of(LocalDateTime, ZoneOffset)
    pub fn of(ldt: crate::LocalDateTime, offset: crate::ZoneOffset) -> Self {
        let pdt: time::PrimitiveDateTime = ldt.into();
        let off: time::UtcOffset = offset.into();
        OffsetDateTime(pdt.assume_offset(off))
    }
}
