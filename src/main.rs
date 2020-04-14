use doomsday::*;

fn main() {
    let dd = Doomsday(2020);
    println!("--------------");
    println!("John H. Conway");
    println!("--------------");
    println!("Died on a doomsday in April of {}", dd.0);
    println!("Anchored on the {}th", dd.anchor(Month::April));
    println!("The second {} of the month", dd);
    println!("R.I.P.");
}
