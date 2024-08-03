//VERY unfinished. Here be no dragons.

use std::io;

fn main() {
    top_menu();
}
fn top_menu() {
    let option1: (u32, &str) = (1, "New Game");
    let option2: (u32, &str) = (2, "Exit");
    let menu_list = [option1, option2];
    let mut menu_loop = true;

    while menu_loop {
        std::process::Command::new("clear").status().unwrap();
        println!("Main Menu:\n----------\n");
        for item in menu_list {
            println!("{}) {}", item.0, item.1);
        }
        let mut selection = String::new();
        io::stdin().read_line(&mut selection).expect("Failed to read line.");
        let selection: u32 = match selection.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid selection. Try again.");
                return;
            }
        };
        menu_loop = main_menu_selection(selection);
    }
}
fn main_menu_selection (selection: u32) -> bool {
    if selection == 1 {
        println!("Starting new game.");
        character_select();
        return false;
    }
    else if selection == 2 {
        println!("Exiting.");
        return false;
    }
    else {
        println!("Invalid Selection.");
        return true;
    }
}

fn character_select() {
    std::process::Command::new("clear").status().unwrap();
    let option1: (u32, &str) = (1, "Knight");
    let option2: (u32, &str) = (2, "Rogue");
    let option3: (u32, &str) = (3, "Ranger");
    let option4: (u32, &str) = 
    println!("Select your character.\n ");
}