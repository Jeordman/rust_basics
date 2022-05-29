pub fn run(){
    println!("--------------");
    // vector arrays have any len
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    
    // re-assign val
    numbers[2] = 20;

    // vector actions
    numbers.push(4);
    numbers.push(8);

    numbers.pop();


    println!("{:?}", numbers);

    println!("Single val: {}", numbers[0]);

    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice {:?}", slice);

    // loop through
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // modify arr
    for x in numbers.iter_mut() {
        // multiply each val by 2
        *x *= 2;
    }

    println!("--------------");
}
