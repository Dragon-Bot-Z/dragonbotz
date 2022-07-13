
// crate
use crate::utils::utils::Utils;
use crate::utils::rarity::Rarity;


pub struct CharacterModel {
    id: i32,
    name: String,
    rarity: i16,
    image: String,
    thumbnail: String,
}

impl CharacterModel {

    /// Returns an instance of CharacterModel
    /// 
    /// ## Arguments:
    /// * id - the character's id
    /// * name - the character's name
    /// * rarity - the character's rarity
    /// * image - the character's image
    /// * thumbnail - the character's thumbnail
    pub fn new(id: i32, 
               name: String, 
               rarity: i16, 
               image: String, 
               thumbnail: String)
        -> Self {

        Self {
            id,
            name,
            rarity,
            image,
            thumbnail,
        }        
    }

    /// Returns the character id
    pub fn id(&self) -> &i32 {
        &self.id
    }

    /// Returns the character name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns the character rarity
    pub fn rarity(&self) -> &i16 {
        &self.rarity
    }

    /// Returns character rarity converted
    pub fn rarity_converted(&self) -> Rarity {
        Utils::convert_rarity(&self.rarity)
    }

    /// Returns the character image
    pub fn image(&self) -> &String {
        &self.image
    }

    /// Returns the character thumbnail
    pub fn thumbnail(&self) -> &String {
        &self.thumbnail
    } 

}
