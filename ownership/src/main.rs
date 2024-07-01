// use std::io;

fn main() {
    let a = String::from ("Hello");
    let x = 5;
    let b = String::from ("World");
    let c = String::from ("I've got a lovely bunch of coconuts.");
    takes_ownership(a);
//    println!("{a}"); //This will not work since the value of a has moved out of scope.
    makes_copy(x); // This works since this value is copied.
    println!("{}", gives_and_takes(b));
    let len = calculate_length(&c);
    print!("{}\n", len);
}

fn takes_ownership(string_a: String) {
    println!("{string_a}");
}

fn makes_copy(some_interger: i32) {
    println!("{some_interger}");
}

fn gives_and_takes(string_b: String) -> String {
    let string_b = String::from ("There");
    string_b
}

fn calculate_length(s: &String) -> usize {
    s.len()
}