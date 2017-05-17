mod urls;
mod wheel;

fn main() {
    let command = "url help".to_string();
    exec_command(command, format!("sjums"));
}

fn exec_command(command: String, user: String) {
    let mut input = command.split_whitespace();
    let command = input.nth(0);
    let mut args = input;

    let ret = match command {
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
                        Some(url) => urls::add(&String::from(url), &desc, &user),
                        None => (),
                    }
                    None
                },
                Some("help") => {
                    Some(urls::help())
                },
                _ => {
                    None
                }
            }
        },
        _ => {
            None
        }
    };
    match ret {
        Some(msg) => {
            println!("{}", msg);
        },
        _ => {
            println!("No message returned.");
        }
    }
}

fn the_rest(args: std::str::SplitWhitespace)  -> String {
    let mut out = String::new();
    for word in args {
        out.push_str(word);
        out.push(' ');
    }
    out.pop();
    out
}