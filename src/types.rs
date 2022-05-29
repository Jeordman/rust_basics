pub fn run() {
    println!("--------------");
    // default i32
    let x = 1;

    // default f64
    let y = 2.5;

    // add explicit type
    let z: i64 = 4444444444444444;

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // boolean
    let is_active = true;
    println!("{:?}", (x, y, z, is_active));

    // bool from expression
    let is_greater = 10 > 5;

    let a1 = 'a';
    let face = '\u{1f600}';

    println!("{:?}", (is_greater, a1, face));

    println!("--------------");
}
