
use teloxide::{prelude::*, utils::command::BotCommands};

use crate::api::get_solana_balance;



#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Display all commands.")]
    Help,
    #[command(description = "Get a wallet solana balance.")]
    Balance(String),
    // #[command(description = "handle a username and an age.", parse_with = "split")]
    // UsernameAndAge { username: String, age: u8 },
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Balance(wallet) => {
            let balance = get_solana_balance(&wallet).await?;

            bot.send_message(
                msg.chat.id,
                format!("Your wallet {} has {} sol.", wallet, balance),
            )
            .await?
           
        }
        // Command::UsernameAndAge { username, age } => {
        //     bot.send_message(
        //         msg.chat.id,
        //         format!("Your username is @{username} and age is {age}."),
        //     )
        //     .await?
        // }
    };

    Ok(())
}

