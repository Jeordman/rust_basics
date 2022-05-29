pub fn run() {
    let age = 10;
    let check_id: bool = false;
    let knows_person_of_age = true;

    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What do u want to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: You gotta leav");
    } else {
        println!("Bartender: I need to see your id");
    }

    // shorthand if
    let is_of_age = if age >= 21 {true} else {false};
}
