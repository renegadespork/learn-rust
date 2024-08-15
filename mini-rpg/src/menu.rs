use std::io;
pub struct MenuOption {
    id: u8,
    displaytext: String,
}
pub enum MainMenuSelection {
    NewGame,
    LoadSave,
    Exit,
    Invalid,
}

pub fn menu(options: &[String]) -> u8 {
    loop {
        std::process::Command::new("clear").status().unwrap();
        let index: u8 = 1;
        for option in options {
            println!("{}) {}", index, option);
            let index: u8 = index + 1;
        }
        let mut selection = String::new();
            io::stdin()
                .read_line(&mut selection)
                .expect("Failed to read line.");
            let selection: u8 = match selection.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("You must enter a number.");
                    continue;
                }
            };
            return selection;
    }
}