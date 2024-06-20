use std::io;

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
    
    {
        let pi = 3.14159; // f64
        let e: f32 = 2.71828; // f32
        let quotient = pi / e;
        let trunc_quotient = quotient as i64;
        let wink: char = '😉';
        println!("Also, pi / e = {}.... which is basically {}. {}\n", quotient, trunc_quotient, wink);
    }
    {
        println!("Now, for something a little different.\n----------------");
        let data1: (i32, f64, char) = (213, 78.433, '🥜');
        let (x, y, z) = data1;
        println!("Data: {}, {}, {}", data1.0, data1.1, data1.2);
        println!("Data: {x}, {y}, {z}\nThese rows should match.\n")
    }
    {
        let stooges = ["Moe", "Larry", "Curly"];
        let months: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
        let m2 = months[1];
        println!("The Three Stooges are {}, {}, and {}.", stooges[0], stooges[1], stooges[2]);
        println!("First two months of the year are {} and {}.\n", months[0], m2);
    }
    {
        let fruits: [char; 5] = ['🍎', '🍌', '🥝', '🍓', '🍍'];
        println!("Now for the **interactive** portion.\nPick your favorite fruit.\n1. {}\n2. {}\n3. {}\n4. {}\n5. {}", fruits[0], fruits[1], fruits[2], fruits[3], fruits[4]);
        println!("Enter the number of your choice:\n");
        let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line.");
        let choice: usize = choice.trim().parse().expect("Invalid choice.");
        println!("You chose {}. Good choice!", fruits[choice-1]);
    }
}