pub fn run(){
    println!("--------------");
    // datatype and length must be specified in regular arrays
    let mut numbers: [i32; 5] = [1,2,3,4,5];
    
    // re-assign val
    numbers[2] = 20;

    println!("{:?}", numbers);

    println!("Single val: {}", numbers[0]);

    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice {:?}", slice);

    println!("--------------");
}
