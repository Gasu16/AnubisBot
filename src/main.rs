//#![allow(unused_imports)]
// Last edit: 22:21 - 31/10/2021
use teloxide::{prelude::*, types::{ChatPermissions, Me}, utils::command::BotCommand};
use std::env;
use std::error::Error;
use std::str;
use std::str::FromStr;
use std::process::Command;
use chrono::{DateTime, Duration, NaiveDateTime, Utc};

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "Lista comandi", parse_with = "split")]
enum Commands {
    #[command(description = "Show command list.")]
    Help,
    #[command(description = "Handle a macro.", parse_with = "split")]
    Macro {option: String, macro_str: String},
    #[command(description = "Ban a user.")]
    Ban, 
    #[command(description = "Kick a user.")]
    Kick,
    #[command(description = "Mute a user.", parse_with = "split")]    
    Mute {time: u64, unit: UnitOfTime},
    #[command(description = "Cancel the ban.")]
    Unban,
    #[command(description = "Check the logs.")]
    Logs,
    #[command(description = "Does the bot reply?")]
    Ping,
    #[command(description = "My github page.")]
    Info,
    #[command(description = "Do the math.")]
    Calc {x: u32, y: u32, operator: String},
}

enum UnitOfTime {
    Seconds,
    Minutes,
    Hours,
}

impl FromStr for UnitOfTime {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "h" | "hours" => Ok(UnitOfTime::Hours),
            "m" | "minutes" => Ok(UnitOfTime::Minutes),
            "s" | "seconds" => Ok(UnitOfTime::Seconds),
            _ => Err("Allowed units: h, m, s"),
        }
    }
}

// Calculates time of user restriction.
fn calc_restrict_time(time: u64, unit: UnitOfTime) -> Duration {
    match unit {
        UnitOfTime::Hours => Duration::hours(time as i64),
        UnitOfTime::Minutes => Duration::minutes(time as i64),
        UnitOfTime::Seconds => Duration::seconds(time as i64),
    }
}


type Cx = UpdateWithCx<AutoSend<Bot>, Message>;



async fn mute_user(cx: &Cx, time: Duration) -> Result<(), Box<dyn Error + Send + Sync>> {
    match cx.update.reply_to_message() {

        Some(msg1) => {


            let member_mute = cx.requester.get_chat_member(cx.update.chat_id(), cx.update.from().unwrap().id).send().await?;
            
            let _member_mute = member_mute.is_privileged();

            match _member_mute {

                true => {


                    let to_mute = cx.requester.get_chat_member(cx.update.chat_id(), msg1.from().unwrap().id).send().await?;
                    let _to_mute = to_mute.is_privileged();

                    match _to_mute {
                        
                        true => {
                            cx.reply_to("I cannot use this command on an admin").send().await?;
                        }

                        false => {                    
                            cx.requester
                                .restrict_chat_member(
                                cx.update.chat_id(),
                                msg1.from().expect("Must be MessageKind::Common").id,
                                ChatPermissions::default(),
                                )
                                .until_date(
                                    DateTime::<Utc>::from_utc(
                                        NaiveDateTime::from_timestamp(cx.update.date as i64, 0),
                                        Utc,
                                    ) + time,
                                )
                                .await?;
                            cx.answer(format!("{} has been muted until {}", msg1.from().unwrap().first_name, DateTime::<Utc>::from_utc(
                                        NaiveDateTime::from_timestamp(cx.update.date as i64, 0),
                                        Utc,
                                    ) + time)).await?;
                        }
                    }
                }

                false => {


                    cx.reply_to("You do not have enough privileges to use this command").send().await?;
                }
            }
        }

        None => {

            cx.reply_to("Use this command to reply to a message").send().await?;
        }
    }
    Ok(())
}


// Kicka un utente
async fn kick_user(cx: &Cx, str_msg: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    match cx.update.reply_to_message() {

        Some(mes) => {


            let member_kick = cx.requester.get_chat_member(cx.update.chat_id(), cx.update.from().unwrap().id).send().await?;
            
            let _member_kick = member_kick.is_privileged();
            
            match _member_kick {

                true => {

                    let to_kick = cx.requester.get_chat_member(cx.update.chat_id(), mes.from().unwrap().id).send().await?;
                    let _to_kick = to_kick.is_privileged();
                    
                    match _to_kick {
                        
                        true => {

                            cx.reply_to("I cannot use this command on an admin").send().await?;
                        }
                        
                        false => {
                            cx.requester
                                .unban_chat_member(cx.update.chat_id(), mes.from().unwrap().id)
                                .send()
                                .await?;
                            cx.answer(format!("{} {}", mes.from().unwrap().first_name, str_msg)).await?;
                        }
                    }
                }

                false => {


                    cx.reply_to("You do not have enough privileges to use this command").send().await?;
                }
            }

        }
        None => {
            // Non viene specificato nessun messaggio a cui rispondere
            
            cx.reply_to("Use this command to reply to a message").send().await?;
        }
    }
    Ok(())
}

// Banna un utente 
async fn ban_user(cx: &Cx) -> Result<(), Box<dyn Error + Send + Sync>> {
    match cx.update.reply_to_message() {
        Some(message) => {
            

            let member_ban = cx.requester.get_chat_member(cx.update.chat_id(), cx.update.from().unwrap().id).send().await?;
            
            let _member_ban = member_ban.is_privileged();

            match _member_ban {
                
                true => { 


                    let to_ban = cx.requester.get_chat_member(cx.update.chat_id(), message.from().unwrap().id).send().await?;
                    let _to_ban= to_ban.is_privileged();

                    match _to_ban {
            
                        true => {
                            cx.reply_to("I cannot use this command on an admin").send().await?;
                        }
                          
                        false => {
               
                            cx.requester
                                .kick_chat_member(
                                    cx.update.chat_id(),
                                    message.from().expect("Must be MessageKind::Common").id,
                                ).await?;
                            cx.answer(format!("{} has been banned", message.from().unwrap().first_name)).await?;
                        }
                    }
                } 

                false => {
                    cx.reply_to("You do not have enough privileges to use this command").send().await?;
                }
            }
        }
        None => {
            cx.reply_to("Use this command to reply to a message").send().await?;
        }
    }
    Ok(())
}

async fn action(cx: UpdateWithCx<AutoSend<Bot>, Message>, command: Commands) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {

        Commands::Help                           => {
            print_(&cx, &Commands::descriptions()).await?;
        }

        Commands::Logs                           => {
            print_(&cx, "@rootinit check the logs").await?;
        }

        Commands::Ping                           => {
            print_(&cx, "pong").await?;
        }

        Commands::Calc{x, y, operator}           => {
            match operator.as_str() {
                "+" | "add"                      => {
                    let a = x.checked_add(y);
                    match a {
                        Some(_v_add) => {
                            print_op(&cx, "", x+y).await?;
                        }
                        
                        None => {
                            print_(&cx, "Ops, an error occured, try again").await?;
                        }
                    };

                }

                "-" | "sub"                      => {
                    let s = x.checked_sub(y);
                    match s {
                        Some(_v_sub) => {
                            print_op(&cx, "", x-y).await?;
                        }
                        
                        None => {
                            print_(&cx, "Ops, an error occured, try again").await?;
                        }
                    };

                }

                "x" | "mul"                      => {
                    let m = x.checked_add(y);
                    match m {
                        Some(_v_mul) => {
                            print_op(&cx, "", x*y).await?;
                        }
                        
                        None => {
                            print_(&cx, "Ops, an error occured, try again").await?;
                        }
                    };

                }

                "/" | "div"                      => {
                    let d = x.checked_add(y);
                    match d {
                        Some(_v_div) => {
                            print_op(&cx, "", x/y).await?;
                        }
                        
                        None => {
                            print_(&cx, "Ops, an error occured, try again").await?;
                        }
                    };

                }

                "**"| "pow"                      => {
                    //let p = x.pow(y);
                    let p = x.checked_pow(y);
                    match p {
                        Some(_v) => {
                            print_op(&cx, "", x.pow(y)).await?;
                        }
                        
                        None => {
                            print_(&cx, "OOps, an error occured, try again").await?;
                        }
                    };
                }

                _                                => {
                    print_(&cx, "I didn't understand which operation should I do").await?;
                }
            }
        }

        Commands::Info                           => {
            print_(&cx, "https://github.com/Gasu16/AnubisBot").await?;    
        }

        Commands::Unban                          => {
            kick_user(&cx, "has been unbanned").await?;
        }

        Commands::Ban                            => {
            ban_user(&cx).await?;
        }
        
        Commands::Kick                           => {           
            kick_user(&cx, "has been banned").await?;
        }

        
        Commands::Mute{time, unit}               => {
            mute_user(&cx, calc_restrict_time(time, unit)).await?;
        }

        Commands::Macro{option, macro_str}       => {
            
            match option.as_str() {
            
                "-a" | "--add"                   => {
                    print_(&cx, "Macro added").await?;
                }

                "-e" | "--edit"                  => {
                    print_(&cx, "Macro edited").await?;
                }

                "-r" | "--remove"                => {
                    print_(&cx, "Macro removed").await?;
                }

                "-c" | "--to-ascii"              => {
                    
                    let mut cmd = Command::new("sh");
                    let j = ["echo", macro_str.as_str()].join(" ");
                    cmd.arg("-c").arg(j);
                    let _cmd = cmd.output().expect("Comando non letto correttamente");
                    print_with(&cx, "", _cmd.stdout).await?;

                }

                _                                => {
                    print_(&cx, "Command not found").await?;
                }
            }
        }

    };

    Ok(())
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn print_(cx: &Cx, to_print: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Err(e) = cx.answer(format!("{}", to_print)).await {
        println!("Error: {}", e.to_string());
    }
    Ok(())
}

async fn print_with(cx: &Cx, to_print_with: &str, to_arg_with: Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Err(er) = cx.answer(format!("{} {:?}", to_print_with, to_arg_with)).await {
        println!("Error: {}", er.to_string());
    }
    Ok(())
}

async fn print_op(cx: &Cx, to_print_op: &str, to_arg_op: u32) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Err(op_err) = cx.answer(format!("{} {:?}", to_print_op, to_arg_op)).await {
        println!("Error: {}", op_err.to_string());
    }
    Ok(())
}



async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting simple_commands_bot...");

    let _bot = Bot::from_env().auto_send();
     
    let Me {user: _bot_user, ..} = _bot.get_me().await.unwrap();
    
    let _bot_name: String = "INIT.D".into();
    teloxide::commands_repl(_bot, _bot_name, action).await;
}
