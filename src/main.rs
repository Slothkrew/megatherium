mod urls;
mod wheel;

fn main() {
    let command = "wheel".to_string();
    exec_command(command);
}

fn exec_command(command: String) {
    let mut input = command.split_whitespace();
    let command = input.nth(0);
    let mut args = input;

    let ret = match command {
        Some("wheel") => {
            let arg = args.nth(0);
            if &arg == &Some("about") {
                wheel::about()
            }
            else if &arg == &Some("help") {
                wheel::help()
            }
            else {
                wheel::spin()
            }
        },
        Some("url") => {
            //Todo: add, clear?, latest/newest, list, find, count, stats
            "".to_string()
        },
        _ => "".to_string()
    };
    println!("{}", ret);
}
