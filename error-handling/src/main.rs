use std::fs::File;

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
    let guestlist_result = File::open("guests.txt");
    let guestslist = match guestlist_result {
        Ok(file) => file,
        Err(error) => panic!("Could not open guest list. Error: {}", error)     ,
    };
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