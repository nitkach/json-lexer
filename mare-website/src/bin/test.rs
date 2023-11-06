fn main() {
    let string = "mare".to_owned();

    let (str1, str2) = string.split_at(4);

    println!("{str1} + {str2} = {string}");
}
