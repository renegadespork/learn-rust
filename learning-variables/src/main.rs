fn main() {
    const PROGRAM_NAME: &str = "Learning Variables";
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}",PROGRAM_NAME);
    println!("==================");
    let x: u32 = 5;
    println!("The value of x is {x}");
    println!("Incrementing by 1.");
    let x = x + 1;
    println!("Now the value of x is {x}");
    let y: i32 = -4;
    println!("The value of y is {y}.");
    let x: i32 = x.try_into().unwrap();
    let z: i32 = x + y;
    println!("x + y = {z}");
    {
        println!("Temporarily doubling x.");
        let x = x * 2;
        let z: i32 = x + y;
        println!("x + y is now {z}");
    }
    println!("The value of x has returned to {x}.\n");
    println!("Three hours is {} seconds btw.\n", THREE_HOURS_IN_SECONDS);
    
    // let 
}