// use serde::Deserialize;
// use serde_json::Value;

fn main() {
    let string = std::fs::read_to_string("./benches/.json").unwrap();

    let now = std::time::Instant::now();
    json::parse(&string).unwrap();
    println!("My json parse: {:?}", now.elapsed());

    let now = std::time::Instant::now();
    serde_json::from_str::<serde_json::Value>(&string).unwrap();
    println!("Serde json parse: {:?}", now.elapsed());

    // let now = std::time::Instant::now();
    // serde_json::from_str::<Root>(&string).unwrap();
    // println!("Serde json strongly typed parse: {:?}", now.elapsed());
}
