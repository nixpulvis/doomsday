use std::fmt;
use crate::*;

/// 🝱 - The twelve months of the year
///
/// These months are roughly derived from the cycles of the moon. Additionally, months line up with
/// various seasons.
///
/// I'll add here that, a system of 13 months could work very nicely. Every
/// month would have 28 days, and there would only be a single day left over,
/// for new years.
//
// TODO: Update with information about the equinoxes.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Month {
    /// ♑ 270° Αἰγόκερως, Capricorn, or Mountain Goat
    ///
    /// Named after Janus the beginnings and endings.
    January,
    /// ♒ 300° Ὑδροχόος, Aquarius, or Water-Bearer
    ///
    /// Named for Februus which was a time of celebrations in Rome.
    February,
    /// ♓ 330° Ἰχθύες, Pisces, or Fish
    ///
    /// Named after the Mars, the Roman god of war. Despite "Aries" being the next sign, this is in
    /// part due to the fact that these signs do not correspond directly to the months.
    March,
    /// ♈ 0° Κριός, Aries, or Ram
    ///
    /// Aprilis the opening of the flowers.
    April,
    /// ♉ 30° Ταῦρος, Taurus, or Bull
    ///
    /// Maiores, meaning "major" or older.
    May,
    /// ♊ 60° Δίδυμοι, Gemini, or Twins
    ///
    /// Iuniores, meaning "junior" or the younger.
    June,
    /// ♋ 90° Καρκίνος, Cancer, or Crab
    ///
    /// Julius, named in 44 BCE, the year of his assassination.
    July,
    /// ♌ 120° Λέων, Leo, or Lion
    ///
    /// Augustus, a Roman emperor.
    August,
    /// ♍ 150° Παρθένος, Virgo, Maiden
    ///
    /// Septem, meaning seven.
    September,
    /// ♎ 180° Ζυγός, Libra, or Scales
    ///
    /// Octo, meaning eight.
    October,
    /// ♏ 210° Σκoρπίος, Scorpio, or Scorpion
    ///
    /// Novem, meaning nine.
    November,
    /// ♐ 240° Τοξότης, Sagittarius, or Archer
    ///
    /// Decem, meaning ten.
    December,

    // It would seem that the fall of Rome occurred before they could finish
    // the naming of the months to fruition.
}

impl From<usize> for Month {
    fn from(date: usize) -> Month {
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
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
