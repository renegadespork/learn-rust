use std::io;

// TBH I still can't see a usecase for enum. Seems like match statements with extra steps.
//enum Coin {
//    Penny,
//    Nickel,
//    Dime,
//    Quarter,
//    None,
//}

fn value(coin: &str) -> u32 {
    match coin {
        "Penny" => 1,
        "Nickel" => 5,
        "Dime" => 10,
        "Quarter" => 25,
        &_ => 0,
    }
}

fn select(selection: u32) -> String {
    match selection {
        1 => String::from("Penny"),
        2 => String::from("Nickel"),
        3 => String::from("Dime"),
        4 => String::from("Quarter"),
        _ => String::from("None"),
    }
}

fn main() {
    let options: (&str, &str, &str, &str) = ("Penny", "Nickel", "Dime", "Quarter");
    let mut total: u32 = 0;
    println!("Your total is {} cents.\nPlease insert a coin:\n1. {}\n2. {}\n3. {}\n4. {}\n", &mut total, options.0, options.1, options.2, options.3);

    loop {
        let mut insert = String::new();

        io::stdin()
            .read_line(&mut insert)
            .expect("Failed to read line.");

        let insert: u32 = match insert.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid entry.");
                continue;
            }
        };
        println!("You inserted a {}\n", select(insert));
        total = total + value(&select(insert));
        let total_hr: f32 = (total as f32) / 100.00;
        println!("Your total is ${}.\nPlease insert a coin:\n1. {}\n2. {}\n3. {}\n4. {}\n", total_hr, options.0, options.1, options.2, options.3);
    }
}
