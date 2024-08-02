//This is broken AF and I cann't figure out why the loops won't work...

use std::io;

fn main() {
    top_menu();
}
fn top_menu() {
    let option1: (u32, &str) = (1, "New Game");
    let option2: (u32, &str) = (2, "Exit");
    let menu_list = [option1, option2];
    let mut selection = String::new();

    loop {
        let menu_loop = true;
        println!("Main Menu:\n----------\n");
        for item in menu_list {
            println!("{}) {}", item.0, item.1);
        }
        io::stdin().read_line(&mut selection).expect("Failed to read line.");
        let selection: u32 = match selection.trim().parse() {
            Ok(selection) => selection,
            Err(_) => {
                println!("Invalid selection. Try again.");
                return;
            }
        };
        let menu_loop = check_selection(selection);
        if menu_loop {
            continue;
        }
        else {
            break;
        }
    }
}
fn check_selection (selection: u32) -> bool {
    if selection == 1 {
        println!("Starting new game.");
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