use std::str::FromStr;
use time::OffsetDateTime;
use time_tz::timezones::get_by_name;
use time_tz::{timezones, OffsetDateTimeExt, TimeZone};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ZoneId(&'static str);

impl ZoneId {
    /// A constant representing the UTC (Coordinated Universal Time) time zone.
    ///
    /// This constant is an instance of `ZoneId` with the time zone set to `"UTC"`.
    /// It can be used whenever the UTC time zone needs to be specified explicitly
    /// within the context of time zone or date-time operations.
    ///
    /// # Example
    /// ```
    /// println!("Time zone: {}", ZoneId::UTC);
    /// ```
    pub const UTC: ZoneId = ZoneId("UTC");

    /// Retrieves the system's default time zone and converts it into a `ZoneId` instance.
    ///
    /// # Returns
    /// A `ZoneId` representing the system's current time zone.
    ///
    /// # Errors
    /// This function will panic if the underlying system time zone cannot be determined.
    /// Ensure that your operating system is properly configured to provide time zone information.
    ///
    /// # Panics
    /// - Panics with the message "Failed to determine system time zone; please check your OS configuration"
    ///   if the time zone cannot be retrieved.
    ///
    /// # Example
    /// ```rust
    /// let default_zone = ZoneId::system_default();
    /// println!("System default time zone: {:?}", default_zone);
    /// ```
    ///
    /// Note: This function relies on the `time_tz` crate to get the system time zone.
    pub fn system_default() -> Self {
        let tz = time_tz::system::get_timezone()
            .expect("Failed to determine system time zone; please check your OS configuration");
        ZoneId::of(tz.name())
    }

    /// Attempts to create a `ZoneId` instance from the given time zone identifier.
    ///
    /// # Parameters
    /// - `id`: A string slice (`&'static str`) representing the identifier of the time zone.
    ///
    /// # Returns
    /// - `Ok(ZoneId)`: If a valid time zone corresponding to the provided `id` is found.
    /// - `Err(&'static str)`: If the provided `id` does not correspond to any known time zone.
    ///   The returned error message will be `"Unknown time zone"`.
    ///
    /// # Example
    /// ```
    /// let zone_id = ZoneId::try_of("America/New_York");
    /// assert!(zone_id.is_ok());
    ///
    /// let invalid_zone_id = ZoneId::try_of("Invalid/Time_Zone");
    /// assert!(invalid_zone_id.is_err());
    /// ```
    ///
    /// # Errors
    /// This function will return an error if the given `id` does not match any known time zone
    /// in the `timezones` module.
    ///
    /// # Notes
    /// The `timezones::get_by_name` function is used internally to verify the validity of the provided `id`.
    /// Ensure that the `timezones` module is properly configured and contains the expected data
    /// before calling this function.
    pub fn try_of(id: &'static str) -> Result<Self, &'static str> {
        if timezones::get_by_name(id).is_some() {
            Ok(ZoneId(id))
        } else {
            Err("Unknown time zone")
        }
    }

    /// Retrieves the identifier associated with the current instance.
    ///
    /// # Returns
    /// A string slice (`&str`) representing the identifier.
    ///
    /// # Example
    /// ```
    /// use joda_rs::ZoneId;
    /// let newYork = ZoneId::try_of("America/New_York");
    /// assert_eq!(newYork.unwrap().id(), "America/New_York");
    /// ```
    pub fn id(&self) -> &str {
        self.0
    }

    /// Converts the current timezone represented by the instance into its corresponding offset in seconds.
    ///
    /// # Returns
    ///
    /// An `i32` value representing the offset, in seconds, from UTC for the associated timezone.
    ///
    /// # Panics
    ///
    /// This method will panic if the timezone name stored in `self.0` does not exist
    /// or if it cannot be resolved to a valid timezone offset using the `timezones::get_by_name` method.
    ///
    /// # Examples
    ///
    /// ```
    /// let zone_id = ZoneId::of("America/New_York");
    /// let offset = zone_id.to_offset();
    /// assert_eq!(offset, -5);
    /// ```
    pub fn to_offset(self) -> i32 {
        let tz = get_by_name(self.0).expect("unknown timezone");
        let now_utc = OffsetDateTime::now_utc();
        let now_tz = now_utc.to_timezone(tz);
        now_tz.offset().whole_seconds()
    }

    const fn of(id: &'static str) -> Self {
        ZoneId(id)
    }
}

impl FromStr for ZoneId {
    type Err = &'static str;

    /// Parse a string slice into a `ZoneId`.
    ///
    /// # Example
    /// ```rust
    /// use joda_rs::ZoneId;
    ///
    /// let z: ZoneId = "America/New_York".parse().unwrap();
    /// assert_eq!(z.id(), "America/New_York");
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(tz) = get_by_name(s) {
            Ok(ZoneId(tz.name()))
        } else {
            Err("Unknown time zone")
        }
    }
}
