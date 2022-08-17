use chrono::{Local, DateTime};

fn main() {
    let dt: DateTime<Local> = Local::now();
    let hour = dt.format("%H").to_string();
    let minute = dt.format("%M").to_string();
    let second = dt.format("%S").to_string();

    println!("It is: {hour} : {minute} : {second}");
}
