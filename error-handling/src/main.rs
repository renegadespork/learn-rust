use std::fs::File;
use std::io::{ErrorKind, Read};

struct GiftDetails {
    name: String,
    emoji: char,
}

enum Gift {
    Cake,
    Bomb,
}

fn gift_details(gift: &Gift) -> GiftDetails {
    match gift {
        Gift::Cake => GiftDetails {
            name: String::from("Cake"),
            emoji: '🎂',
        },
        Gift::Bomb => GiftDetails {
            name: String::from("Bomb"),
            emoji: '💥',
        }
    }
}

fn main() {
    let gift: Gift = Gift::Cake;
    let emoji = gift_details(&gift).emoji;
    let guests = ["Charlie", "Melinda", "Jenny", "Prudence"];
    read_guest_list();
    println! ("Here is your gift: {}", emoji);
    open_gift(gift);
    print_guest_name(&guests, 6);
}

fn open_gift(gift: Gift) {
    match gift {
        Gift::Cake => {
            let result: char = '🎉';
            println!("{} Happy Birthday! {}", result, result);
        },
        Gift::Bomb => {
            let result: char = '💥';
            panic!("{} Happy Birthday! {}", result, result);
        },
    }
}

fn print_guest_name(guests: &[&str], mut guestnum: usize) {
    let guestnum = *&mut guestnum + 1;
    let suffix: String = suffix(&guestnum);
    println!("Here comes your {}{} guest, {}!", guestnum, suffix, guests[guestnum]);
}

fn suffix(index: &usize) -> String {
    if *index == 1 {
        let suffix = String::from("st");
        suffix
    }
    else if *index == 2 {
        let suffix = String::from("nd");
        suffix
    }
    else if *index == 3 {
        let suffix = String::from("rd");
        suffix
    }
    else {
        let suffix = String::from("th");
        suffix
    }
}

fn read_guest_list() -> String {
    let mut guests = String::new();
    let guestlist_result = File::open("guests.txt");
    let guestslist = match guestlist_result {
        Ok(mut file) => {
            file.read_to_string(&mut guests);
            return guests
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("guests.txt") {
                Ok(mut created_file) => {
                    created_file.read_to_string(&mut guests);
                    return guests
                },
                Err(e) => {
                    panic!("Guest list doesn't exist and could not be created. Reason: {:?}", e);
                },
            },
            other_error => {
                panic!("Guest list could not be opened. Reason: {:?}", other_error);
            },
        },
    };
}