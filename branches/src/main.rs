use std::io;
use std::{thread, time::Duration};

fn main () {
    let mut num = String::new();

        println!("Please input a number.");

        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line.");

        let num: u64 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid entry. Don't you know what a number is? Exiting.");
                return;
            }
        };

    evaluate(num);
    let num = shop(num);
    countdown(num);
}

fn evaluate(num: u64) {
    if num < 5 {
        println!("{} is less than five.", num);
    } 
    else if num == 5 {
        println!("{} is 5.", num);
    }
    else {
        println!("{} is greater than five.", num);
    }
}

fn countdown(mut num: u64) {
    println!("Uh oh...");
    thread::sleep(Duration::from_secs(1));
    let bomb: char = '💣';
    let explosion = 'count_down: loop {
        if num > 0 {
        let dots: String = "💵".repeat(num.try_into().unwrap());
        println!("{bomb} {dots}🔥 {num}");
        num = num - 1;
        thread::sleep(Duration::from_secs(1));
        continue;
        }
        else {
            break 'count_down '💥';
        }
    };
    println!("{} BOOM! {}", explosion, explosion);
}

fn shop (mut money: u64) -> u64 {
    println!("==================\nWelcome to the shop!\n==================\nEnter the number of the item you want to buy.");
    'shop: while money > 0 {
        // Store stock
        let apples: (u32, &str, u32, char) = (1, "Apples", 2, '🍎');
        let bananas: (u32, &str, u32, char) = (2, "Bananas", 1, '🍌');
        let grapes: (u32, &str, u32, char) = (3, "Grapes", 3, '🍇');
        let watermelons: (u32, &str, u32, char) = (4, "Watermelons", 5, '🍉');
        let bombs: (u32, &str, u32, char) = (5, "Bombs", 0, '💣');
        let stock = [apples, bananas, grapes, watermelons, bombs];

        println!("You currently have ${}.", money);
        for item in stock {
            println!("{}. {} - ${}", item.0, item.1, item.2);
        }
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        let choice: u32 = match choice.trim().parse() {
            Ok(choice) => choice,
            Err(_) => {
                println!("Invalid entry. Try again.");
                continue;
            }
        };
        match choice {
            1 => {
                if money >= 2 {
                println!("You bought an apple. {}\n", apples.3);
                money = money - 2;
                }
                else {
                    println!("You don't have enough money for that.");
                    continue;
                }
            },
            2 => {
                if money >= 1 {
                println!("You bought a banana. {}\n", bananas.3);
                money = money - 1;
                }
                else {
                    println!("You don't have enough money for that.");
                    continue;
                }
            },
            3 => {
                if money >= 3 {
                println!("You bought some grapes. {}\n", grapes.3);
                money = money - 3;
                }
                else {
                    println!("You don't have enough money for that.");
                    continue;
                }
            },
            4 => {
                if money >= 5 {
                println!("You bought a watermelon. {}\n", watermelons.3);
                money = money - 5;
                }
                else {
                    println!("You don't have enough money for that.");
                    continue;
                }
            },
            5 => {
                println!("You bought a bomb. {}\nUsing your remaining money (${}) as a fuse...", bombs.3, money);
                break 'shop;
            },
            _ => {
                println!("Invalid choice. Try again.");
            }
        }
    }
    if money == 0 {
        println!("You're out of money. Here's a free bomb: 💣");
        money
    }
    else {
        money
    }
}