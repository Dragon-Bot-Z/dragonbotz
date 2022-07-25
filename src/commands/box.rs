
// lib
    // tokio-postgres
use tokio_postgres;

    // serenity
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::model::interactions::message_component::ButtonStyle;
use serenity::builder::CreateButton;

// crate
use crate::data::repository::unique_character_repository::{
    UniqueCharacterRepository,
    UniqueCharacterRepositoryTrait,
};

use crate::data::models::player_model::PlayerModel;
use crate::data::models::character_model::CharacterModel;

use crate::core::command::Command;
use crate::utils::utils::Utils;
use crate::utils::error::Error;


pub struct BoxCommand;

impl BoxCommand {

    /// Returns the total number of pages
    /// 
    /// ## Arguments:
    /// * number_of_characters - the total number of characters
    /// * display_per_page - the number of characters to display per page
    fn total_number_of_pages(number_of_characters: &u32, display_per_page: &u32) 
        -> u32 {
        
        // simply calculates the total number of pages by substracting the
        // integer number of pages from the real number of pages
        // giving a real number of pages between 0.0 and 0.9
        // if the real number of pages is > 0, increase the toal of pages by 1
        //
        // Example: 53 characters, display 10 per page
        // real = 53/10 = 5.3
        // integer = 5
        // difference = real - integer <=> 5.3 - 5 = 0.3
        // difference > 0.0, so total = integer + 1 = 6 pages
        let real_number_of_pages: f32 = (number_of_characters/display_per_page) as f32;
        let number_of_pages: u32 = number_of_characters/display_per_page;
        let difference: f32 = real_number_of_pages - *number_of_characters as f32;

        let mut total = number_of_pages;
        if total == 0 {
            total = 1;
        }

        if difference > 0.0 {
            total += 1;
        }

        total
    }

    /// Returns the slice indexes according to the page displayed, 
    /// the total number of characters and the number of characters
    /// to display per page
    /// 
    /// ## Arguments:
    /// * page - the page displayed
    /// * number_of_characters - the total number of characters
    /// * display_per_page - the number of character displayed per page
    fn page_slice(page: &u32, 
                  number_of_characters: &u32, 
                  display_per_page: &u32)
        -> [usize; 2] {

        let mut slice: [usize; 2] = [0, 0];
        slice[0] = (page * display_per_page) as usize;

        if number_of_characters-page * display_per_page > *display_per_page {
            slice[1] = ((display_per_page-1)+page*display_per_page) as usize;
        } else {
            slice[1] = *number_of_characters as usize;
        }
        
        slice
    }

}

#[async_trait]
trait BoxCommandTrait {

    /// Manages the player's box
    /// 
    /// ## Arguments:
    /// * player - the player
    /// * context - the context
    /// * characters - the player's characters
    /// * interaction - the command's interaction
    async fn manage_box(player: &PlayerModel, 
                        context: &Context,
                        characters: &Vec<(CharacterModel, i64)>,
                        interaction: &ApplicationCommandInteraction)
        -> Result<(), Error>;

    /// Displays the box page
    /// 
    /// ## Arguments:
    /// * context - the context
    /// * interaction - the command interaction
    /// * page - the page to display
    /// * characters - the characters to display
    /// * display_per_page - the number of characters to display per page
    async fn display_page(context: &Context,
                          interaction: &ApplicationCommandInteraction,
                          player: &PlayerModel,
                          page: &u32, 
                          characters: &Vec<(CharacterModel, i64)>,
                          display_per_page: &u32)
        -> Result<(), Error>;

    /// Allows the player to change the box page
    /// 
    /// ## Argupments:
    /// * context - the context
    /// * interaction - the interaction that triggered the box
    /// * player - the player to await
    /// * page - the starting page
    /// * number_of_characters - the total number of characters
    /// * display_per_page - the number of characters to display per page
    async fn page_switching(context: &Context,
                            interaction: &ApplicationCommandInteraction,
                            player: &PlayerModel,
                            page: &u32,
                            number_of_characters: &u32,
                            display_per_page: &u32)
        -> Result<(), Error>;

}

#[async_trait]
impl BoxCommandTrait for BoxCommand {

    async fn manage_box(player: &PlayerModel,
                        context: &Context, 
                        characters: &Vec<(CharacterModel, i64)>,
                        interaction: &ApplicationCommandInteraction)
        -> Result<(), Error> {

        // number of characters to display per box page
        let display_per_page = 10;

        BoxCommand::display_page(
            &context, &interaction, &player, &0, &characters, &display_per_page
        ).await?;
        
        Ok(())
    }

    async fn display_page(context: &Context, 
                          interaction: &ApplicationCommandInteraction, 
                          player: &PlayerModel,
                          page: &u32, 
                          characters: &Vec<(CharacterModel, i64)>, 
                          display_per_page: &u32)
        -> Result<(), Error> {

        if characters.len() == 0 {
            return Err(Error::Box("You don't have enough characters".to_string()))
        }

        let slice = BoxCommand::page_slice(
            &page, &(characters.len() as u32), &display_per_page
        );
        let characters_to_display = &characters[slice[0]..slice[1]];

        let mut embed = Utils::default_embed(&context.cache);
        embed.thumbnail(interaction.user.face());
        embed.title(format!("{}'s box", interaction.user.name));

        if let Err(error) = interaction.edit_original_interaction_response(
            &context.http, 
            |message| {
                let mut description = String::new();
                for character in characters_to_display {
                    description.push_str(
                        format!("{}: *x{}*\n", character.0.short_display(), character.1).as_str()
                    )
                }

                embed.description(description);
                message.content("");
                message.set_embed(embed)
            }
        ).await {

            return Err(Error::Box(format!("{} while displaying player's box content", error)))
        }

        BoxCommand::page_switching(
            &context, &interaction, &player, 
            &page, &(characters.len() as u32), &display_per_page
        ).await?;

        Ok(())
    }

    async fn page_switching(context: &Context,
                            interaction: &ApplicationCommandInteraction,
                            player: &PlayerModel,
                            page: &u32,
                            number_of_characters: &u32,
                            display_per_page: &u32)
        -> Result<(), Error> {

        let mut page = page.clone();
        let total_pages = BoxCommand::total_number_of_pages(
            &number_of_characters, &display_per_page
        );

        // set buttons
        // go to first page button
        let mut first_page_button = CreateButton::default();
        first_page_button
            .style(ButtonStyle::Primary)
            .custom_id("box_first_page_button")
            .label("<<");

        if page == 0 {
            first_page_button.disabled(true);
        }

        // go to previous page button
        let mut previous_page_button = CreateButton::default();
        previous_page_button
            .style(ButtonStyle::Primary)
            .custom_id("box_prev_page_button")
            .label("<");

        if page == 0 {
            previous_page_button.disabled(true);
        }

        // middle button displaying the current page
        let mut middle_button = CreateButton::default();
        middle_button
            .style(ButtonStyle::Secondary)
            .label(format!("{}/{}", page+1, total_pages))
            .custom_id("box_middle_button_button")
            .disabled(true);

        // go to next page button
        let mut next_page_button = CreateButton::default();
        next_page_button
            .style(ButtonStyle::Primary)
            .custom_id("box_next_page_button")
            .label(">");
        
        if page == total_pages-1 {
            next_page_button.disabled(true);
        }

        // go the last page button
        let mut last_page_button = CreateButton::default();
        last_page_button
            .style(ButtonStyle::Primary)
            .custom_id("box_last_page_button")
            .label(">>");

        if page == total_pages-1 {
            last_page_button.disabled(true);
        }

        // add buttons to the message
        if let Err(error) = interaction.edit_original_interaction_response(
            &context.http, 
            |message| {
                message.components(
                    |components| {
                        components.create_action_row(
                            |action_row| action_row
                                .add_button(first_page_button)
                                .add_button(previous_page_button)
                                .add_button(middle_button)
                                .add_button(next_page_button)
                                .add_button(last_page_button)
                        )
                    }
                )
            }
        ).await {

            return Err(Error::Box(format!("{} while adding components to the box message", error)))
        }

        Ok(())
    }

}

#[async_trait]
impl Command for BoxCommand {

    fn name(&self) -> String {
        "Box".to_string()
    }

    fn short_name(&self) -> String {
        "box".to_string()
    }

    fn description(&self) -> String {
        "Allows you to see which characters you own".to_string()
    }

    async fn run(&self, 
                 context: &Context, 
                 interaction: &ApplicationCommandInteraction, 
                 database: &tokio_postgres::Client)
        -> Result<(), Error> {

        let player = Utils::convert_user_id_to_player_model(&interaction.user.id)?;

        // get player's characters
        let unique_character_repository = UniqueCharacterRepository::new(&database);
        let characters = unique_character_repository
            .get_player_unique_characters_and_count(&player)
            .await?;

        BoxCommand::manage_box(&player, &context, &characters, &interaction).await?;

        Ok(())
    }

}
