mod urls;
mod wheel;

fn main() {
    let command = "url find something".to_string();
    exec_command(command, format!("sjums"));
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
                    respond(last_url.to_string());
                },
                Some("find") => {
                    let query = the_rest(args);
                    let matches = urls::find(query);
                    for m in matches {
                        respond(m.to_string());
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
                    let added = match nick {
                        Some(nick) => {
                            let added = urls::count(Some(nick));
                            Some(format!("{} links found, added by {}", added, nick))
                        },
                        None => {
                            let added = urls::count(None);
                            Some(format!("{} delicious urls found in our collective collection", added))
                        }
                    };
                }
                _ => None
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
