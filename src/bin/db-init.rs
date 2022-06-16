use hug_reminder::db::*;

fn main() {
    let conn = get_db_rc();

    initialize_db(conn.clone()).expect("unable to initialize DB");
    insert_quota(conn, 5, 30).unwrap();
}
