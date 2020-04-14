use std::fmt;
use crate::*;

/// 🜨 - The seven days in a week
///
/// Daylight (⫰) and night (⫯, and also 🝯), together forming a complete day (🝰). Each day is is
/// made up of **exactly** (ignoring leap seconds) 24 hours (🝮).
///
/// The earth has a few shores with the distinct coloring of verdigris (🜨).
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Day {
    /// ○ Moon or ⬜ Silver
    ///
    /// 🌑🌒🌓🌔🌕🌖🌗🌘🌑
    Monday,
    /// ♂ Mars or ⬜ Iron
    Tuesday,
    /// ☿ ⬜ Mercury
    Wednesday,
    /// ♃ Jupiter or ⬜ Tin
    Thursday,
    /// ♀ Venus or 🟫 Copper
    Friday,
    /// ♄ Saturn or 🟦 Lead
    Saturday,
    /// ☉ Sol or 🟧 Gold, our sun
    Sunday,
}

impl Day {
    /// Convert an anchor date to it's corresponding day of the week.
    ///
    /// An anchor date is defined with respect to a [`Doomsday`]. For example the 21st is an anchor
    /// date, with meaning when used in a specific year.
    //
    // TODO: This is inconsistent with (Day as usize), document or correct.
    pub fn from_anchor(anchor: usize) -> Day {
        match anchor % 7 {
            1 => Monday,
            2 => Tuesday,
            3 => Wednesday,
            4 => Thursday,
            5 => Friday,
            6 => Saturday,
            0 => Sunday,
            _ => unreachable!(),
        }
    }
}

impl Default for Day {
    fn default() -> Day { Sunday }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_from_anchor() {
        assert_eq!(Sunday, Day::from_anchor(0));
        assert_eq!(Monday, Day::from_anchor(1));
        assert_eq!(Friday, Day::from_anchor(5));
        assert_eq!(Saturday, Day::from_anchor(6));
    }

}
