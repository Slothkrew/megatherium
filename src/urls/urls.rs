extern crate rusqlite;
extern crate chrono;

use self::chrono::prelude::*;


/*
CREATE TABLE urls (timestamp INTEGER PRIMARY KEY, url TEXT, author TEXT, summary TEXT);
1491376530|https://odin.handmade.network/|sjums|Odin programming language. New, hip, must try!
*/
pub struct Url {
    pub timestamp: i64,
    pub url: String,
    pub author: String,
    pub summary: String
}

impl Url {
    fn new(timestamp: i64, url: String, author: String, summary: String) -> Url {
        Url{ timestamp: timestamp, url: url, author: author, summary: summary }
    }

    pub fn to_string(&self) -> String {
        // (Tue Aug 18 13:32:49 2015)
        let time_format = "%a %b %d %H:%M:%S %Y".to_string();
        let date_time = DateTime::<UTC>::from_utc(NaiveDateTime::from_timestamp(self.timestamp, 0), UTC);
        let time = date_time.format(&time_format);

        format!("{} -- \"{}\" -- {} ({})", self.url, self.summary, self.author, time)
    }

}

fn connection() -> rusqlite::Connection {
    let db_path = "urls.db";
    let con = rusqlite::Connection::open(db_path).expect("Could not open urls.db");
    con
}

fn epoch() -> i64 {
    UTC::now().timestamp()
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

pub fn add(url: &String, summary: &String, author: &String) {
    let connection = connection();
    let res = connection.execute("INSERT INTO urls (timestamp, url, author, summary) VALUES(?, ?, ?, ?);", &[&epoch(), url, author, summary]);
    match res {
        Ok(row_c) => {
            println!("URL added! {} row(s) changed.", row_c);
        },
        Err(e) => {
            println!("Couldn't add URL to SQLite DB.");
            println!("{:?}", e);
        }
    }
}

pub fn get_last() -> Option<Url> {
    let connection = connection();
    let p_statement = connection.prepare("SELECT * FROM urls ORDER BY timestamp DESC LIMIT 1;");

    let url = match p_statement {
        Ok(mut statement) => {
            match statement.query(&[]) {
                Ok(mut rows) => {
                    let row = rows.next();
                    match row {
                        Some(row) => {
                            match row {
                                Ok(row) => {
                                    Some(Url::new(row.get(0), row.get(1), row.get(2), row.get(3)))
                                },
                                Err(_) => None
                            }
                        },
                        None => None
                    }
                },
                Err(_) => None,
            }
        },
        Err(_) => None
    };
    url
}

pub fn find(query: String) -> Vec<Url> {
    vec! {
        Url::new(epoch(), "https://example.com".to_string(), "yoyo".to_string(), format!("cool {}. Very example", query)),
        Url::new(epoch() - 36000, "https://website.com".to_string(), "manman".to_string(), format!("website about {}.", query))
    }
}

