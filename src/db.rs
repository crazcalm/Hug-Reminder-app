use std::path::Path;
use std::rc::Rc;

use rusqlite::{params, Connection};

pub type DBLocal = Rc<Connection>;

pub fn get_connection() -> Connection {
    let path = Path::new("hugs.db");
    Connection::open(path).expect("Unable to find DB")
}

pub fn get_db_rc() -> DBLocal {
    Rc::new(get_connection())
}

pub fn initialize_db(conn: DBLocal) -> Result<(), String> {
    let result = conn.execute_batch(
        "
            DROP TABLE IF EXISTS record;
            CREATE TABLE record (
	            id	INTEGER NOT NULL UNIQUE,
	            datetime	TEXT NOT NULL,
	            PRIMARY KEY(id)
            );

            DROP TABLE IF EXISTS quota;
            CREATE TABLE quota (
	            id	INTEGER NOT NULL UNIQUE,
	            max	INTEGER NOT NULL,
	            hit_percentage	INTEGER NOT NULL,
	            PRIMARY KEY(id)
            );",
    );

    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Unable to create db: {:?}", err)),
    }
}

pub fn get_hug_count_by_date(conn: DBLocal, date: String) -> i32 {
    conn.query_row(
        "SELECT COUNT(datetime) from record where DATE(datetime) == DATE(?);",
        params![date],
        |row| Ok(row.get(0).expect("unable to get hug count")),
    )
    .unwrap()
}

pub fn get_daily_hug_number(conn: DBLocal) -> usize {
    conn.query_row("SELECT max from quota", [], |row| {
        Ok(row.get(0).expect("unable to get daily hug quata"))
    })
    .unwrap()
}

pub fn set_daily_hug_number(conn: DBLocal, num: usize) -> Result<usize, String> {
    let result = conn
        .execute("UPDATE quota SET max = ? where id = 1", params![num])
        .expect("unable to update daily hug number");

    Ok(result)
}

pub fn set_hit_percentage(conn: DBLocal, num: usize) -> Result<usize, String> {
    let result = conn
        .execute(
            "UPDATE quota SET hit_percentage = ? where id = 1",
            params![num],
        )
        .expect("unable to update daily hug number");

    Ok(result)
}
