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

#[allow(dead_code)]
fn get_first() -> Option<Url> {
    let url = match query_many("SELECT * FROM urls ORDER BY timestamp ASC LIMIT 1;".to_string(), &[]) {
        Some(mut urls) => {
            urls.pop()
        },
        None => None
    };

    url
}



/*

<slothbot_>         Let's see who's on top in here!
<slothbot_> dot|not |#######################                 | 59.76%
<slothbot_>   sjums |#########                               | 23.37%
<slothbot_> sleeper |###                                     | 9.17%
<slothbot_>    jsrn |#                                       | 4.44%
<slothbot_>   maruu |                                        | 2.37%
<slothbot_>   rfmon |                                        | 0.59%
<slothbot_>   jsrn_ |                                        | 0.3%
<slothbot_>         +----------------------------------------+
<slothbot_>         Links added per day: 0.42

*/
pub fn stats() -> String {
    let mut results = Vec::<(String, i32)>::new();

    let connection = connection();
    let p_statement = connection.prepare("SELECT author, COUNT(author) FROM urls GROUP BY author ORDER BY COUNT(author) DESC;");

    match p_statement {
        Ok(mut statement) => {
            match statement.query(&[]) {
                Ok(mut rows) => {
                    while let Some(res_row) = rows.next() {
                        match res_row {
                            Ok(row) => {
                                let author: String = row.get(0);
                                let urls: i32 = row.get(1);
                                results.push((author, urls));
                            },
                            Err(_) => ()
                        }
                    }
                },
                Err(_) => ()
            }
        },
        Err(_) => ()
    }

    let url_count = count(None) as f32;
    let mut max_user_len = 0;
    for res in &results {
        let ref uname = res.0;
        if uname.len() > max_user_len {
            max_user_len = uname.len();
        }
    }

    let bar_width = 50;
    let mut result = String::new();
    result.push_str(&"```".to_string());

    for res in results {
        let author = res.0;
        let urls = res.1 as f32;
        result.push_str(&format!("{author:>width$} |", author=author, width=max_user_len));
        let squares = ((urls / url_count) * bar_width as f32) as i32;
        for _ in 0..squares {
            result.push('#');
        }
        for _ in 0..bar_width-squares {
            result.push(' ');
        }
        result.push_str(&format!(" | {:>5.2}%\n", (urls/url_count)*100.0));
    }

    result.push_str(&format!("{plus:>width$}", plus="+", width=max_user_len+2));
    for _ in 0..bar_width+1 {
        result.push('-');
    }
    result.push('+');

    result.push_str(&"```".to_string());
    result
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
