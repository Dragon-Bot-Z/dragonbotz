
// lib
    // chrono
use chrono;


pub struct PlayerModel {
    discord_id: i64,
    register_date: chrono::NaiveDate,
}

impl PlayerModel {

    /// Returns an instance of PlayerModel
    /// 
    /// ## Arguments:
    /// * discord_id - the player's discord id
    /// * register_date - the player's register date
    pub fn new(discord_id: i64, register_date: chrono::NaiveDate) -> Self {
        Self {
            discord_id,
            register_date,
        }
    }

    /// Returns a partially constructed PlayerModel containing only its Discord
    /// id
    /// 
    /// ## Arguments:
    /// * discord_id - the player's discord id
    pub fn new_partial_with_discord_id(discord_id: i64) -> Self {
        Self {
            discord_id,
            register_date: chrono::NaiveDate::from_ymd(1970, 1, 1),
        }
    }

    /// Returns the player's discord id
    pub fn discord_id(&self) -> &i64 {
        &self.discord_id
    }

    /// Returns the player's register date
    pub fn register_date(&self) -> &chrono::NaiveDate {
        &self.register_date
    }

}
