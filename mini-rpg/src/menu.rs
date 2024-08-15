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