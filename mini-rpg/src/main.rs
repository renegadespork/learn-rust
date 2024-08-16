//VERY unfinished. Here be no dragons.

use mini_rpg::base::{class_stats, trait_info};
use mini_rpg::base::{CharClass, Save,Traits};
use mini_rpg::menu::menu;
use std::io;
use std::process;

fn main() {
    top_menu();
}

fn top_menu() {
    let header: String = String::from("Main Menu");
    let options: Vec<&str> = vec!["New Game", "Load Save", "Exit"];
    let top_menu_selection: u8 = menu(header, options);
    match top_menu_selection {
        1 => {
            let character_selection = character_select();
            let char_name = enter_name(character_selection);
            let player_save = Save {
                class: char_name.1,
                char_name: char_name.0,
                page: 1,
                alignment: 0,
            };
            println!(
                "---\n{} the {}\n---",
                player_save.char_name,
                class_stats(player_save.class).name
            );
            intro();
        },
        2 => {
            println!("Saves have not yet been implemented.");
            process::exit(0);
        },
        3 => {
            println!("Exiting...");
            process::exit(0);
        }
        _ => process::exit(1),
    }
}

fn character_select() -> CharClass {
    loop {
        let header: String = String::from("Select your character.");
        let options: Vec<&str> = vec!["Knight", "Rogue", "Ranger", "Wizard"];
        let selection: u8 = menu(header, options);
        let character_details_output: (bool, CharClass) = character_details(selection);
        let menu_loop = character_details_output.0;
        if menu_loop {
            continue;
        } else {
            return character_details_output.1;
        }
    }
}

fn character_details(selection: u8) -> (bool, CharClass) {
    match selection {
        1 => {
            let knight_str: (String, u8) = (
                trait_info(Traits::Strength).name,
                class_stats(CharClass::Knight).strength,
            );
            let knight_for: (String, u8) = (
                trait_info(Traits::Fortitude).name,
                class_stats(CharClass::Knight).fortitude,
            );
            let knight_agi: (String, u8) = (
                trait_info(Traits::Agility).name,
                class_stats(CharClass::Knight).agility,
            );
            let knight_int: (String, u8) = (
                trait_info(Traits::Intelligence).name,
                class_stats(CharClass::Knight).intelligence,
            );
            let knight_cha: (String, u8) = (
                trait_info(Traits::Charisma).name,
                class_stats(CharClass::Knight).charisma,
            );
            let knight_luc: (String, u8) = (
                trait_info(Traits::Luck).name,
                class_stats(CharClass::Knight).luck,
            );
            let knight_stats = [
                knight_str, knight_for, knight_agi, knight_int, knight_cha, knight_luc,
            ];

            std::process::Command::new("clear").status().unwrap();
            println!("{}:", class_stats(CharClass::Knight).name);
            for stat in knight_stats {
                println!(" {} - {}", stat.0, stat.1);
            }
            println!("---\nChoose this class?\n1) Yes\n2) No");
            let mut selection = String::new();
            io::stdin()
                .read_line(&mut selection)
                .expect("Failed to read line.");
            let selection: u32 = match selection.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("You must enter 1 or 2.");
                    return (true, CharClass::Knight);
                }
            };
            match selection {
                1 => return (false, CharClass::Knight),
                2 => return (true, CharClass::Knight),
                _ => {
                    println!("You must enter 1 or 2.");
                    return (true, CharClass::Knight);
                }
            };
        }
        2 => {
            let rogue_str: (String, u8) = (
                trait_info(Traits::Strength).name,
                class_stats(CharClass::Rogue).strength,
            );
            let rogue_for: (String, u8) = (
                trait_info(Traits::Fortitude).name,
                class_stats(CharClass::Rogue).fortitude,
            );
            let rogue_agi: (String, u8) = (
                trait_info(Traits::Agility).name,
                class_stats(CharClass::Rogue).agility,
            );
            let rogue_int: (String, u8) = (
                trait_info(Traits::Intelligence).name,
                class_stats(CharClass::Rogue).intelligence,
            );
            let rogue_cha: (String, u8) = (
                trait_info(Traits::Charisma).name,
                class_stats(CharClass::Rogue).charisma,
            );
            let rogue_luc: (String, u8) = (
                trait_info(Traits::Luck).name,
                class_stats(CharClass::Rogue).luck,
            );
            let rogue_stats = [
                rogue_str, rogue_for, rogue_agi, rogue_int, rogue_cha, rogue_luc,
            ];

            std::process::Command::new("clear").status().unwrap();
            println!("{}:", class_stats(CharClass::Rogue).name);
            for stat in rogue_stats {
                println!(" {} - {}", stat.0, stat.1);
            }
            println!("---\nChoose this class?\n1) Yes\n2) No");
            let mut selection = String::new();
            io::stdin()
                .read_line(&mut selection)
                .expect("Failed to read line.");
            let selection: u32 = match selection.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("You must enter 1 or 2.");
                    return (true, CharClass::Rogue);
                }
            };
            match selection {
                1 => return (false, CharClass::Rogue),
                2 => return (true, CharClass::Rogue),
                _ => {
                    println!("You must enter 1 or 2.");
                    return (true, CharClass::Rogue);
                }
            };
        }
        3 => {
            let ranger_str: (String, u8) = (
                trait_info(Traits::Strength).name,
                class_stats(CharClass::Ranger).strength,
            );
            let ranger_for: (String, u8) = (
                trait_info(Traits::Fortitude).name,
                class_stats(CharClass::Ranger).fortitude,
            );
            let ranger_agi: (String, u8) = (
                trait_info(Traits::Agility).name,
                class_stats(CharClass::Ranger).agility,
            );
            let ranger_int: (String, u8) = (
                trait_info(Traits::Intelligence).name,
                class_stats(CharClass::Ranger).intelligence,
            );
            let ranger_cha: (String, u8) = (
                trait_info(Traits::Charisma).name,
                class_stats(CharClass::Ranger).charisma,
            );
            let ranger_luc: (String, u8) = (
                trait_info(Traits::Luck).name,
                class_stats(CharClass::Ranger).luck,
            );
            let ranger_stats = [
                ranger_str, ranger_for, ranger_agi, ranger_int, ranger_cha, ranger_luc,
            ];

            std::process::Command::new("clear").status().unwrap();
            println!("{}:", class_stats(CharClass::Ranger).name);
            for stat in ranger_stats {
                println!(" {} - {}", stat.0, stat.1);
            }
            println!("---\nChoose this class?\n1) Yes\n2) No");
            let mut selection = String::new();
            io::stdin()
                .read_line(&mut selection)
                .expect("Failed to read line.");
            let selection: u32 = match selection.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("You must enter 1 or 2.");
                    return (true, CharClass::Ranger);
                }
            };
            match selection {
                1 => return (false, CharClass::Ranger),
                2 => return (true, CharClass::Ranger),
                _ => {
                    println!("You must enter 1 or 2.");
                    return (true, CharClass::Ranger);
                }
            };
        }
        4 => {
            let wizard_str: (String, u8) = (
                trait_info(Traits::Strength).name,
                class_stats(CharClass::Wizard).strength,
            );
            let wizard_for: (String, u8) = (
                trait_info(Traits::Fortitude).name,
                class_stats(CharClass::Wizard).fortitude,
            );
            let wizard_agi: (String, u8) = (
                trait_info(Traits::Agility).name,
                class_stats(CharClass::Wizard).agility,
            );
            let wizard_int: (String, u8) = (
                trait_info(Traits::Intelligence).name,
                class_stats(CharClass::Wizard).intelligence,
            );
            let wizard_cha: (String, u8) = (
                trait_info(Traits::Charisma).name,
                class_stats(CharClass::Wizard).charisma,
            );
            let wizard_luc: (String, u8) = (
                trait_info(Traits::Luck).name,
                class_stats(CharClass::Wizard).luck,
            );
            let wizard_stats = [
                wizard_str, wizard_for, wizard_agi, wizard_int, wizard_cha, wizard_luc,
            ];

            std::process::Command::new("clear").status().unwrap();
            println!("{}:", class_stats(CharClass::Wizard).name);
            for stat in wizard_stats {
                println!(" {} - {}", stat.0, stat.1);
            }
            println!("---\nChoose this class?\n1) Yes\n2) No");
            let mut selection = String::new();
            io::stdin()
                .read_line(&mut selection)
                .expect("Failed to read line.");
            let selection: u32 = match selection.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("You must enter 1 or 2.");
                    return (true, CharClass::Wizard);
                }
            };
            match selection {
                1 => return (false, CharClass::Wizard),
                2 => return (true, CharClass::Wizard),
                _ => {
                    println!("You must enter 1 or 2.");
                    return (true, CharClass::Wizard);
                }
            };
        }
        _ => {
            println!("Invalid Selection.");
            (true, CharClass::Wizard)
        }
    }
}

fn enter_name(character_select: CharClass) -> (String, CharClass) {
    loop {
        std::process::Command::new("clear").status().unwrap();
        let mut input = String::new();
        println!("Hello there, traveller. What is your name?");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let char_name = String::from(input.trim());

        std::process::Command::new("clear").status().unwrap();
        println!("{}? Did I hear that right?\n1) Yes\n2) No", char_name);
        let mut selection = String::new();
        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read line.");
        let selection: u32 = match selection.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You must enter 1 or 2.");
                continue;
            }
        };
        match selection {
            1 => {
                println!("Ah yes, welcome {}! Your journey begins...", char_name);
                break (char_name, character_select);
            }
            2 => continue,
            _ => {
                println!("You must enter 1 or 2.");
                continue;
            }
        }
    }
}

fn intro() {
    std::process::Command::new("clear").status().unwrap();
    println!("This is the introduction...");
    println!("Press any key to continue...");   
}