use doomsday::*;

fn main() {
    let dd = Doomsday(1937);
    println!("{:?} is {}, anchor in February on {}",
             dd, dd.day(), dd.anchor(February));
}

