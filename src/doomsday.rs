use std::fmt;
use crate::*;

///  Behold the 💥 doomsday of a year
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

impl Doomsday {
    /// Create a new doomsday for a given year.
    /// TODO: Random or something?
    #[inline]
    pub fn new(year: usize) -> Doomsday {
        Doomsday(year)
    }

    /// Return the **day number**, of the anchor doomsday in a given month.
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
    //
    // TODO: Formatting function. Would almost be trivial, if it wasn't for the 3rd.
    pub fn anchor(&self, month: Month) -> usize {
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
        Day::from_anchor(self.0 + leaps(self.0))
    }
}

/// The display of doomsday is day, one for every week if you wish.
impl fmt::Display for Doomsday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.day())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        assert_eq!(Day::Tuesday,  Doomsday(1752).day());
        assert_eq!(Day::Sunday,   Doomsday(1993).day());
        assert_eq!(Day::Saturday, Doomsday(2020).day());
    }

    #[test]
    fn test_anchor() {
        assert_eq!(3,  Doomsday(1993).anchor(Month::January));
        assert_eq!(4,  Doomsday(2020).anchor(Month::January));
        assert_eq!(28, Doomsday(1993).anchor(Month::February));
        assert_eq!(29, Doomsday(2020).anchor(Month::February));
        assert_eq!(0,  Doomsday(2000).anchor(Month::March));
        assert_eq!(4,  Doomsday(2020).anchor(Month::April));
        assert_eq!(9,  Doomsday(2020).anchor(Month::May));
        assert_eq!(6,  Doomsday(2020).anchor(Month::June));
        assert_eq!(11, Doomsday(2020).anchor(Month::July));
        assert_eq!(8,  Doomsday(2020).anchor(Month::August));
        assert_eq!(5,  Doomsday(2020).anchor(Month::September));
        assert_eq!(10, Doomsday(2020).anchor(Month::October));
        assert_eq!(7,  Doomsday(2020).anchor(Month::November));
        assert_eq!(12, Doomsday(2020).anchor(Month::December));
    }
}
