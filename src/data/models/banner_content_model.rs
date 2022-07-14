
// crate
use crate::data::models::character_model::CharacterModel;
use crate::data::models::banner_model::BannerModel;


pub struct BannerContentModel {
    characters: Vec<CharacterModel>,
    banner: BannerModel,
}

impl BannerContentModel {

    /// Returns an instance of BannerContentModel
    ///
    /// ## Arguments:
    /// * characters - a vector of characters
    /// * banner - the banner
    pub fn new(characters: Vec<CharacterModel>, banner: BannerModel) -> Self {
        Self {
            characters,
            banner,
        }
    }

    /// Returns the banner characters
    pub fn characters(&self) -> &Vec<CharacterModel> {
        &self.characters
    }

    /// Returns the banner 
    pub fn banner(&self) -> &BannerModel {
        &self.banner
    }

}
