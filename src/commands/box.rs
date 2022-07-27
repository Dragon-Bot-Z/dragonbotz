
// lib
    // tokio-postgres
use tokio_postgres;

    // serenity
use serenity::async_trait;
use serenity::client::Context;

use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::component::ButtonStyle;
use serenity::model::application::component::ActionRowComponent;
use serenity::model::application::command::CommandOptionType;
use serenity::model::channel::ReactionType;
use serenity::model::id::EmojiId;

use serenity::builder::CreateButton;
use serenity::builder::CreateApplicationCommandOption;

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
use crate::utils::icons::Icons;


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
        let real_number_of_pages: f32 = *number_of_characters as f32 / *display_per_page as f32;
        let number_of_pages: u32 = number_of_characters/display_per_page;
        let difference: f32 = real_number_of_pages - number_of_pages as f32;
        
        let mut total = number_of_pages;
        if total == 0 {
            total = 1;
        }

        if difference > 0.0 && *number_of_characters > *display_per_page {
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
            slice[1] = ((display_per_page)+page*display_per_page) as usize;
        } else {
            slice[1] = *number_of_characters as usize;
        }
        
        slice
    }

    /// Checks the player has enough characters
    /// 
    /// ## Arguments:
    /// * characters - the player's characters
    fn check_player_has_enough_unique_characters(characters: &Vec<CharacterModel>)
        -> Result<(), Error> {

        if characters.len() == 0 {
            return Err(Error::BoxCommand("You don't have this character".to_string()))
        }

        Ok(())
    }

    /// Returns the proper character slice
    /// 
    /// ## Arguments:
    /// * page - the current page
    /// * number_of_characters - the total number of characters
    /// * display_per_page - the number of characters displayed on a page
    fn unique_characters_slice(page: &u32, 
                        characters: &Vec<CharacterModel>,
                        display_per_page: &u32)
        -> Vec<CharacterModel> {

        let slice = BoxCommand::page_slice(
            &page, &(characters.len() as u32), &display_per_page
        );
        
        characters[slice[0]..slice[1]].to_vec()
    }

}

#[async_trait]
trait BoxCommandTrait {

    /// Manages the player's box for characters
    /// 
    /// ## Arguments:
    /// * player - the player
    /// * context - the context
    /// * characters - the player's characters
    /// * interaction - the command's interaction
    async fn manage_box_characters(player: &PlayerModel, 
                                   context: &Context,
                                   characters: &Vec<(CharacterModel, i64)>,
                                   interaction: &ApplicationCommandInteraction)
        -> Result<(), Error> {
        
        // number of characters to display per box page
        let display_per_page = 5;

        BoxCommand::display_page_characters(
            &context, &interaction, &player, &0, &characters, &display_per_page
        ).await?;

        Ok(())
    }

    /// Manages the player's box for unique characters
    /// 
    /// ## Arguments:
    /// * player - the player
    /// * context - the context,
    /// * characters - the player's unique characters
    /// *interaction - the command's interaction
    async fn manage_box_unique_characters(player: &PlayerModel,
                                          context: &Context,
                                          characters: &Vec<CharacterModel>,
                                          interaction: &ApplicationCommandInteraction)
        -> Result<(), Error> {
        
        let display_per_page = 10;

        BoxCommand::display_page_unique_characters(
            &context, &interaction, &player, 
            &0, &characters, &display_per_page
        ).await?;

        Ok(())
    }

    /// Displays the box page
    /// 
    /// ## Arguments:
    /// * context - the context
    /// * interaction - the command interaction
    /// * page - the page to display
    /// * characters - the characters to display
    /// * display_per_page - the number of characters to display per page
    async fn display_page_characters(context: &Context,
                                     interaction: &ApplicationCommandInteraction,
                                     player: &PlayerModel,
                                     page: &u32, 
                                     characters: &Vec<(CharacterModel, i64)>,
                                     display_per_page: &u32)
        -> Result<(), Error> {

        if characters.len() == 0 {
            return Err(Error::BoxCommand("You don't have enough characters".to_string()))
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

            return Err(Error::BoxCommand(format!("{} while displaying player's box content", error)))
        }

        BoxCommand::page_switching_characters(
            &context, &interaction, &player, 
            &page, &(characters.len() as u32), &display_per_page, &characters
        ).await?;

        Ok(())
    }

    /// Displays the box page for unique characters
    /// 
    /// ## Arguments:
    /// * context - the context
    /// * interaction - the command interaction
    /// * page - the page to display
    /// * characters - the characters to display
    /// * display_per_page - the number of characters to display per page
    async fn display_page_unique_characters(context: &Context,
                                            interaction: &ApplicationCommandInteraction,
                                            player: &PlayerModel,
                                            page: &u32,
                                            characters: &Vec<CharacterModel>,
                                            display_per_page: &u32)
        -> Result<(), Error> {

        BoxCommand::check_player_has_enough_unique_characters(&characters)?;

        // find the characters to display
        let characters_slice = BoxCommand::unique_characters_slice(
            &page, &characters, &display_per_page
        );

        let mut embed = Utils::default_embed(&context.cache);
        embed.thumbnail(interaction.user.face());
        embed.title(format!("{}'s box", interaction.user.name));

        if let Err(error) = interaction.edit_original_interaction_response(
            &context.http, 
            |message| {
                let mut description = String::new();
                for character in &characters_slice {
                    description.push_str(
                        format!("{}\n", character.short_display()).as_str()
                    )
                }

                embed.description(description);
                message.content("");
                message.set_embed(embed)
            }
        ).await {

            return Err(Error::BoxCommand(format!("{error} while displaying player's unique characters box")))
        }

        BoxCommand::page_switching_unique_characters(
            &context, &interaction, &player,
            &page, &(characters.len() as u32), &display_per_page,
            &characters
        ).await?;

        Ok(())
    }

    /// Allows the player to change the box page
    /// 
    /// ## Argupments:
    /// * context - the context
    /// * interaction - the interaction that triggered the box
    /// * player - the player to await
    /// * page - the starting page
    /// * number_of_characters - the total number of characters
    /// * display_per_page - the number of characters to display per page
    /// * characters - the characters to display
    async fn page_switching_characters(context: &Context,
                                        interaction: &ApplicationCommandInteraction,
                                        player: &PlayerModel,
                                        page: &u32,
                                        number_of_characters: &u32,
                                        display_per_page: &u32,
                                        characters: &Vec<(CharacterModel, i64)>)
        -> Result<(), Error> {

        let mut page = page.clone();
        let total_pages = BoxCommand::total_number_of_pages(
            &number_of_characters, &display_per_page
        );

        BoxCommand::create_and_add_buttons_to_interaction(
            &context, &interaction, &page, &total_pages
        ).await?;

        BoxCommand::wait_for_user_to_click_box_button(
            &context, &interaction, &mut page, &total_pages
        ).await?;

        BoxCommand::display_page_characters(
            &context, &interaction, &player, &page, &characters, &display_per_page
        ).await?;

        Ok(())
    }

    /// Allows the player to change the characters unique box page
    /// 
    /// ## Argument:
    /// * context - the context
    /// * interaction - the command's interaction
    /// * player - the player to await
    /// * page - the starting page
    /// * number_of_characters - the total number of characters
    /// * display_per_page - the number of characters to display per page
    /// * characters - the characters to display
    async fn page_switching_unique_characters(context: &Context,
                                              interaction: &ApplicationCommandInteraction,
                                              player: &PlayerModel,
                                              page: &u32,
                                              number_of_characters: &u32,
                                              display_per_page: &u32,
                                              characters: &Vec<CharacterModel>)
        -> Result<(), Error> {

        // change page mutability
        let mut page = page.clone();
        let total_pages = BoxCommand::total_number_of_pages(
            &number_of_characters, &display_per_page
        );

        BoxCommand::create_and_add_buttons_to_interaction(
            &context, &interaction, &page, &total_pages
        ).await?;
        
        BoxCommand::wait_for_user_to_click_box_button(
            &context, &interaction, &mut page, &total_pages
        ).await?;

        BoxCommand::display_page_unique_characters(
            &context, &interaction, &player, &page, &characters, &display_per_page
        ).await?;

        Ok(())
    }

    /// Adds buttons to the interaction message
    /// 
    /// ## Arguments:
    /// * context - the context
    /// * interaction - the interaction
    /// * page - the current page
    /// * total_pages - the total number of pages
    async fn create_and_add_buttons_to_interaction(context: &Context,
                                                   interaction: &ApplicationCommandInteraction,
                                                   page: &u32,
                                                   total_pages: &u32)
        -> Result<(), Error> {

        let page = page.clone();

        // set buttons
        // close button
        let mut close_button = CreateButton::default();
        close_button
            .style(ButtonStyle::Danger)
            .custom_id("box_close_button")
            .emoji(Icons::CLOSE.emoji_id())
            .label("");

        // go to first page button
        let mut first_page_button = CreateButton::default();
        first_page_button
            .style(ButtonStyle::Primary)
            .custom_id("box_first_page_button")
            .emoji(Icons::ArrowLeftEnd.emoji_id())
            .label("");

        if page == 0 {
            first_page_button.disabled(true);
        }

        // go to previous page button
        let mut previous_page_button = CreateButton::default();
        previous_page_button
            .style(ButtonStyle::Primary)
            .custom_id("box_prev_page_button")
            .emoji(Icons::ArrowLeft.emoji_id())
            .label("");

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
            .emoji(Icons::ArrowRight.emoji_id())
            .label("");
        
        if page == total_pages-1 {
            next_page_button.disabled(true);
        }

        // go the last page button
        let mut last_page_button = CreateButton::default();
        last_page_button
            .style(ButtonStyle::Primary)
            .custom_id("box_last_page_button")
            .emoji(Icons::ArrowRightEnd.emoji_id())
            .label("");

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
                        );

                        components.create_action_row(
                            |second_ar| second_ar
                                .add_button(close_button)
                        )
                    }
                )
            }
        ).await {

            return Err(Error::BoxCommand(format!("{} while adding components to the box message", error)))
        }

        Ok(())
    }

    /// Waits for the user to click a box button
    async fn wait_for_user_to_click_box_button(context: &Context,
                                               interaction: &ApplicationCommandInteraction,
                                               page: &mut u32,
                                               total_pages: &u32)
        -> Result<(), Error> {
        
        // get the response message to await interaction on it
        let box_message = match interaction.get_interaction_response(&context.http).await {
            Ok(box_message) => box_message,
            Err(error) => return Err(Error::BoxCommand(format!("{error} while waiting for player's interaction"))),
        };

        let component_interaction = box_message
            .await_component_interaction(&context.shard)
            .author_id(interaction.user.id)
            .message_id(box_message.id)
            .timeout(std::time::Duration::from_secs(60 * 2))
            .await;
        
        // if timeout
        // disable all buttons
        if let None = component_interaction {
            if let Err(error) = box_message.delete(&context.http).await {
                return Err(Error::BoxCommand(format!("{error} while trying to delete box message after time out")))
            }

            return Ok(())
        }

        let button_clicked = component_interaction.unwrap();
        
        // remove the error message on interaction
        if let Err(_) = button_clicked
            .create_interaction_response(&context.http, |response| response)
            .await {}

        // change the next page to display
        let mut delete_message = false;
        match button_clicked.data.custom_id.as_str() {
            "box_first_page_button" => *page = 0,
            "box_prev_page_button" => *page -= 1,
            "box_next_page_button" => *page += 1,
            "box_last_page_button" => *page = total_pages-1,
            "box_close_button" => delete_message = true,
            &_ => *page = 0,
        };

        if delete_message {
            if let Err(error) = box_message.delete(&context.http).await {
                return Err(Error::BoxCommand(format!("{error} while trying to delete box message use asked to")))
            }
        }

        Ok(())
    }

}

#[async_trait]
impl BoxCommandTrait for BoxCommand {}

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

    fn options(&self) -> Option<Vec<CreateApplicationCommandOption>> {
        
        // add character option
        let mut box_type_option = CreateApplicationCommandOption::default();
        box_type_option
            .kind(CommandOptionType::String)
            .name("type")
            .description("Allows you to choose what kind of data you want to display, by default, displays your characters")
            .add_string_choice("character", "character")
            .required(true);
        
        let mut box_filter_option = CreateApplicationCommandOption::default();
        box_filter_option
            .kind(CommandOptionType::String)
            .name("filter")
            .description("Allows you to filter out the data displayed, by default, it is set to \"all\"")
            .add_string_choice("all", "all")
            .add_string_choice("unique", "unique");
        
        let mut box_identifier_option = CreateApplicationCommandOption::default();
        box_identifier_option
            .kind(CommandOptionType::Integer)
            .name("identifier")
            .description("The data identifier, ignored if the filter is set to \"all\", required otherwise");
            
        Some(vec![box_type_option, box_filter_option, box_identifier_option])
    }

    async fn run(&self, 
                 context: &Context, 
                 interaction: &ApplicationCommandInteraction, 
                 database: &tokio_postgres::Client)
        -> Result<(), Error> {

        let options = match self.options_map(&interaction) {
            Some(options) => options,
            None => return Err(Error::BoxCommand("No parameter specified, please at least pass the box type".to_string()))
        };

        if options.contains_key("type") {
            let player = Utils::convert_user_id_to_player_model(&interaction.user.id)?;
            let unique_character_repository = UniqueCharacterRepository::new(&database);

            // get the type of the box
            if let Some(box_type) = &options["type"].value {
                if box_type == "character" {

                    if options.contains_key("filter") {
                        if let Some(box_filter) = &options["filter"].value {
                            // display unique character box
                            if box_filter == "unique" {
                                // check if the unique id had been passed
                                if options.contains_key("identifier") {
                                    // the identifier had been passed
                                    if let Some(box_id) = &options["identifier"].value {
                                        // get the identifier value
                                        let id = match box_id.as_i64().clone() {
                                            Some(id) => id,
                                            None => return Err(
                                                Error::BoxCommand(
                                                    "Unable to parse the identifier".to_string()
                                                )
                                            )
                                        };

                                        let characters = unique_character_repository
                                            .get_player_unique_characters_with_id(&player, &id)
                                            .await?;
                                        
                                        BoxCommand::manage_box_unique_characters(
                                            &player, &context, &characters, &interaction
                                        ).await?;
                                    }

                                } else {
                                    return Err(
                                        Error::BoxCommand(
                                            "The identifier is required for unique characters".to_string()
                                        )
                                    )
                                }
                            }
                        }
                    }

                    // display character box
                    let characters = unique_character_repository
                        .get_player_unique_characters_and_count(&player)
                        .await?;
                    BoxCommand::manage_box_characters(
                        &player, &context, &characters, &interaction
                    ).await?;
                }
            }

        } else {
            return Err(Error::BoxCommand("The box type parameter is required".to_string()))
        }

        Ok(())
    }

}
