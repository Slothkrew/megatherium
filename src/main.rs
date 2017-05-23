extern crate serenity;
extern crate json;

mod urls;
mod wheel;

use std::io::Read;
use serenity::client::Client;

fn main() {
    //println!("{}", exec_command(&"url find h".to_string(), &"sjums".to_string()).unwrap());

    let token = get_token();
    //println!("Token is {}", token);
    let mut client = Client::login(&token);
    client.on_ready(|_context, ready|{
        println!("Ready: {}#{}", ready.user.name, ready.user.discriminator);
    });
    client.on_message(|_context, message| {
        //println!("{}", message.content);
        if message.content.starts_with("!") {
            println!("executing {}", message.content);
            let resp = exec_command(&message.content[1..], &message.author.name);
            match resp {
                Some(resp) => {
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
                None => (),
            }
        }
    });

    let _ = client.start();
}

fn exec_command(command: &str, user: &str) -> Option<String> {
    let mut input = command.split_whitespace();
    let command = input.nth(0);
    let mut args = input;

    match command {
        Some("wheel") => {
            let arg = args.nth(0);
            if &arg == &Some("about") {
                Some(wheel::about())
            }
            else if &arg == &Some("help") {
                Some(wheel::help())
            }
            else {
                Some(wheel::spin())
            }
        },
        Some("url") => {
            //Todo: add, clear?, latest/newest, list, find, count, stats
            let arg = args.nth(0);
            match arg {
                Some("add") => {
                    let url = args.nth(0);
                    let desc = the_rest(args);
                    match url {
                        Some(url) => {
                            urls::add(&String::from(url), &desc, &String::from(user));
                            None
                        },
                        None => None
                    }
                },
                Some("help") => {
                    Some(urls::help())
                },
                Some("latest") | Some("newest") => {
                    let last_url = urls::get_last();
                    match last_url {
                        Some(url) => Some(url.to_string()),
                        None => None
                    }
                },
                Some("find") => {
                    let query = the_rest(args);
                    let query_res = urls::find(query);
                    match query_res {
                        Some(matches) => {
                            let mut ret_msg = String::new();
                            for m in matches {
                                ret_msg.push_str(&m.to_string());
                                ret_msg.push('\n');
                            }
                            Some(ret_msg)
                        },
                        None => None
                    }
                },
                Some("delete") => {
                    let url = args.nth(0);
                    match url {
                        Some(url) => {
                            urls::delete(&String::from(url), &String::from(user))
                        },
                        None => ()
                    };
                    None
                },
                Some("count") => {
                    let nick = args.nth(0);
                    match nick {
                        Some(nick) => {
                            let added = urls::count(Some(nick));
                            Some(format!("{} links found, added by {}", added, nick))
                        },
                        None => {
                            let added = urls::count(None);
                            Some(format!("{} delicious urls found in our collective collection", added))
                        }
                    }
                }
                _ => {
                    match urls::random() {
                        Some(url) => Some(url.to_string()),
                        None => None,
                    }
                }
            }
        },
        _ => None
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

fn get_token() -> String {
    let mut file = std::fs::File::open("config.json").expect("Could not file config.json file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read file content!");

    let data = json::parse(&contents).expect("Could not parse config.json");
    return String::from(data["bot_token"].as_str().expect("Could not read bot_token as string!"));
}