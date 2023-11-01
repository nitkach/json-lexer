// OptIn Sized
// OptOut Default: Sized ON. required to write: ?Sized
// OptOut Terraform def: ask. --auto-confirm to turn off save

// // OptOut for generic parameters: Sized
// fn generic<T: Display + ?Sized>(display: &T) {
//     println!("{}", display)
// }

fn main() {

    // let string = "pony";
    // generic(&40);
    // generic(string);
    // generic(&"Pony".to_owned())

    // println!("{:?}", 'ૌ' as u32);
    // println!("{:?}", '❤' as u32);
    // println!("{:?}", char::from_u32(0x2764));
    // println!("{:?}", char::from_u32(2764));

    // let foo = 0x2764;
    // let bar = 2764;

    // println!("foo = {:?}", char::from_u32(foo));
    // println!("bar = {:?}", char::from_u32(bar));
}
