
// lib
    // serenity
use serenity::builder::CreateEmbed;
use serenity::cache::Cache;

// crate
use crate::utils::utils::Utils;
use crate::utils::icons::Icons;
use crate::utils::rarity::Rarity;
use crate::utils::colors::Colors;


#[derive(Clone)]
pub struct CharacterModel {
    id: i32,
    name: String,
    rarity: i16,
    image: String,
    thumbnail: String,
    is_origins: bool,
    unique_id: Option<i64>,
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
    /// * is_origins - tell if the character is origins
    /// * unique_id - [Optional] the character's unique id
    pub fn new(id: i32, 
               name: String, 
               rarity: i16, 
               image: String, 
               thumbnail: String,
               is_origins: bool,
               unique_id: Option<i64>)
        -> Self {

        Self {
            id,
            name,
            rarity,
            image,
            thumbnail,
            is_origins,
            unique_id,
        }        
    }

    /// Returns the character id
    pub fn id(&self) -> &i32 {
        &self.id
    }

    /// Returns the formatted id
    pub fn id_formatted(&self) -> String {
        let mut id = *self.id() as i64;
        
        if let Some(unique_id) = *self.unique_id() {
            id = unique_id;
        }
        
        format!("`#{}`", id).to_string()
    }

    /// Returns the character name
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn name_formatted(&self) -> String {
        format!("**{}**", self.name()).to_string()
    }

    /// Returns the character rarity
    pub fn rarity(&self) -> &i16 {
        &self.rarity
    }

    /// Returns character rarity converted
    pub fn rarity_converted(&self) -> Rarity {
        Rarity::from_id(self.rarity())
    }

    /// Returns the character image
    pub fn image(&self) -> &String {
        &self.image
    }

    /// Returns the character thumbnail
    pub fn thumbnail(&self) -> &String {
        &self.thumbnail
    } 

    /// Tells if the character is Origins
    pub fn is_origins(&self) -> &bool {
        &self.is_origins
    }

    /// Returns the characters unique id
    pub fn unique_id(&self) -> &Option<i64> {
        &self.unique_id
    }

    /// Returns the summon-like display card of the character
    /// 
    /// ## Arguments:
    /// * cache - the cache
    pub fn summon_display(&self, cache: &Cache) -> CreateEmbed {
        let mut embed = Utils::default_embed(cache);

        let mut rarity = format!(
            "{} {}", 
            Icons::from_rarity(&self.rarity_converted()),
            self.rarity_converted()
        );

        if *self.is_origins() {
            rarity = format!(
                "{}{} {} {}",
                Icons::from_rarity(&self.rarity_converted()),
                Icons::ORIGINS,
                Rarity::from_id(&self.rarity()),
                Rarity::ORIGINS
            )
        }

        embed.description(
            format!(
                "Name: {} - {}
Rarity: {}",
                self.id_formatted(),
                self.name_formatted(),
                rarity
            )
        );

        embed.thumbnail(self.thumbnail());
        embed.image(self.image());
        embed.color(Colors::from_rarity(&self.rarity_converted()));

        embed
    }

    /// Returns the short display of the character
    pub fn short_display(&self) -> String {
        let mut rarity = format!("{}", Icons::from_rarity(&self.rarity_converted()));
        if *self.is_origins() {
            rarity = format!(
                "{}{}", 
                Icons::from_rarity(&self.rarity_converted()),
                Icons::ORIGINS
            );
        }

        format!("{} - {}{}", self.id_formatted(), self.name_formatted(), rarity)
    }

}
