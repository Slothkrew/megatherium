extern crate rusqlite;
extern crate time;
extern crate chrono;


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

    fn epoch_to_tm(epoch: i64) -> time::Tm {
        time::Tm{ };
    }

    pub fn to_string(&self) -> String {
        /// TODO: Convert epoch to datetime.. Maybe the time-crate can do some magic?
        // (Tue Aug 18 13:32:49 2015)
        let time_format = "".to_string();
        let time = self::time::strftime(&time_format, &self.epoch_to_tm(self.timestamp));

        format!("{} -- \"{}\" -- {} ({})", self.url, self.summary, self.author, self.timestamp)
    }
}

fn connection() -> rusqlite::Connection {
    let db_path = "urls.db";
    let con = rusqlite::Connection::open(db_path);
    let connection = match con {
        Ok(con) => {
            con
        },
        Err(e) => {
            println!("Err(e): Could not open urls.db");
            panic!(e);
        }
    };
    connection
}

fn epoch() -> i64 {
    time::get_time().sec
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

pub fn get_last() -> Url {
    Url::new(-1, "https://example.com".to_string(), "yoyo".to_string(), "cool site. Very example".to_string())
}

pub fn find(query: String) -> Vec<Url> {
    vec! {
        Url::new(-1, "https://example.com".to_string(), "yoyo".to_string(), format!("cool {}. Very example", query)),
        Url::new(-2, "https://website.com".to_string(), "manman".to_string(), format!("website about {}.", query))
    }
}

