pub use Day::*;
pub use Month::*;

mod doomsday;
pub use self::doomsday::*;
mod day;
pub use self::day::*;
mod month;
pub use self::month::*;

/// The initial leap year (actually of 11 years).
///
/// This year marks the transition to the Gregorian calendar for the British Empire. Dates before
/// this year aren't supported by this doomsday algorithm.
pub const FIRST_LEAP_YEAR: usize = 1752;

/// Return the day of the week for a given year, month, day.
///
/// # Example
///
/// ```
/// use doomsday::*;
/// assert_eq!(Tuesday, day_of_week(2020, 04, 14));
/// ```
pub fn day_of_week(year: usize, month: usize, day: usize) -> Day {
    let dd = Doomsday(year);
    Day::from_anchor(dd.day() as usize + dd.anchor(Month::from(month)) + day)
}

/// Return true for given years which are defined to be leap years.
///
/// This is designed to help account for the fact there are not in fact a perfect 365 days in each
/// year.  Where, a _day_ is defined to be a complete rotation of the earth around it's axis (not
/// to be confused with a _solar day_), and a _year_ is defined to be the time it takes the earth
/// to complete a trip in it's orbit around the sun.
pub fn is_leap(year: usize) -> bool {
    assert!(year >= FIRST_LEAP_YEAR);

    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// Number of leap years since (not including) [`FIRST_LEAP_YEAR`].
pub fn leaps(year: usize) -> usize {
    assert!(year >= FIRST_LEAP_YEAR);

    let offset = year - FIRST_LEAP_YEAR;
    (offset / 4) - (offset / 100) + (offset / 400)
}


#[cfg(test)]
mod tests {
    use super::*;

    // Day of the week function (the main trick).

    #[test]
    fn test_day_of_week() {
        assert_eq!(Friday, day_of_week(1937, 03, 26));
        assert_eq!(Saturday, day_of_week(2020, 04, 11));
    }

    // Leap year functions (飛躍).

    #[test]
    fn test_is_leap() {
        assert!(is_leap(1752));
        assert!(is_leap(1756));
        assert!(is_leap(2020));
        assert!(!is_leap(1800));
        assert!(!is_leap(1900));
        assert!(is_leap(2000));
    }

    #[test]
    fn test_leaps() {
        assert_eq!(0,  leaps(1752));
        assert_eq!(1,  leaps(1756));
        assert_eq!(60, leaps(2000));
        assert_eq!(65, leaps(2020));
    }
}
