pub fn run(){
    fn greet(greet: &str, name: &str) { }

    fn add(n1: i32, n2: i32) -> i32 {
        return n1 + n2;
    }

    // closure use vars outside function
    let n3 = 1;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    
}
