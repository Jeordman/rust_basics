pub fn run(){
    println!("--------------");

    let person: (&str, &str, i8) = ("Jeordin", "Utah", 24);
    println!("{} is from {} and is {}", person.0, person.1, person.2);

    println!("--------------");
}
