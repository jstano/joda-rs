#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DayOfWeek {
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
    Sunday = 7,
}

impl DayOfWeek {
    pub fn of(value: i32) -> Self {
        match value {
            1 => DayOfWeek::Monday,
            2 => DayOfWeek::Tuesday,
            3 => DayOfWeek::Wednesday,
            4 => DayOfWeek::Thursday,
            5 => DayOfWeek::Friday,
            6 => DayOfWeek::Saturday,
            7 => DayOfWeek::Sunday,
            _ => panic!("invalid day-of-week value (1-7)"),
        }
    }

    pub fn value(self) -> i32 { self as i32 }

    pub fn plus(self, days: i64) -> Self {
        // Normalize to 0..6, add, wrap, then map back to 1..7
        let zero_based = (self.value() - 1) as i64;
        let wrapped = (zero_based + (days % 7) + 7) % 7; // handle negative
        DayOfWeek::of((wrapped as i32) + 1)
    }

    pub fn minus(self, days: i64) -> Self { self.plus(-(days)) }

    pub fn from_date(date: crate::LocalDate) -> Self {
        let d: time::Date = date.into();
        let wd = d.weekday();
        DayOfWeek::from(wd)
    }
}

impl From<time::Weekday> for DayOfWeek {
    fn from(w: time::Weekday) -> Self {
        match w {
            time::Weekday::Monday => DayOfWeek::Monday,
            time::Weekday::Tuesday => DayOfWeek::Tuesday,
            time::Weekday::Wednesday => DayOfWeek::Wednesday,
            time::Weekday::Thursday => DayOfWeek::Thursday,
            time::Weekday::Friday => DayOfWeek::Friday,
            time::Weekday::Saturday => DayOfWeek::Saturday,
            time::Weekday::Sunday => DayOfWeek::Sunday,
        }
    }
}

impl From<DayOfWeek> for time::Weekday {
    fn from(d: DayOfWeek) -> Self {
        match d {
            DayOfWeek::Monday => time::Weekday::Monday,
            DayOfWeek::Tuesday => time::Weekday::Tuesday,
            DayOfWeek::Wednesday => time::Weekday::Wednesday,
            DayOfWeek::Thursday => time::Weekday::Thursday,
            DayOfWeek::Friday => time::Weekday::Friday,
            DayOfWeek::Saturday => time::Weekday::Saturday,
            DayOfWeek::Sunday => time::Weekday::Sunday,
        }
    }
}

impl core::fmt::Display for DayOfWeek {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            DayOfWeek::Monday => "MONDAY",
            DayOfWeek::Tuesday => "TUESDAY",
            DayOfWeek::Wednesday => "WEDNESDAY",
            DayOfWeek::Thursday => "THURSDAY",
            DayOfWeek::Friday => "FRIDAY",
            DayOfWeek::Saturday => "SATURDAY",
            DayOfWeek::Sunday => "SUNDAY",
        };
        f.write_str(s)
    }
}
