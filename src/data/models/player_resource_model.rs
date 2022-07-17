
// crate
use crate::data::models::player_model::PlayerModel;


pub struct PlayerResourceModel {
    owner: PlayerModel,
    summon_ticket_base: i64,
}

impl PlayerResourceModel {

    /// Returns an instance of PlayerResourceModel
    ///
    /// ## Arguments:
    /// * owner - the player that owns the resources
    /// * summon_ticket_base - the amount of summon tickets the player owns
    pub fn new(owner: PlayerModel, summon_ticket_base: i64) -> Self {
        Self {
            owner,
            summon_ticket_base,
        }
    }

}
