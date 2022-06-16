use std::path::Path;
use std::rc::Rc;

use chrono::prelude::*;
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

pub fn get_hug_count_by_date(conn: DBLocal, date: String) -> usize {
    conn.query_row(
        "SELECT COUNT(datetime) from record where DATE(datetime) == DATE(?);",
        params![date],
        |row| Ok(row.get(0).expect("unable to get hug count")),
    )
    .unwrap()
}

pub fn get_daily_hug_number(conn: DBLocal) -> usize {
    conn.query_row("SELECT max from quota where id = 1", [], |row| {
        Ok(row.get(0).expect("unable to get daily hug quata"))
    })
    .unwrap()
}

pub fn update_daily_hug_number(conn: DBLocal, num: usize) -> Result<usize, String> {
    let result = conn
        .execute("UPDATE quota SET max = ? where id = 1", params![num])
        .expect("unable to update daily hug number");

    Ok(result)
}

pub fn insert_record(conn: DBLocal, datetime: String) -> Result<usize, String> {
    let result = conn
        .execute(
            "INSERT INTO record (datetime) VALUES (?)",
            params![datetime],
        )
        .unwrap();

    Ok(result)
}

pub fn insert_record_for_today(conn: DBLocal) -> Result<usize, String> {
    let time = Local::now();
    insert_record(conn, time.to_string())
}

pub fn insert_quota(conn: DBLocal, max: usize, hit_percentage: usize) -> Result<usize, String> {
    let result = conn
        .execute(
            "INSERT INTO quota (max, hit_percentage) VALUES (?, ?)",
            params![max, hit_percentage],
        )
        .unwrap();

    Ok(result)
}

pub fn update_hit_percentage(conn: DBLocal, num: usize) -> Result<usize, String> {
    let result = conn
        .execute(
            "UPDATE quota SET hit_percentage = ? where id = 1",
            params![num],
        )
        .expect("unable to update daily hug number");

    Ok(result)
}

pub fn get_hit_percentage(conn: DBLocal) -> Result<usize, String> {
    let result = conn
        .query_row("SELECT hit_percentage from quota", [], |row| {
            Ok(row.get(0).expect("unable to get hit percentage"))
        })
        .unwrap();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use rusqlite::Connection;

    use super::*;

    fn in_memory_db() -> DBLocal {
        let conn = Connection::open_in_memory().unwrap();

        let local_conn = Rc::new(conn);

        initialize_db(local_conn.clone()).unwrap();

        local_conn
    }

    #[test]
    fn test_quota_table() {
        let conn = in_memory_db();
        insert_quota(conn.clone(), 5, 25).unwrap();

        let hit_percentage = get_hit_percentage(conn.clone()).unwrap();
        assert_eq!(hit_percentage, 25);

        let hug_number = get_daily_hug_number(conn.clone());
        assert_eq!(hug_number, 5);

        let num: usize = 10;
        let result = update_hit_percentage(conn.clone(), num);
        let expected: usize = 1;
        assert_eq!(result.unwrap(), expected);

        let hit_percentage = get_hit_percentage(conn.clone()).unwrap();
        assert_eq!(hit_percentage, num);

        let num: usize = 8;
        let result = update_daily_hug_number(conn.clone(), num).unwrap();
        let expected = 1;
        assert_eq!(result, expected);

        let hug_number = get_daily_hug_number(conn.clone());
        assert_eq!(hug_number, num);
    }

    #[test]
    fn test_record_table() {
        let conn = in_memory_db();
        insert_record(conn.clone(), "2022-06-14T19:06:19.790140".to_string()).unwrap();
        insert_record(conn.clone(), "2022-06-14T19:05:19.790140".to_string()).unwrap();
        insert_record(conn.clone(), "2022-06-14T19:04:19.790140".to_string()).unwrap();
        insert_record(conn.clone(), "2022-06-13T19:06:19.790140".to_string()).unwrap();
        insert_record(conn.clone(), "2022-06-13T19:05:19.790140".to_string()).unwrap();

        let expected = 3;
        let result = get_hug_count_by_date(conn.clone(), "2022-06-14T19:06:19.790140".to_string());
        assert_eq!(result, expected);

        let expected = 2;
        let result = get_hug_count_by_date(conn.clone(), "2022-06-13T19:06:19.790140".to_string());
        assert_eq!(result, expected);

        let expected = 0;
        let result = get_hug_count_by_date(conn.clone(), "2022-06-12T19:06:19.790140".to_string());
        assert_eq!(result, expected);

        let today = Local::now().to_string();
        insert_record_for_today(conn.clone()).unwrap();
        let expected = 1;
        let result = get_hug_count_by_date(conn.clone(), today);
        assert_eq!(result, expected);
    }
}
