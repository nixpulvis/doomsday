use doomsday::*;

fn main() {
    println!("JHC was born on {}", day_of_the_week(1937, 03, 26));
    println!("JHC died on {}", day_of_the_week(2020, 04, 11));

    println!("Next week is {}", day_of_the_week(2020, 04, 19));
    println!("Today is {}", day_of_the_week(2020, 04, 14));
}
