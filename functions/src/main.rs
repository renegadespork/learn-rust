fn main() {
    intro_function(3, "Jesse", '🐢');
    println!("{}", mean(1,6,14));
}

// Prints arguments
fn intro_function(x: i32, name: &str, emoji: char) {
    println!("Hello {name} {emoji}! The value of x is {x}.");
}

// Averages arguments
fn mean(x: i64, y: i64, z: i64) -> i64 {
    ( x + y +  z ) / 3
}