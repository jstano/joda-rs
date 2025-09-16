#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ZoneId(pub &'static str);
impl ZoneId {
    pub const UTC: ZoneId = ZoneId("UTC");
    pub const fn of(id: &'static str) -> Self { ZoneId(id) }
}
