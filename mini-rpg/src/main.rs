//VERY unfinished. Here be no dragons.

use std::io;
use std::process;

enum MainMenuSelection {
    NewGame,
    LoadSave,
    Exit,
    Invalid,
}

#[derive(Debug)]
enum Class {
    Knight,
    Rogue,
    Ranger,
    Wizard,
}

fn class_name(class: Class) -> String {
    match class {
        Class::Knight => String::from("Knight"),
        Class::Rogue => String::from("Rogue"),
        Class::Ranger => String::from("Ranger"),
        Class::Wizard => String::from("Wizard"),
    }
}

fn class_id(class: Class) -> u8 {
    match class {
        Class::Knight => 1,
        Class::Rogue => 2,
        Class::Ranger => 3,
        Class::Wizard => 4,
    }
}

struct Info {
    id: u8,
    strength: u8,
    fortitude: u8,
    agility: u8,
    intelligence: u8,
    charisma: u8,
    luck: u8,
}

struct Stats {
    max_health: u32,
    max_stamina: u32,
    max_mana: u32,
}

struct Save {
    class: Class,
    char_name: String,
    page: u64,
    alignment: i32,
}

fn main() {
    let top_menu_selection: MainMenuSelection = top_menu();
    match top_menu_selection {
        MainMenuSelection::NewGame => {
            let character_selection = character_select();
            introduction(character_selection);
        },
        MainMenuSelection::LoadSave => process::exit(0),
        MainMenuSelection::Exit => process::exit(0),
        MainMenuSelection::Invalid => process::exit(1),
    }
}

fn top_menu() -> MainMenuSelection {
    let option1: (u8, &str) = (1, "New Game");
    let option2: (u8, &str) = (2, "Exit");
    let menu_list = [option1, option2];
    loop {
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
                println!("You must enter a number.");
                return MainMenuSelection::Invalid;
            }
        };
        if selection == 1 {
            println!("Starting new game.");
            return MainMenuSelection::NewGame;
        }
        else if selection == 2 {
            println!("Exiting.");
            return MainMenuSelection::Exit;
        }
        else {
            println!("Invalid Selection.");
            return MainMenuSelection::Invalid;
        }
    }
}

fn character_select() -> u8 {
    let option1: (u32, &str) = (1, "Knight");
    let option2: (u32, &str) = (2, "Rogue");
    let option3: (u32, &str) = (3, "Ranger");
    let option4: (u32, &str) = (4, "Wizard");
    let menu_list = [option1, option2, option3, option4];

    loop {
        std::process::Command::new("clear").status().unwrap();
        println!("Select your character.");
        for item in menu_list {
            println!("{}) {}", item.0, item.1);
        }
        let mut selection = String::new();
        io::stdin().read_line(&mut selection).expect("Failed to read line.");
        let selection: u32 = match selection.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You must enter a number.");
                continue;
            }
        };
        let character_details_output: (bool, u8) = character_details(selection);
        let menu_loop = character_details_output.0;
        if menu_loop {
            continue;
        }
        else {
            if character_details_output.1 == 1 {
                return class_id(Class::Knight);
            }
            else if character_details_output.1 == 2 {
                return class_id(Class::Rogue);
            }
            else if character_details_output.1 == 3 {
                return class_id(Class::Ranger);
            }
            else if character_details_output.1 == 4 {
                return class_id(Class::Wizard);                
            }
            else {
                continue;
            }
        }
    };

}

fn character_details(selection: u32) -> (bool, u8) {
    let trait_list = ["Strength", "Fortitude", "Agility", "Intelligence", "Charisma", "Luck"];

    let knight_info = Info {
        id: 1,
        strength: 8,
        fortitude: 8,
        agility: 2,
        intelligence: 3,
        charisma: 4,
        luck: 5,
    };
    
    let rogue_info = Info {
        id: 2,
        strength: 3,
        fortitude: 4,
        agility: 7,
        intelligence: 2,
        charisma: 7,
        luck: 7,
    };
    
    let ranger_info = Info {
        id: 3,
        strength: 5,
        fortitude: 4,
        agility: 8,
        intelligence: 4,
        charisma: 4,
        luck: 5,
    };
    
    let wizard_info = Info {
        id: 4,
        strength: 2,
        fortitude: 3,
        agility: 5,
        intelligence: 8,
        charisma: 6,
        luck: 6,
    };
    match selection {
        1 => {
            std::process::Command::new("clear").status().unwrap();
            println!("{}:", class_name(Class::Knight));
            println!("  {} - {}", trait_list[0], knight_info.strength);
            println!("  {} - {}", trait_list[1], knight_info.fortitude);
            println!("  {} - {}", trait_list[2], knight_info.agility);
            println!("  {} - {}", trait_list[3], knight_info.intelligence);
            println!("  {} - {}", trait_list[4], knight_info.charisma);
            println!("  {} - {}", trait_list[5], knight_info.luck);
            println!("---");
            println!("Choose this class?\n1) Yes\n2) No");
            let mut selection = String::new();
            io::stdin().read_line(&mut selection).expect("Failed to read line.");
            let selection: u32 = match selection.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("You must enter 1 or 2.");
                    return (true, 0);
                }
            };
            match selection {
                1 => return (false, knight_info.id),
                2 => return (true, 0),
                _ => {
                    println!("You must enter 1 or 2.");
                    return (true, 0);
                },
            };
        },
        2 => {
            std::process::Command::new("clear").status().unwrap();
            println!("{}:", class_name(Class::Rogue));
            println!("  {} - {}", trait_list[0], rogue_info.strength);
            println!("  {} - {}", trait_list[1], rogue_info.fortitude);
            println!("  {} - {}", trait_list[2], rogue_info.agility);
            println!("  {} - {}", trait_list[3], rogue_info.intelligence);
            println!("  {} - {}", trait_list[4], rogue_info.charisma);
            println!("  {} - {}", trait_list[5], rogue_info.luck);
            println!("---");
            println!("Choose this class?\n1) Yes\n2) No");
            let mut selection = String::new();
            io::stdin().read_line(&mut selection).expect("Failed to read line.");
            let selection: u32 = match selection.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("You must enter 1 or 2.");
                    return (true, 0);
                }
            };
            match selection {
                1 => return (false, rogue_info.id),
                2 => return (true, 0),
                _ => {
                    println!("You must enter 1 or 2.");
                    return (true, 0);
                },
            };
        },
        3 => {
            std::process::Command::new("clear").status().unwrap();
            println!("{}:", class_name(Class::Ranger));
            println!("  {} - {}", trait_list[0], ranger_info.strength);
            println!("  {} - {}", trait_list[1], ranger_info.fortitude);
            println!("  {} - {}", trait_list[2], ranger_info.agility);
            println!("  {} - {}", trait_list[3], ranger_info.intelligence);
            println!("  {} - {}", trait_list[4], ranger_info.charisma);
            println!("  {} - {}", trait_list[5], ranger_info.luck);
            println!("---");
            println!("Choose this class?\n1) Yes\n2) No");
            let mut selection = String::new();
            io::stdin().read_line(&mut selection).expect("Failed to read line.");
            let selection: u32 = match selection.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("You must enter 1 or 2.");
                    return (true, 0);
                }
            };
            match selection {
                1 => return (false, ranger_info.id),
                2 => return (true, 0),
                _ => {
                    println!("You must enter 1 or 2.");
                    return (true, 0);
                },
            };
        },
        4 => {
            std::process::Command::new("clear").status().unwrap();
            println!("{}:", class_name(Class::Wizard));
            println!("  {} - {}", trait_list[0], wizard_info.strength);
            println!("  {} - {}", trait_list[1], wizard_info.fortitude);
            println!("  {} - {}", trait_list[2], wizard_info.agility);
            println!("  {} - {}", trait_list[3], wizard_info.intelligence);
            println!("  {} - {}", trait_list[4], wizard_info.charisma);
            println!("  {} - {}", trait_list[5], wizard_info.luck);
            println!("---");
            println!("Choose this class?\n1) Yes\n2) No");
            let mut selection = String::new();
            io::stdin().read_line(&mut selection).expect("Failed to read line.");
            let selection: u32 = match selection.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("You must enter 1 or 2.");
                    return (true, 0);
                }
            };
            match selection {
                1 => return (false, wizard_info.id),
                2 => return (true, 0),
                _ => {
                    println!("You must enter 1 or 2.");
                    return (true, 0);
                },
            };
        },
        _ => {
            println!("Invalid Selection.");
            (true, 0)
        },
    }
}

fn introduction(character_select: u8) {
    if character_select == 1 {
        let player_save = Save {
            class: Class::Knight,
            char_name: String::new(),
            page: 1,
            alignment: 0,
        };
        println!("Welcome, {:?}!", class_name(Class::Knight));
    }
    else if character_select == 2 {
        let player_save = Save {
            class: Class::Rogue,
            char_name: String::new(),
            page: 1,
            alignment: 0,
        };
        println!("Welcome, {:?}!", class_name(Class::Rogue));
    }
    else if character_select == 3 {
        let player_save = Save {
            class: Class::Ranger,
            char_name: String::new(),
            page: 1,
            alignment: 0,
        };
        println!("Welcome, {:?}!", class_name(Class::Ranger));
    }
    else if character_select == 4 {
        let player_save = Save {
            class: Class::Wizard,
            char_name: String::new(),
            page: 1,
            alignment: 0,
        };
        println!("Welcome, {:?}!", class_name(Class::Wizard));
    }
}