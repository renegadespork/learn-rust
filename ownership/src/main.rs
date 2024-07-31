// use std::io;

fn main() {
    let a = String::from ("Hello");
    let x = 5;
    let b = String::from ("World");
    let c = String::from ("I've got a lovely bunch of coconuts.");
    let d = String::from ("General Kenobi");
    let e = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    takes_ownership(a);
//    println!("{a}"); //This will not work since the value of a has moved out of scope.
    makes_copy(x); 
    println!("{x}"); // This works since x is copied.
    println!("{}", gives_and_takes(b));
    let len = calculate_length(&c);
    print!("{}\n", len);
    println!("{} {}", deconstructs(&d)[0], deconstructs(&d)[1]);
    println!("My workdays are {}, {}, {}, {}, and {}.", array_slice(&e)[0], array_slice(&e)[1], array_slice(&e)[2], array_slice(&e)[3], array_slice(&e)[4]);
}

fn takes_ownership(string_a: String) {
    println!("{string_a}");
}

fn makes_copy(some_interger: i32) {
    println!("{some_interger}");
}

fn gives_and_takes(_string_b: String) -> String {
    let string_b = String::from ("There");
    string_b
}

// References (borrows) without taking ownership.
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn deconstructs(s: &String) -> [&str; 2] {
    let first = &s[0..7];
    let second = &s[8..=13];
    let rebuild = [first, second];
    rebuild
}

fn array_slice<'a>(e: &'a[&'a str]) -> &'a[&'a str] {
    let work_week = &e[1..=5];
    work_week
}