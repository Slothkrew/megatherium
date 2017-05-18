//extern crate discord;
extern crate json;

mod urls;
mod wheel;

use std::io::Read;

fn main() {
    let token = get_token();
    println!("Token is {}", token);
    // let mut connection = discord::Discord::from_bot_token ("".to_string());
    // while let Some(event) = connection.recv_event() {

    // }

    let command = "unkown command".to_string();
    exec_command(command, "sjums".to_string());
}

fn exec_command(command: String, user: String) {
    let mut input = command.split_whitespace();
    let command = input.nth(0);
    let mut args = input;

    match command {
        Some("wheel") => {
            let arg = args.nth(0);
            if &arg == &Some("about") {
                respond(wheel::about());
            }
            else if &arg == &Some("help") {
                respond(wheel::help());
            }
            else {
                respond(wheel::spin());
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
                        Some(url) => urls::add(&String::from(url), &desc, &user),
                        None => (),
                    }
                },
                Some("help") => {
                    respond(urls::help());
                },
                Some("latest") | Some("newest") => {
                    let last_url = urls::get_last();
                    match last_url {
                        Some(url) => respond(url.to_string()),
                        None => (),
                    };
                },
                Some("find") => {
                    let query = the_rest(args);
                    let query_res = urls::find(query);
                    match query_res {
                        Some(matches) => {
                            for m in matches {
                                respond(m.to_string());
                            }
                        },
                        None => ()
                    };
                },
                _ => ()
            }
        },
        _ => ()
    }
}

fn respond(msg: String) {
    println!("{}", msg);
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