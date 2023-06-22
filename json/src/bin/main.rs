fn main() {
    let path = std::env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
}
