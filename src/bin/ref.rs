fn main() {

    println!("{:?}", 'ૌ' as u32);
    println!("{:?}", '❤' as u32);
    println!("{:?}", char::from_u32(0x2764));
    println!("{:?}", char::from_u32(2764));

    let foo = 0x2764;
    let bar = 2764;

    println!("foo = {:?}", char::from_u32(foo));
    println!("bar = {:?}", char::from_u32(bar));
}
