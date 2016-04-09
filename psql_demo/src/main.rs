extern crate postgres;

extern crate dotenv;
use dotenv::dotenv;
use std::env;

use postgres::{Connection, SslMode};

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("db url: {}", database_url);

    let conn = Connection::connect(database_url.as_str(), SslMode::None).unwrap();

    conn.execute("select now()", &[]).unwrap();

}

