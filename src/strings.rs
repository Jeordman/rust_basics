pub fn run(){
    // growable string
    let mut hello = String::from("Hello ");

    // get len of str
    println!("length of str: {}", hello.len());

    hello.push('W');

    hello.push_str("orld");

    // capacity in bytes
    println!("Capacity: {}", hello.capacity());

    println!("Is Empty: {}", hello.is_empty());

    println!("Contains 'world' {}", hello.contains("World"));

    println!("Replace: {}", hello.replace("World", "There"));

    // loot through str on whitespace
    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);

    println!("{}", hello);
    println!("--------------");
}
