//VERY unfinished. Here be no dragons.

use std::io;

enum Traits {
    Strength,
    Fortitude,
    Agility,
    Intelligence,
    Charisma,
    Luck,
}

struct TraitInfo {
    id: u8,
    name: String,
}

fn trait_info(traits: Traits) -> TraitInfo {
    match traits {
        Traits::Strength => TraitInfo {
            id: 1,
            name: String::from("Strength"),
        },
        Traits::Fortitude => TraitInfo {
            id: 2,
            name: String::from("Fortitude"),
        },
        Traits::Agility => TraitInfo {
            id: 3,
            name: String::from("Agility"),
        },
        Traits::Intelligence => TraitInfo {
            id: 4,
            name: String::from("Intelligence"),
        },
        Traits::Charisma => TraitInfo {
            id: 5,
            name: String::from("Charisma"),
        },
        Traits::Luck => TraitInfo {
            id: 6,
            name: String::from("Luck"),
        },
    }
}

enum Class {
    Knight,
    Rogue,
    Ranger,
    Wizard,
}

struct ClassStats {
    id: u8,
    name: String,
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

fn class_stats(class: Class) -> ClassStats {
    match class {
        Class::Knight => ClassStats {
            id: 1,
            name: String::from("Knight"),
            strength: 8,
            fortitude: 8,
            agility: 2,
            intelligence: 3,
            charisma: 4,
            luck: 5,
        },
        Class::Rogue => ClassStats {
            id: 2,
            name: String::from("Rogue"),
            strength: 3,
            fortitude: 4,
            agility: 7,
            intelligence: 2,
            charisma: 7,
            luck: 7,
        },
        Class::Ranger => ClassStats {
            id: 3,
            name: String::from("Ranger"),
            strength: 5,
            fortitude: 4,
            agility: 8,
            intelligence: 4,
            charisma: 4,
            luck: 5,
        },
        Class::Wizard => ClassStats {
            id: 4,
            name: String::from("Wizard"),
            strength: 2,
            fortitude: 3,
            agility: 5,
            intelligence: 8,
            charisma: 6,
            luck: 6,
        },
    }
}

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
                println!("You must enter a number.");
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
    let option1: (u32, &str) = (1, "Knight");
    let option2: (u32, &str) = (2, "Rogue");
    let option3: (u32, &str) = (3, "Ranger");
    let option4: (u32, &str) = (4, "Wizard");
    let menu_list = [option1, option2, option3, option4];
    // let mut menu_loop = true;

    let character_select = loop {
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
                return;
            }
        };
        let character_details_output: (bool, u8) = character_details(selection);
        let menu_loop = character_details_output.0;
        if menu_loop {
            continue;
        }
        else {
            break character_details_output.1;
        }
    };
    introduction(character_select);
}

fn character_details(selection: u32) -> (bool, u8) {
    let trait_list = [trait_info(Traits::Strength).name, trait_info(Traits::Fortitude).name, trait_info(Traits::Agility).name, trait_info(Traits::Intelligence).name, trait_info(Traits::Charisma).name, trait_info(Traits::Luck).name];


    match selection {
        1 => {
            let knight_str: (String, u8) = (trait_info(Traits::Strength).name, class_stats(Class::Knight).strength);
            let knight_for: (String, u8) = (trait_info(Traits::Fortitude).name, class_stats(Class::Knight).fortitude);
            let knight_agi: (String, u8) = (trait_info(Traits::Agility).name, class_stats(Class::Knight).agility);
            let knight_int: (String, u8) = (trait_info(Traits::Intelligence).name, class_stats(Class::Knight).intelligence);
            let knight_cha: (String, u8) = (trait_info(Traits::Charisma).name, class_stats(Class::Knight).charisma);
            let knight_luc: (String, u8) = (trait_info(Traits::Luck).name, class_stats(Class::Knight).luck);
            let knight_stats = [knight_str, knight_for, knight_agi, knight_int, knight_cha, knight_luc];

            std::process::Command::new("clear").status().unwrap();
            println!("{}:", class_stats(Class::Knight).name);
            for stat in knight_stats {
                println!(" {} - {}", stat.0, stat.1);
            }
            println!("---\nChoose this class?\n1) Yes\n2) No");
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
                1 => return (false, class_stats(Class::Knight).id),
                2 => return (true, 0),
                _ => {
                    println!("You must enter 1 or 2.");
                    return (true, 0);
                },
            };
        },
        2 => {
            let rogue_str: (String, u8) = (trait_info(Traits::Strength).name, class_stats(Class::Rogue).strength);
            let rogue_for: (String, u8) = (trait_info(Traits::Fortitude).name, class_stats(Class::Rogue).fortitude);
            let rogue_agi: (String, u8) = (trait_info(Traits::Agility).name, class_stats(Class::Rogue).agility);
            let rogue_int: (String, u8) = (trait_info(Traits::Intelligence).name, class_stats(Class::Rogue).intelligence);
            let rogue_cha: (String, u8) = (trait_info(Traits::Charisma).name, class_stats(Class::Rogue).charisma);
            let rogue_luc: (String, u8) = (trait_info(Traits::Luck).name, class_stats(Class::Rogue).luck);
            let rogue_stats = [rogue_str, rogue_for, rogue_agi, rogue_int, rogue_cha, rogue_luc];

            std::process::Command::new("clear").status().unwrap();
            println!("{}:", class_stats(Class::Rogue).name);
            for stat in rogue_stats {
                println!(" {} - {}", stat.0, stat.1);
            }
            println!("---\nChoose this class?\n1) Yes\n2) No");
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
                1 => return (false, class_stats(Class::Rogue).id),
                2 => return (true, 0),
                _ => {
                    println!("You must enter 1 or 2.");
                    return (true, 0);
                },
            };
        },
        3 => {
            let ranger_str: (String, u8) = (trait_info(Traits::Strength).name, class_stats(Class::Ranger).strength);
            let ranger_for: (String, u8) = (trait_info(Traits::Fortitude).name, class_stats(Class::Ranger).fortitude);
            let ranger_agi: (String, u8) = (trait_info(Traits::Agility).name, class_stats(Class::Ranger).agility);
            let ranger_int: (String, u8) = (trait_info(Traits::Intelligence).name, class_stats(Class::Ranger).intelligence);
            let ranger_cha: (String, u8) = (trait_info(Traits::Charisma).name, class_stats(Class::Ranger).charisma);
            let ranger_luc: (String, u8) = (trait_info(Traits::Luck).name, class_stats(Class::Ranger).luck);
            let ranger_stats = [ranger_str, ranger_for, ranger_agi, ranger_int, ranger_cha, ranger_luc];

            std::process::Command::new("clear").status().unwrap();
            println!("{}:", class_stats(Class::Ranger).name);
            for stat in ranger_stats {
                println!(" {} - {}", stat.0, stat.1);
            }
            println!("---\nChoose this class?\n1) Yes\n2) No");
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
                1 => return (false, class_stats(Class::Ranger).id),
                2 => return (true, 0),
                _ => {
                    println!("You must enter 1 or 2.");
                    return (true, 0);
                },
            };
        },
        4 => {
            let wizard_str: (String, u8) = (trait_info(Traits::Strength).name, class_stats(Class::Wizard).strength);
            let wizard_for: (String, u8) = (trait_info(Traits::Fortitude).name, class_stats(Class::Wizard).fortitude);
            let wizard_agi: (String, u8) = (trait_info(Traits::Agility).name, class_stats(Class::Wizard).agility);
            let wizard_int: (String, u8) = (trait_info(Traits::Intelligence).name, class_stats(Class::Wizard).intelligence);
            let wizard_cha: (String, u8) = (trait_info(Traits::Charisma).name, class_stats(Class::Wizard).charisma);
            let wizard_luc: (String, u8) = (trait_info(Traits::Luck).name, class_stats(Class::Wizard).luck);
            let wizard_stats = [wizard_str, wizard_for, wizard_agi, wizard_int, wizard_cha, wizard_luc];

            std::process::Command::new("clear").status().unwrap();
            println!("{}:", class_stats(Class::Wizard).name);
            for stat in wizard_stats {
                println!(" {} - {}", stat.0, stat.1);
            }
            println!("---\nChoose this class?\n1) Yes\n2) No");
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
                1 => return (false, class_stats(Class::Wizard).id),
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
        println!("Welcome, {:?}!", class_stats(player_save.class).name);
    }
    else if character_select == 2 {
        let player_save = Save {
            class: Class::Rogue,
            char_name: String::new(),
            page: 1,
            alignment: 0,
        };
        println!("Welcome, {:?}!", class_stats(player_save.class).name);
    }
    else if character_select == 3 {
        let player_save = Save {
            class: Class::Ranger,
            char_name: String::new(),
            page: 1,
            alignment: 0,
        };
        println!("Welcome, {:?}!", class_stats(player_save.class).name);
    }
    else if character_select == 4 {
        let player_save = Save {
            class: Class::Wizard,
            char_name: String::new(),
            page: 1,
            alignment: 0,
        };
        println!("Welcome, {:?}!", class_stats(player_save.class).name);
    }
}