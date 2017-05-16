extern crate sqlite;
extern crate time;

/*
CREATE TABLE urls (timestamp INTEGER PRIMARY KEY, url TEXT, author TEXT, summary TEXT);
1491376530|https://odin.handmade.network/|sjums|Odin programming language. New, hip, must try!
*/
fn connection() -> sqlite::Connection {
    let con = sqlite::open("urls.db");
    let connection = match con {
        Ok(con) => {
            println!("Ok(con)");
            con
        },
        Err(e) => {
            panic!(e);
        }
    };
    connection
}

fn epoch() -> i64 {
    time::get_time().sec * 1000
}

pub fn help() -> String {
        "\
        **************************************************\n\
        | url: url storage utility                       |\n\
        |------------------------------------------------|\n\
        | !url                     | get a random link   |\n\
        | !url add <url> [summary] | add a new link      |\n\
        | !url clear               | clear your urls     |\n\
        | !url latest|newest       | get the latest link |\n\
        | !url list [nick]         | list urls           |\n\
        | !url find <string>       | list urls by search |\n\
        | !url count [nick]        | you guessed it!     |\n\
        | !url stats               | print pretty stats  |\n\
        **************************************************".to_string()
}

pub fn add(url: &str, desc: &str, author: &str) {
    //TODO: Add url to sqlite DB
    let connection = connection();
    let mut statement = connection.prepare("INSERT INTO urls VALUES(?, ?, ?, ?);").unwrap();
    statement.bind(1, epoch());
    statement.bind(2, url);
    statement.bind(3, author);
    statement.bind(4, desc);
    
    //connection.execute(statement);   

}