use chrono::prelude::*;

fn main() {
    let utc = Utc::now().timestamp();       // e.g. `2014-11-28T12:45:59.324310806Z`
    let local = Local::now().timestamp();

    println!("DateTime: {utc}\nLocal: {local}", );
}
