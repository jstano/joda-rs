#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Instant(pub std::time::Instant);

impl From<std::time::Instant> for Instant { fn from(t: std::time::Instant) -> Self { Instant(t) } }
impl From<Instant> for std::time::Instant { fn from(w: Instant) -> Self { w.0 } }
impl Instant {
    pub fn inner(&self) -> &std::time::Instant { &self.0 }

    // java.time.Instant.now()
    pub fn now() -> Self { Instant(std::time::Instant::now()) }
}
