#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

impl Month {
    pub fn of(value: i32) -> Self {
        match value {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => panic!("invalid month value (1-12)"),
        }
    }

    pub fn value(self) -> i32 { self as i32 }

    pub fn length(self, leap_year: bool) -> u8 {
        match self {
            Month::January => 31,
            Month::February => if leap_year { 29 } else { 28 },
            Month::March => 31,
            Month::April => 30,
            Month::May => 31,
            Month::June => 30,
            Month::July => 31,
            Month::August => 31,
            Month::September => 30,
            Month::October => 31,
            Month::November => 30,
            Month::December => 31,
        }
    }

    pub fn min_length(self) -> u8 { self.length(false) }

    pub fn max_length(self) -> u8 { self.length(true) }

    pub fn plus(self, months: i64) -> Self {
        let zero = (self.value() - 1) as i64;
        let wrapped = (zero + (months % 12) + 12) % 12;
        Month::of((wrapped as i32) + 1)
    }

    pub fn minus(self, months: i64) -> Self { self.plus(-(months)) }
}

impl From<time::Month> for Month {
    fn from(m: time::Month) -> Self {
        match m {
            time::Month::January => Month::January,
            time::Month::February => Month::February,
            time::Month::March => Month::March,
            time::Month::April => Month::April,
            time::Month::May => Month::May,
            time::Month::June => Month::June,
            time::Month::July => Month::July,
            time::Month::August => Month::August,
            time::Month::September => Month::September,
            time::Month::October => Month::October,
            time::Month::November => Month::November,
            time::Month::December => Month::December,
        }
    }
}

impl From<Month> for time::Month {
    fn from(m: Month) -> Self {
        match m {
            Month::January => time::Month::January,
            Month::February => time::Month::February,
            Month::March => time::Month::March,
            Month::April => time::Month::April,
            Month::May => time::Month::May,
            Month::June => time::Month::June,
            Month::July => time::Month::July,
            Month::August => time::Month::August,
            Month::September => time::Month::September,
            Month::October => time::Month::October,
            Month::November => time::Month::November,
            Month::December => time::Month::December,
        }
    }
}

impl core::fmt::Display for Month {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            Month::January => "JANUARY",
            Month::February => "FEBRUARY",
            Month::March => "MARCH",
            Month::April => "APRIL",
            Month::May => "MAY",
            Month::June => "JUNE",
            Month::July => "JULY",
            Month::August => "AUGUST",
            Month::September => "SEPTEMBER",
            Month::October => "OCTOBER",
            Month::November => "NOVEMBER",
            Month::December => "DECEMBER",
        };
        f.write_str(s)
    }
}
