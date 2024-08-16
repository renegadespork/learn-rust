use std::io;
pub struct MenuOption {
    id: u8,
    displaytext: String,
}

pub fn menu(header: String, options: Vec<&str>) -> u8 {
    loop {
        std::process::Command::new("clear").status().unwrap();
        let mut h: usize = header.len();
        let mut separator = String::new();
        while h > 0 {
            separator = separator + "=";
            h = *&mut h - 1;
        }
        println!("{}\n{}", header, separator);
        for (index, option) in options.iter().enumerate() {
            let x: usize = index + 1;
            println!("{}) {}", x, option);
            
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