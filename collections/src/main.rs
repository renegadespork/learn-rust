use std::collections::HashMap;

#[derive(Debug)]
enum Location {
    Id(u32),
    Name(String),
    X(f64),
    Y(f64),
}
fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    println!("My values are {:?}.", v);
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    print_n_check_express(&v);
    let mut v: Vec<i32> = vec! [9, 10];
    print_n_check_express(&v);
    v.push(11);
    print_n_check_express(&v);
    v.push(13);
    print_n_check_express(&v);
    let mut v2: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    v2.append(&mut v);
    print_n_check_express(&v2);
    println! ("{:?}", mirror(&v2));
    print_values(&v2);
    println! ("{:?}\n", double(&v2));
    let v3: Vec<Location> = vec![
        Location::Id(3),
        Location::Name(String::from("My favorite pizza place.")),
        Location::X(38.655633),
        Location::Y(-121.503206),
    ];
    location_details(v3);
    println!("\nTime to play with Strings.\n=========================");
    let array: [&str; 4] = ["Pre", "con", "cep", "tion"];
    println!("When you combine the following:\n{:?}\nYou get:\n{}", array, combinator(array));
    let word: &str = "trust";
    let ord: &str = &word[1..=4];
    println!("You can't spell \"{}\" without \"{}\".", word, ord);
    let people: &str = "🧍🧍🧍🧍🧍🧍";
    let space: char = '🌌';
    println!("\nThese people are too close together: {}\nLet's put some space between them:", people);
    personal_space(people, space);
    re_hash();
    let mut prices = HashMap::new();
    prices.insert("Apple", 0.30);
    prices.insert("Banana", 0.10);
    prices.insert("Watermelon", 3.00);
    prices.insert("Kiwi", 0.50);
    print_prices(&prices);
    println!("\n🍉🍉Watermelon Sale!!!🍉🍉\n50% off!");
    let prices = sale(&mut prices, "Watermelon", 0.5);
    print_prices(&prices);
    println!("\nTime for some exercises.\n");
    print_exercise(1, "Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.");
    exercise1();
    print_exercise(2, "Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.");
    exercise2();
}

fn print_n_check_express (v: &Vec<i32>) {
    print_values(&v);
    check_values(&v);
}

fn print_values(v: &Vec<i32>) {
    println!("My values are now {:?}", v);
}

fn check_values (v: &Vec<i32>) {
    let third: Option<&i32> = v.get(2);
    let fourth: Option<&i32> = v.get(3);
    match third {
        Some(third) => match fourth {
            Some(fourth) => println!("What comes between {} and {}?", third, fourth),
            None => println!("There is no fourth value."),
        }
        None => println!("There is no third value."),
    }
}

fn mirror(v: &Vec<i32>) -> Vec<i32> {
    println!("Mirroring vector...");
    let mut vtemp: Vec<i32> = Vec::new();
    for item in v {
        let x = item * -1;
        vtemp.push(x);
    }
    vtemp.push(0);
    for item in v {
        let x = item;
        vtemp.push(*x);
    }
    vtemp.sort();
    vtemp
}

fn double(v: &Vec<i32>) -> Vec<i32> {
    println!("Doubling all contents...");
    let mut vtemp: Vec<i32> = Vec::new();
    for item in v {
        let x = item * 2;
        vtemp.push(x);
    }
    vtemp
}

fn location_details(location: Vec<Location>) {
    let first = location.get(0);
    let second = location.get(1);
    let third = location.get(2);
    let fourth = location.get(3);
    match first {
        Some(first) => match second {
            Some(second) => match third {
                Some(third) => match fourth {
                    Some(fourth) => println!("Location Details:\nID: {:?}\nName: {:?}\nCoordinates: {:?},{:?}", first, second, third, fourth),
                    None => println!("Error. Missing location data."),
                },
                None => println!("Error. Missing location data."),
            },
            None => println!("Error. Missing location data."),
        },
        None => println!("Error. Missing location data."),
    }
}

fn combinator(input: [&str; 4]) -> String {
    let a = input[0];
    let b = input[1];
    let c = input[2];
    let d = input[3];
    let combined: String = format!("{a}{b}{c}{d}");
    combined
}

fn personal_space(emoji1: &str, emoji2: char) {
    for e in emoji1.chars() {
        println!("{}\n{}", e, emoji2);
    }
}

fn re_hash () {
    let mut scores = HashMap::new();
    scores.insert(String::from("Red Team"), 15);
    scores.insert(String::from("Blue Team"), 18);
    let team_name1 = String::from("Red Team");
    let team_name2 = String::from("Blue Team");
    let red_score = scores.get(&team_name1).copied().unwrap_or(0);
    let blue_score = scores.get(&team_name2).copied().unwrap_or(0);
    println!("\nScoreboard\n==========\nRed Team: {}\nBlue Team: {}", red_score, blue_score);
    println!("\nScoreboard... again...\n======================");
    for (team, score) in & scores {
        println!("{}: {}", team, score);
    }
}

fn print_prices(input: &HashMap<&str, f32>) {
    println!("\nPrices\n======");
    for key in input {
        println!("{:?} - ${:?}", key.0, key.1);
    }
}

fn sale<'a>(list: &'a mut HashMap<&'a str, f32>, name: &'a str, discount: f32) -> &'a mut HashMap<&'a str, f32> {
    let price = list.entry(name).or_insert(0.0);
    let sale_price = *price * discount;
    list.insert(name, sale_price);
    list
}

fn print_exercise(num: u8, desc: &str) {
    println!("\nExercise number {}:\n==================\n{}\n", num, desc);
}

fn exercise1() {
    let mut integers = vec![6, 2, 34, 25, 17, 12, 33, 34, 99, 4, 5, 5, 6, 6, 77, 104];
    println!("The values are {:?}", integers);
    fn mean (integers: &Vec<i32>) -> f32 {
        let mut sum = 0;
        let mut count = 0; 
        for value in integers {
            sum += value;
            count += 1;
        }
        let mean: f32 = sum as f32 / count as f32;
        mean
    }
    println!("The mean is {}", mean(&integers));
    integers.sort();
    let size = integers.capacity();
    let middle = size / 2;
    let median = integers.get(middle);
    match median {
        Some(median) => println!("The median is {}.", median),
        None => println!("Error. No median found.")
    }
    let mut freq = HashMap::new();
    for value in integers {
        let count = freq.entry(value).or_insert(0);
        *count += 1;
    }
    let max = freq.iter().max_by_key(|entry | entry.1);
    match max {
        Some(max) => println!("The mode is {:?}.", max.0),
        None => println!("Error. No mode found.")
    }
    let max = freq.iter().max_by_key(|entry | entry.0);
    match max {
        Some(max) => println!("The largest number is {:?}.", max.0),
        None => println!("Error. No maximum found.")
    }
}

fn exercise2() {
    use std::io;
    let users: Vec<String> = vec![];
    let user_depts: HashMap<String, String> = HashMap::new();
    enum Commands {
        Add,
        List,
        Unknown,
    }
    println!("Enter a command. Your options are \"Add [User] [Group]\" or \"List\"");
    let command = parse_user_input();
    match command {
        Commands::Add => println!("The user sent an add command."),
        Commands::List => println!("The user sent a list command."),
        Commands::Unknown => println!("The user sent an unknown command."),
    }
    fn parse_user_input() -> Commands {
        let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line.");
        let input: Vec<&str> = input.trim()
            .split_whitespace()
            .collect();
        let command: String = input[0].to_lowercase();
        match command.as_str() {
            "add" => Commands::Add,
            "list" => Commands::List,
            _ => Commands::Unknown,
        }
    }
}