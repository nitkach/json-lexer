use chrono::prelude::*;

fn main() {
    // let utc = Utc::now().timestamp();
    // let local = Local::now().timestamp();

    let from_timestamp = chrono::NaiveDateTime::from_timestamp_opt(1700176335, 0).unwrap();

    // from_timestamp.for
    println!("From timestamp: {from_timestamp}.");
}
