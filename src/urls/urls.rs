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
        | !url latest|newest       | get the latest link |\n\
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
    let res = connection.execute("DELETE FROM urls WHERE url = ? AND author = ?;", 
        &[url, author]);
    match res {
        Ok(res) => {
            println!("Removed {} rows from DB.", res);
        },
        Err(e) => {
            println!("Error remove url from DB; {}", e);
        }
    }
}

pub fn count(author: &Option<String>) -> usize {
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
        },
    }
}

