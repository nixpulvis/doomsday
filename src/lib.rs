/// Return true for given years which are defined to be leap years, to help
/// account for the fact there are not in fact a perfect 365 days in each year.
/// Where, a _day_ is defined to be a complete rotation of the earth around
/// it's axis (not to be confused with a _solar day_), and a _year_ is defined
/// to be the time it takes the earth to complete a trip in it's orbit around
/// the sun.
pub fn is_leap(year: usize) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// Number of leap years since (not including) 1752, the first leap year.
///
/// We don't include 1752, since it was the initial jump of 11 years; thus
/// marking the transition to the Gregorian calendar for the British Empire.
/// Therefor, dates before this year aren't supported by this doomsday
/// algorithm.
pub fn leaps(year: usize) -> usize {
    const FIRST_LEAP_YEAR: usize = 1752;
    assert!(year >= FIRST_LEAP_YEAR);

    let offset = year - FIRST_LEAP_YEAR;
    (offset / 4) - (offset / 100) + (offset / 400)
}

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
    Saterday,
    /// Sol (the sun itself)
    Sunday,
}

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

/// Behold the doomsday of a year.
///
/// NOTE: Years before `FIRST_LEAP_YEAR` aren't supported.
pub struct Doomsday(pub usize);

impl Doomsday {
    /// Return the **day of the week** which doomsday lies on in a given
    /// year.
    pub fn day(&self) -> Day {
        use Day::*;

        match (self.0 + leaps(self.0)) % 7 {
            1 => Monday,
            2 => Tuesday,
            3 => Wednsday,
            4 => Thursday,
            5 => Friday,
            6 => Saterday,
            0 => Sunday,
            _ => unreachable!(),
        }
    }

    /// Return the **day number** which doomsday lies on in a given month.
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
    pub fn day_number(&self, month: Month) -> usize {
        use Month::*;

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
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_leap() {
        assert!(is_leap(1752));
        assert!(is_leap(1756));
        assert!(is_leap(2000));
        assert!(is_leap(2020));

        assert!(!is_leap(1800));
        assert!(!is_leap(1900));
    }

    #[test]
    fn test_leaps() {
        assert_eq!(0, leaps(1752));
        assert_eq!(1, leaps(1756));
        assert_eq!(65, leaps(2020));
    }

    #[test]
    fn test_day() {
        assert_eq!(Day::Saterday, Doomsday(2020).day());
    }

    #[test]
    fn test_month() {
        assert_eq!(3,  Doomsday(1993).day_number(Month::January));
        assert_eq!(4,  Doomsday(2020).day_number(Month::January));
        assert_eq!(28, Doomsday(1993).day_number(Month::February));
        assert_eq!(29, Doomsday(2020).day_number(Month::February));
        assert_eq!(0,  Doomsday(2000).day_number(Month::March));
        assert_eq!(4,  Doomsday(2020).day_number(Month::April));
        assert_eq!(9,  Doomsday(2020).day_number(Month::May));
        assert_eq!(6,  Doomsday(2020).day_number(Month::June));
        assert_eq!(11, Doomsday(2020).day_number(Month::July));
        assert_eq!(8,  Doomsday(2020).day_number(Month::August));
        assert_eq!(5,  Doomsday(2020).day_number(Month::September));
        assert_eq!(10, Doomsday(2020).day_number(Month::October));
        assert_eq!(7,  Doomsday(2020).day_number(Month::November));
        assert_eq!(12, Doomsday(2020).day_number(Month::December));
    }

}
