use std::fmt;

pub use Day::*;
pub use Month::*;

/// Seven days a week, derived from the celestial objects around us all.
#[derive(Debug, Eq, PartialEq)]
pub enum Day {
    /// Moon
    Monday,
    /// Mars
    Tuesday,
    /// Mercury
    Wednsday,
    /// Jupiter
    Thursday,
    /// Venus
    Friday,
    /// Saturn
    Saturday,
    /// Sol (the sun itself)
    Sunday,
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Twelve months a year, roughly derived from the phases of the moon, and the
/// seasons.
///
/// I'll add here that, a system of 13 months could work very nicely. Every
/// month would have 28 days, and there would only be a single day left over,
/// for new years.
#[derive(Debug, Eq, PartialEq)]
pub enum Month {
    /// Janus, beginnings and endings.
    January,
    /// Februus, celebrations of Rome.
    February,
    /// Mars, Roman god of war.
    March,
    /// Aprilis, opening of the flowers.
    April,
    /// Maiores, meaning "major" or older.
    May,
    /// Iuniores, meaning "junior" or the younger.
    June,
    /// Julius, named in 44 BCE, the year of his assassination.
    July,
    /// Augustus, a Roman emperor.
    August,
    /// Septem, meaning seven.
    September,
    /// Octo, meaning eight.
    October,
    /// Novem, meaning nine.
    November,
    /// Decem, meaning ten.
    December,

    // It would seem that the fall of Rome occurred before they could finish
    // the naming of the months to fruition.
}

/// The initial leap year (actually of 11 years).
///
/// This year marks the transition to the Gregorian calendar for the British
/// Empire. Dates before this year aren't supported by this doomsday algorithm.
pub const FIRST_LEAP_YEAR: usize = 1752;

/// Behold the doomsday of a year.
///
/// Every year has a single **doomsday**, for example `Doomsday(2020)` is
/// `Saturday`. Additionally, each month has fixed day number for which
/// doomsday lies, for example, in `April` doomsday is always on the 4th. Given
/// these two peices of information, it's possible to quickly determain the day
/// of any date.
///
/// Years before `FIRST_LEAP_YEAR` aren't supported.
#[derive(Debug)]
pub struct Doomsday(pub usize);

/// TLFs (Two letter functions).
impl Doomsday {
    #[inline]
    pub fn on(&self) -> Day { self.day() }

    /// Indicate the starting doomsday for this month, in this year.
    /// TODO: Format as 4th? 2nd?
    #[inline]
    pub fn of(&self, month: Month) -> usize { self.date(month) }
}

impl Doomsday {
    /// Create a new doomsday for a given year.
    /// TODO: Random or something?
    #[inline]
    pub fn new(year: usize) -> Doomsday {
        Doomsday(year)
    }

    /// Return the **day number**, (we'll call a date) of the first doomsday in a given month.
    ///
    /// This is the main utility of this crate. Notice how there are really
    /// only a few mnemonics to memorize here:
    ///
    /// - 4/4, 6/6, 8/8, 10/10, and 12/12
    /// - 9 to 5 at 7-Eleven
    /// - The last day of February (twice)
    /// - 3 for three years, then 4 on the forth.
    ///
    /// These tricks are listed again below for each appropriate month.
    pub fn date(&self, month: Month) -> usize {
        match month {
            // 3 for three years, then 4 on the forth.
            January => if is_leap(self.0) { 4 } else { 3 },
            // Always the last day of the month.
            February => if is_leap(self.0) { 29 } else { 28 },
            // Also the last day of February, interestingly.
            March => 0,
            // 4/4
            April => 4,
            // 9 to 5 (reversed)
            May => 9,
            // 6/6
            June => 6,
            // 7-Eleven
            July => 11,
            // 8/8
            August => 8,
            // 9 to 5
            September => 5,
            // 10/10
            October => 10,
            // 7-Eleven (reversed)
            November => 7,
            // 12/12
            December => 12,
        }
    }

    /// Return the **day of the week** which doomsday lies on in a given
    /// year.
    pub fn day(&self) -> Day {
        day_from_anchor(self.0 + leaps(self.0))
    }
}

/// The display of doomsday is day, one for every week if you wish.
impl fmt::Display for Doomsday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.day())
    }
}

/// Return the day of the week for a given year, month, day.
///
/// # Example
///
/// ```
/// use doomsday::*;
/// assert_eq!(Tuesday, day_of_the_week(2020, 04, 14));
/// ```
pub fn day_of_the_week(year: usize, month: usize, day: usize) -> Day {
    let dd = Doomsday(year);
    day_from_anchor(dd.day() as usize + dd.of(month_from_usize(month)) + day)
}

fn month_from_usize(date: usize) -> Month {
    match date {
        1  => January,
        2  => February,
        3  => March,
        4  => April,
        5  => May,
        6  => June,
        7  => July,
        8  => August,
        9  => September,
        10 => October,
        11 => November,
        12 => December,
        _ => unreachable!(),
    }
}

fn day_from_anchor(anchor: usize) -> Day {
    match anchor % 7 {
        1 => Monday,
        2 => Tuesday,
        3 => Wednsday,
        4 => Thursday,
        5 => Friday,
        6 => Saturday,
        0 => Sunday,
        _ => unreachable!(),
    }
}

/// Return true for given years which are defined to be leap years, to help
/// account for the fact there are not in fact a perfect 365 days in each year.
/// Where, a _day_ is defined to be a complete rotation of the earth around
/// it's axis (not to be confused with a _solar day_), and a _year_ is defined
/// to be the time it takes the earth to complete a trip in it's orbit around
/// the sun.
fn is_leap(year: usize) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// Number of leap years since (not including) 1752, the first leap year.
fn leaps(year: usize) -> usize {
    assert!(year >= FIRST_LEAP_YEAR);

    let offset = year - FIRST_LEAP_YEAR;
    (offset / 4) - (offset / 100) + (offset / 400)
}


#[cfg(test)]
mod tests {
    use super::*;

    // Day of the week (the trick).

    #[test]
    fn test_day_of_the_week() {
        assert_eq!(Friday, day_of_the_week(1937, 03, 26));
        assert_eq!(Saturday, day_of_the_week(2020, 04, 11));
    }

    #[test]
    fn test_day_from_anchor() {
        assert_eq!(Sunday, day_from_anchor(0));
        assert_eq!(Monday, day_from_anchor(1));
        assert_eq!(Friday, day_from_anchor(5));
        assert_eq!(Saturday, day_from_anchor(6));
    }


    // Leap year functions.

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

    // Doomsday functions.

    #[test]
    fn test_day() {
        assert_eq!(Day::Tuesday,  Doomsday(1752).day());
        assert_eq!(Day::Sunday,   Doomsday(1993).day());
        assert_eq!(Day::Saturday, Doomsday(2020).day());
    }

    #[test]
    fn test_date() {
        assert_eq!(3,  Doomsday(1993).date(Month::January));
        assert_eq!(4,  Doomsday(2020).date(Month::January));
        assert_eq!(28, Doomsday(1993).date(Month::February));
        assert_eq!(29, Doomsday(2020).date(Month::February));
        assert_eq!(0,  Doomsday(2000).date(Month::March));
        assert_eq!(4,  Doomsday(2020).date(Month::April));
        assert_eq!(9,  Doomsday(2020).date(Month::May));
        assert_eq!(6,  Doomsday(2020).date(Month::June));
        assert_eq!(11, Doomsday(2020).date(Month::July));
        assert_eq!(8,  Doomsday(2020).date(Month::August));
        assert_eq!(5,  Doomsday(2020).date(Month::September));
        assert_eq!(10, Doomsday(2020).date(Month::October));
        assert_eq!(7,  Doomsday(2020).date(Month::November));
        assert_eq!(12, Doomsday(2020).date(Month::December));
    }

}
