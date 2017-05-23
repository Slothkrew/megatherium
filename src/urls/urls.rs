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
        "```\
        **************************************************\n\
        | url: url storage utility                       |\n\
        |------------------------------------------------|\n\
        | !url                     | get a random link   |\n\
        | !url add <url> [summary] | add a new link      |\n\
        | !url latest|newest       | get the latest link |\n\
        | !url find <string>       | list urls by search |\n\
        | !url count [nick]        | you guessed it!     |\n\
        | !url stats               | print pretty stats  |\n\
        **************************************************```".to_string()
}

pub fn random() -> Option<Url> {
    match query_many("SELECT * FROM urls ORDER BY random() LIMIT 1;".to_string(), &[]) {
        Some(mut urls) => urls.pop(),
        None => None
    }
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
    let url = match query_many("SELECT * FROM urls ORDER BY timestamp DESC LIMIT 1;".to_string(), &[]) {
        Some(mut urls) => {
            urls.pop()
        },
        None => None
    };

    url
}

fn query_many(query: String, params: &[&self::rusqlite::types::ToSql]) -> Option<Vec<Url>> {
    let mut results = Vec::<Url>::new();

    let connection = connection();
    match connection.prepare(&query) {
        Ok(mut statement) => {
            match statement.query(&params) {
                Ok(mut rows) => {
                    while let Some(res_row) = rows.next() {
                        match res_row {
                            Ok(row) => {
                                results.push(
                                    Url::new(
                                        row.get(0),
                                        row.get(1),
                                        row.get(2),
                                        row.get_checked(3).unwrap_or_default()
                                    )
                                );
                            },
                            Err(_) => ()
                        };
                    };
                },
                Err(_) => ()
            }
        },
        Err(_) => ()
    };

    if &results.len() > &0 {
        Some(results)
    } else {
        None
    }
}

pub fn find(query: String) -> Option<Vec<Url>> {
    let mut sql_query = query.replace(" ", "%");
    sql_query.insert(0, '%');
    sql_query.push('%');

    query_many("SELECT * FROM urls WHERE [summary] LIKE ? OR [url] LIKE ? OR [author] LIKE ? ORDER BY timestamp DESC;".to_string(),
                    &[&sql_query, &sql_query, &sql_query])
}

pub fn delete(url: &String, author: &String) {
    let connection = connection();

    println!("DELETE FROM urls WHERE url = {} AND author = {};", url, author);
    let res = connection.execute("DELETE FROM urls WHERE url = ? AND author = ?;", &[url, author]);
    match res {
        Ok(res) => {
            println!("Removed {} rows from DB.", res);
        },
        Err(e) => {
            println!("Error remove url from DB; {}", e);
        }
    }
}

pub fn count(author: Option<&str>) -> usize {
    match author {
        Some(author) => {
            match query_many("SELECT * FROM urls WHERE [author] = ?;".to_string(), &[&author]) {
                Some(urls) => urls.len(),
                None => 0
            }
        },
        None => {
            match query_many("SELECT * FROM urls;".to_string(), &[]) {
                    Some(urls) => urls.len(),
                    None => 0
            }
        }
    }
}
