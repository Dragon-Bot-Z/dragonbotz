
// lib
    // chrono
use chrono;


pub struct PlayerModel {
    discord_id: u64,
    register_date: chrono::NaiveDate,
}

impl PlayerModel {

    /// Returns an instance of PlayerModel
    /// 
    /// ## Arguments:
    /// * discord_id - the player's discord id
    /// * register_date - the player's register date
    pub fn new(discord_id: u64, register_date: chrono::NaiveDate) -> Self {
        Self {
            discord_id,
            register_date,
        }
    }

    /// Returns the player's discord id
    pub fn discord_id(&self) -> &u64 {
        &self.discord_id
    }

    /// Returns the player's register date
    pub fn register_date(&self) -> &chrono::NaiveDate {
        &self.register_date
    }

}
