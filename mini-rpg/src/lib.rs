pub mod base {
    pub enum Traits {
        Strength,
        Fortitude,
        Agility,
        Intelligence,
        Charisma,
        Luck,
    }
    pub struct TraitInfo {
        pub id: u8,
        pub name: String,
    }
    pub fn trait_info(traits: Traits) -> TraitInfo {
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
    pub enum CharClass {
        Knight,
        Rogue,
        Ranger,
        Wizard,
    }
    pub struct CharClassStats {
        pub id: u8,
        pub name: String,
        pub strength: u8,
        pub fortitude: u8,
        pub agility: u8,
        pub intelligence: u8,
        pub charisma: u8,
        pub luck: u8,
    }
    pub struct Stats {
        pub max_health: u32,
        pub max_stamina: u32,
        pub max_mana: u32,
    }

    pub struct Save {
        pub class: CharClass,
        pub char_name: String,
        pub page: u64,
        pub alignment: i32,
    }

    pub fn class_stats(class: CharClass) -> CharClassStats {
        match class {
            CharClass::Knight => CharClassStats {
                id: 1,
                name: String::from("Knight"),
                strength: 8,
                fortitude: 8,
                agility: 2,
                intelligence: 3,
                charisma: 4,
                luck: 5,
            },
            CharClass::Rogue => CharClassStats {
                id: 2,
                name: String::from("Rogue"),
                strength: 3,
                fortitude: 4,
                agility: 7,
                intelligence: 2,
                charisma: 7,
                luck: 7,
            },
            CharClass::Ranger => CharClassStats {
                id: 3,
                name: String::from("Ranger"),
                strength: 5,
                fortitude: 4,
                agility: 8,
                intelligence: 4,
                charisma: 4,
                luck: 5,
            },
            CharClass::Wizard => CharClassStats {
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
}

pub mod menu {
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
}
