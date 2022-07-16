
// crate
use crate::data::models::character_model::CharacterModel;
use crate::data::models::player_model::PlayerModel;


pub struct UniqueCharacter {
    id: i64,
    character: CharacterModel,
    player: PlayerModel,
}

impl UniqueCharacter {

    /// Returns an instance of UniqueCharacter
    ///
    /// ## Arguments:
    /// * id - the unique character id
    /// * character - the character representing this character
    /// * player - the uniqe character owner
    pub fn new(id: i64, character: CharacterModel, player: PlayerModel) 
        -> Self {

        Self {
            id,
            character,
            player,
        }
    }

    /// Returns the unique character id
    pub fn id(&self) -> &i64 {
        &self.id
    }

    /// Returns the character that represents this unique character
    pub fn character(&self) -> &CharacterModel {
        &self.character
    }

    /// Returns the player that owns this character
    pub fn player(&self) -> &PlayerModel {
        &self.player
    }

}
