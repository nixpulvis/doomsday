use doomsday::*;

fn main() {
    let dd = Doomsday(2020);
    println!("John H. Conway");
    println!("Died on the second {} of April", dd.on());
    println!("Nearest Doomsday: April {}", dd.of(Month::April));
    println!("R.I.P.");
}
