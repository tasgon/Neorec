extern crate rusqlite;

use rusqlite::Connection;
use std::fs::File;
use std::thread::sleep;
use std::time::Duration;

mod recorder;

static INPUT: &'static str = "/sensors/acceleromoter/data";
static OUTPUT: &'static str = "data.db";

fn main() {
    let db = Connection::open(OUTPUT).unwrap();
    db.execute("CREATE TABLE data (
            ax      INTEGER NOT NULL,
            ay      INTEGER NOT NULL,
            az      INTEGER NOT NULL,
            time    TEXT NOT NULL", &[]).unwrap();
    let sleep_time = Duration::from_millis(100);
    loop {
        let mut file = File::create(INPUT).unwrap();
        let acc = recorder::read_acceleration(&mut file);
        db.execute("INSERT INTO data (ax, ay, az, time)
            VALUES (?1, ?2, ?3, strftime('%f', 'now')", &[&acc.ax, &acc.ay, &acc.az]).unwrap();
        sleep(sleep_time);
    }
}
