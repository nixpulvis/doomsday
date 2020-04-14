use doomsday::*;

fn main() {
    let dd = Doomsday(1993);
    println!("{:?} is {}, anchor in February on {}",
             dd, dd.day(), dd.anchor(February));
    println!("{:?} is {}, anchor in March on {}",
             dd, dd.day(), dd.anchor(March));

    let leap_dd = Doomsday(2000);
    println!("{:?} is {}, anchor in February on {}",
             leap_dd, leap_dd.day(), leap_dd.anchor(February));
    println!("{:?} is {}, anchor in March on {}",
             leap_dd, leap_dd.day(), leap_dd.anchor(March));
}
