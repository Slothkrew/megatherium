extern crate serenity;
extern crate json;

mod urls;
mod wheel;

use std::io::Read;
use serenity::client::Client;
use std::error::Error;
use urls::*;
use std::str::FromStr;

fn main() {
    //println!("{}", exec_command(&"url find h".to_string(), &"sjums".to_string()).unwrap());

    let config = get_config();
    //println!("Token is {}", token);
    let mut client = Client::login(&config.bot_token);
    client.on_ready(|_context, ready|{
        println!("Ready: {}#{}", ready.user.name, ready.user.discriminator);
    });
    client.on_message(move |_context, message| {
        //println!("{}", message.content);
        if message.content.starts_with("!") {
            println!("executing {}", message.content);
            let resp = exec_command(&message.content[1..], &message.author.name, &config);
            match resp {
                Ok(resp) => {
                    if &resp.len() > &2000 {
                        println!("Message is {} bytes longs. Truncating messages.", resp.len());
                        let _ = message.channel_id.say(&resp[..2000]);
                        let _ = message.channel_id.say(&"<Message truncated>".to_string());
                    }
                    else {
                        let _ = message.channel_id.say(&resp);
                    }
                    ()
                },
                _ => ()
            }
        }
    });

    let _ = client.start();
}

fn exec_command(command: &str, user: &str, config: &Config) -> Result<String, Box<Error>> {
    let mut input = command.split_whitespace();
    let command = input.nth(0);
    let mut args = input;

    match command {
        Some("wheel") => {
            let arg = args.nth(0);
            if &arg == &Some("about") {
                Ok(wheel::about())
            }
            else if &arg == &Some("help") {
                Ok(wheel::help())
            }
            else {
                Ok(wheel::spin())
            }
        },
        Some("url") => {
            //Todo: add, clear?, latest/newest, list, find, count, stats
            let url_mod = Urls::new(&config.sqlite_path);
            let arg = args.nth(0);
            match arg {
                Some("add") => {
                    let url = args.nth(0);
                    let desc = the_rest(args);
                    match url {
                        Some(url) => {
                            url_mod.add(&String::from(url), &desc, &String::from(user));
                            Err(From::from(""))
                        },
                        None => Err(From::from(""))
                    }
                },
                Some("help") => {
                    Ok(url_mod.help())
                },
                Some("latest") | Some("newest") => {
                    let last_url = url_mod.get_last()?;
                    Ok(last_url.to_string())
                },
                Some("find") => {
                    let query = the_rest(args);
                    let query_res = url_mod.find(query);
                    let mut ret_msg = String::new();
                    match query_res {
                        Ok(urls) => {
                            for m in urls {
                                ret_msg.push_str(&m.to_string());
                                ret_msg.push('\n');
                            }
                            Ok(ret_msg)
                         },
                        Err(_) => Ok("That's a 404!".to_string())
                    }
                },
                Some("delete") => {
                    let url = args.nth(0);
                    match url {
                        Some(url) => {
                            url_mod.delete(&String::from(url), &String::from(user))
                        },
                        _ => ()
                    };
                    Err(From::from(""))
                },
                Some("count") => {
                    let nick = args.nth(0);
                    match nick {
                        Some(nick) => {
                            let added = url_mod.count(Some(nick))?;
                            Ok(format!("{} links found, added by {}", added, nick))
                        },
                        None => {
                            let added = url_mod.count(None)?;
                            Ok(format!("{} delicious urls found in our collective collection", added))
                        }
                    }
                },
                Some("stats") => {
                    let stats = url_mod.stats()?;
                    Ok(stats)
                }
                _ => {
                    let rnd = url_mod.random()?;
                    Ok(rnd.to_string())
                }
            }
        },
        _ => Ok("You what bro?!".to_string())
    }
}


fn the_rest(args: std::str::SplitWhitespace) -> String {
    let mut out = String::new();
    for word in args {
        out.push_str(word);
        out.push(' ');
    }
    out.pop();
    out
}

struct Config {
    bot_token: String,
    #[allow(dead_code)] //for now :)
    sqlite_path: String
}

fn get_config() -> Config {
    let mut file = std::fs::File::open("config.json").expect("Could not file config.json file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read file content!");

    let data = json::parse(&contents).expect("Could not parse config.json");
    Config {
        bot_token: String::from(data["bot_token"].as_str().expect("Could not read bot_token as string!")),
        sqlite_path: String::from(data["sqlite_path"].as_str().expect("Could not read sqlite_path as string!"))
    }
}