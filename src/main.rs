//#![allow(unused_imports)]
// Last edit: 13:26 - 21/11/2021 
use teloxide::{prelude::*, types::{ChatPermissions, Me}, utils::command::BotCommand};
use std::env;
use std::error::Error;
use std::str;
use std::str::FromStr;
use std::process::Command;
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
//use teloxide::payloads::SendPhoto;
//use teloxide::types::InputFile;
//use std::borrow::Cow;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "Lista comandi")]
enum Commands {
    #[command(description = "Show command list.")]
    Help,
    #[command(description = "Handle a macro.", parse_with = "split")]
    Macro {option: String, macro_str: String},
    #[command(description = "Ban a user.")]
    Ban {reason: String}, 
    #[command(description = "Kick a user.")]
    Kick {reason_k: String},
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
    #[command(description = "Do the math.", parse_with = "split")]
    Calc {x: u32, y: u32, operator: String},
    #[command(description = "Search on Google")]
    Google {query: String},
    #[command(description = "Check updated news")]
    News,
    #[command(description = "Get pics")]
    Pic {picquery: String},
    #[command(description = "Search pics in Pixabay")]
    Pix {pixquery: String},
    #[command(description = "Ancient Egypt Archeology")]
    Arch,
    #[command(description = "Search on Wikipedia")]
    Wiki {res: String},
    #[command(description = "Which blogs should I read")]
    Blog,
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


// Mute a user for a specific amount of time
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
                            cx.reply_to(format!("{} has been muted until {}", msg1.from().unwrap().first_name, DateTime::<Utc>::from_utc(
                                        NaiveDateTime::from_timestamp(cx.update.date as i64, 0),
                                        Utc,
                                    ) + time)).send().await?;
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


// Kick a user
async fn kick_user(cx: &Cx, str_msg: &str, reason_k: String) -> Result<(), Box<dyn Error + Send + Sync>> {
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
                            let mut rsn_k = "";
                            let mut r_k = rsn_k.to_owned();
                            cx.requester
                                .unban_chat_member(cx.update.chat_id(), mes.from().unwrap().id)
                                .send()
                                .await?;
                            
                            if reason_k.is_empty() == false {
                                rsn_k = "Reason: ";
                                r_k = rsn_k.to_owned() + &reason_k;
                            }

                            cx.reply_to(format!("{} {}\n{}", mes.from().unwrap().first_name, str_msg, r_k)).send().await?;
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

// Ban a user
async fn ban_user(cx: &Cx, reason: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    match cx.update.reply_to_message() {
        Some(message) => {
            

            let member_ban = cx.requester.get_chat_member(cx.update.chat_id(), cx.update.from().unwrap().id).send().await?;
            
            let _member_ban = member_ban.is_privileged();

            match _member_ban {
                
                true => { 

                    // If true, an admin is calling the command
                    // Check if who is "go up" the command is an admin or a normal user

                    let to_ban = cx.requester.get_chat_member(cx.update.chat_id(), message.from().unwrap().id).send().await?;
                    let _to_ban= to_ban.is_privileged();

                    match _to_ban {
                        // If true, the user who is undergo the command is an admin, so is not a
                        // good idea to ban him
                        true => {
                            cx.reply_to("I cannot use this command on an admin").send().await?;
                        }
                        
                        // It's a normal user, so it is allowed for an admin to invoke this command
                        // on a user
                        false => {
                            
                            let mut rsn = "";
                            let mut r = rsn.to_owned();
                            cx.requester
                                .kick_chat_member(
                                    cx.update.chat_id(),
                                    message.from().expect("Must be MessageKind::Common").id,
                                ).await?;
                            if reason.is_empty() == false {
                                rsn = "Reason: ";
                                r = rsn.to_owned() + &reason;
                            }

                            cx.reply_to(format!("{} has been banned\n{}", message.from().unwrap().first_name, r)).send().await?;
                            //cx.answer(format!("{} has been banned", message.from().unwrap().first_name)).await?;
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

        Commands::Google{query}                  => {
            let result = query.replace(" ", "+");
            cx.reply_to(format!("https://www.google.com/search?q={}", result)).send().await?;
        }

        Commands::News                           => {
            cx.reply_to(format!("https://www.ancientegyptalive.com/blog/")).send().await?;
        }

        Commands::Arch                           => {
            cx.reply_to(format!("http://www.ancientegyptarchaeologyfund.com/blog/")).send().await?;
        }

        Commands::Wiki{res}                      => {
            let req = res.replace(" ", "_");
            cx.reply_to(format!("https://en.wikipedia.org/wiki/{}", req)).send().await?;
        }

        Commands::Blog                           => {
            cx.reply_to(format!("https://tombraiderhorizons.com/2018/03/06/8-egyptology-blogs-you-should-be-following/")).send().await?;
        }

        Commands::Pic{picquery}                  => {
            let result_pic = picquery.replace(" ", "+");
            cx.reply_to(format!("https://unsplash.com/s/photos/{}", result_pic)).send().await?;
        }

        Commands::Pix{pixquery}                  => {
            let result_pix = pixquery.replace(" ", "+");
            cx.reply_to(format!("https://pixabay.com/images/search/{}", result_pix)).send().await?;
        }

/*
        Commands::News                           => {
            let a: u8 = 0; 
            let s = "egypt".to_string();
            cx.answer_photo(InputFile::Memory{file_name: "/home/matt/Immagini/AncientEgyptBOTPICS/2020082405030838.jpg".to_string(), data: Cow::from(&s)}).send().await?;
           print_(&cx, "Image sent").await?;
        }
*/
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
                            print_(&cx, "Ops, an error has occured, try again").await?;
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
            kick_user(&cx, "has been unbanned", "".to_string()).await?;
        }

        Commands::Ban{reason}                    => {
            ban_user(&cx, reason).await?;
        }
        
        Commands::Kick{reason_k}                 => {           
            kick_user(&cx, "has been kicked", reason_k).await?;
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
                    let _cmd = cmd.output().expect("Command error");
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
    if let Err(e) = cx.reply_to(format!("{}", to_print)).send().await {
        println!("Error: {}", e.to_string());
    }
    Ok(())
}

async fn print_with(cx: &Cx, to_print_with: &str, to_arg_with: Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Err(er) = cx.reply_to(format!("{} {:?}", to_print_with, to_arg_with)).send().await {
        println!("Error: {}", er.to_string());
    }
    Ok(())
}

async fn print_op(cx: &Cx, to_print_op: &str, to_arg_op: u32) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Err(op_err) = cx.reply_to(format!("{} {:?}", to_print_op, to_arg_op)).send().await {
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
