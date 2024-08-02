struct User {
    id: u64,
    firstname: String,
    lastname: String,
    email: String,
    active: bool,
}

impl User {
    fn fullname(&self) -> String {
        let fullname = String::from(&self.firstname) + &String::from(" ") + &self.lastname;
        fullname
    }
    fn emaildomain(&self) -> String {
        let domain_delimiter_index = self.email.find("@").unwrap_or(0) + 1;
        let emaildomain = String::from(&self.email[domain_delimiter_index..]);
        emaildomain
    }
    fn new_user(userid: u64) -> Self {
        Self {
            id: userid,
            firstname: String::from("user".to_owned() + &userid.to_string() + "_firstname"),
            lastname: String::from("user".to_owned() + &userid.to_string() + "_lastname"),
            email: String::from("user".to_owned() + &userid.to_string() + "@domain.com"),
            active: true,
        }
    }
}

fn main() {
    let user1 = User {
        id: 1,
        firstname: String::from("Bruce"),
        lastname: String::from("Wayne"),
        email: String::from("iambatman@gotham.com"),
        active: true,
    };

    let user2 = User {
        id: 2,
        firstname: String::from("Joker"),
        lastname: String::from("[Unknown]"),
        email: String::from("howigotthesescars@watchtheworldburn.io"),
        active: false,
    };

    let user3 = User {
        id: 3,
        firstname: String::from("James"),
        lastname: String::from("Gordon"),
        email: String::from("lightthesignal@pd.gotham.gov"),
        active: true,
    };
    let users = [&user1, &user2, &user3];
    println!("There are currently {}/{} active users.\n---", active_users(&users), user_count(&users));
    print_details(&user1);
    print_details(&user2);
    print_details(&user3);
    println!("Creating new user...");
    print_details(&User::new_user(4));
}

fn activity_checker (active: bool) -> String {
    if active {
        let activity = String::from("active");
        activity
    }
    else {
        let activity = String::from("inactive");
        activity
    }
}

fn print_details (user: &User) {
    let activity = activity_checker(user.active);
    println!("User ID {} is {}:\n Name: {}\n Email: {}\n Domain: {}\n", user.id, activity, user.fullname(), user.email,user.emaildomain());
}

fn active_users (users: &[&User; 3]) -> u32 {
    let mut count: u32 = 0;
    for user in users {
        if user.active {
        count = count + 1;
        }
        else {}
    }
    count
}

fn user_count (users: &[&User; 3]) -> u32 {
    let mut count: u32 = 0;
    for _user in users {
        count = count + 1;
    }
    count
}