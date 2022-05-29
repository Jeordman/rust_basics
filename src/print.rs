pub fn run() {
    // print to console
    println!("--------------");

    println!("Hello from the print.rs file");

    // Basic formatting
    println!("{} is from {}", "Jeordin", "Utah");

    // positional 
    println!("{0} is from {1} and likes to {2}", "Jeordin", "Utah", "Code");

    // named
    println!("{name} is from {state}", name="Jeordin", state="utah");

    // debug trait 
    println!("{:?}", (12, true, "hello"));

    println!("--------------");
}
