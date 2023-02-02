use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::application::interaction::Interaction;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::prelude::{GuildId, Ready};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // When the bot is ready
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(crate::constants::GUILD_ID);

        // Register the command
        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| {
                command
                    .name("amir")
                    .description("Edit Amir's grades")
                    .create_option(|option| {
                        option
                            .name("grade")
                            .description("Grade change (-100 to 100)")
                            .kind(CommandOptionType::Number)
                            .min_number_value(-100.0)
                            .max_number_value(100.0)
                            .required(true)
                    })
            })
        })
        .await;
    }

    /// Handle interaction events
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            // If the user is not authorized, error out
            let author_uid = command.member.as_ref().unwrap().user.id.0;
            if !crate::constants::AUTHORIZED_USERS.contains(&author_uid) {
                if let Err(why) = command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message.content("Only the professor can edit Amir's grades")
                            })
                    })
                    .await
                {
                    println!("Cannot respond to slash command: {}", why);
                }
                return;
            }

            // Handle the command
            let content = match command.data.name.as_str() {
                "amir" => {
                    match command
                        .data
                        .options
                        .get(0)
                        .unwrap()
                        .resolved
                        .as_ref()
                        .unwrap()
                    {
                        CommandDataOptionValue::Number(grade) => {
                            println!("Grade change: {}", grade);

                            // If amir's grade file doesn't exist, create it
                            if !std::path::Path::new(crate::constants::GRADE_FILE_PATH).exists() {
                                std::fs::write(crate::constants::GRADE_FILE_PATH, "100").unwrap();
                            }

                            // Read the current grade
                            let mut current_grade =
                                std::fs::read_to_string(crate::constants::GRADE_FILE_PATH)
                                    .unwrap()
                                    .parse::<f64>()
                                    .unwrap();

                            // Modify the grade
                            current_grade += grade;

                            // Write the new grade
                            std::fs::write(
                                crate::constants::GRADE_FILE_PATH,
                                current_grade.to_string(),
                            )
                            .unwrap();

                            // Build the response
                            format!(
                                "<@{}>'s grade was {} by `{}%`. It is now `{}%`",
                                crate::constants::AMIR_ID,
                                match grade.is_sign_positive() {
                                    true => "increased",
                                    false => "reduced",
                                },
                                grade,
                                current_grade
                            )
                        }
                        _ => "Error. Invalid type.".to_string(),
                    }
                }
                _ => "Command not implemented.".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
}
