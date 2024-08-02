// This program doesn't do anything... yet...

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Item {
    Potion,
    Scroll,
}

enum Action {
    Attack(u32),
    Defend(u32),
    Move(Direction),
    Use(Item),
    Wait,
}

struct Enemy {
    health: u32,
    defense: u32,
    speed: u32,
}

fn main{}