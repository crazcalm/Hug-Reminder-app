use chrono::prelude::*;
use hug_reminder::{cli, db, email};
use rand::prelude::*;

fn should_send_email(
    hug_goal: usize,
    current_hug_count: usize,
    hit_percentage: usize,
    rand_number: usize,
) -> bool {
    if current_hug_count >= hug_goal {
        return false;
    }

    if hit_percentage >= rand_number {
        return true;
    }
    false
}

fn main() -> Result<(), String> {
    let cli_app = cli::get_app();

    let matches = cli_app.get_matches();

    let env_file_path = matches.get_one::<String>("env-file").unwrap();
    let _ = dotenv::from_filename(env_file_path).ok();

    let mut rng = thread_rng();
    let rand_number: usize = rng.gen_range(0..=100);

    let conn = db::get_db_rc();
    let hug_goal = db::get_daily_hug_number(conn.clone());
    let today = Local::now().to_string();
    let current_hug_count = db::get_hug_count_by_date(conn.clone(), today);
    let hit_percentage = db::get_hit_percentage(conn.clone()).unwrap();

    if should_send_email(hug_goal, current_hug_count, hit_percentage, rand_number) {
        if env_file_path.starts_with("dev") {
            println!("We do not send emails in dev");
        } else {
            let (email, password, to_email) = email::get_info()?;
            let today = Local::now().to_string();
            let subject = "Hug Reminder".to_string();
            let body = format!(
                "{} of {} hugs for {}",
                current_hug_count + 1,
                hug_goal,
                today
            );
            let _ = email::send(email, password, to_email, subject, body)?;
            db::insert_record(conn, today).unwrap();
        }
    } else {
        println!("We are not sending an email");
    }
    Ok(())
}
