extern crate rusqlite;

use rusqlite::Connection;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

mod recorder;

fn main() {
    let db = Connection::open("./data.db").unwrap();
    db.execute("CREATE TABLE data (
            ax      INTEGER NOT NULL,
            ay      INTEGER NOT NULL,
            az      INTEGER NOT NULL)", &[]).unwrap();
    let sleep_time = Duration::from_millis(100);
    loop {
        let cur_time = SystemTime::now();
        let acc = recorder::acceleration();
        println!("ax: {}, ay: {}, az: {}", acc.ax, acc.ay, acc.az);
        db.execute("INSERT INTO data (ax, ay, az)
            VALUES (?1, ?2, ?3)", &[&acc.ax, &acc.ay, &acc.az]).unwrap();
        sleep(sleep_time - cur_time.elapsed().unwrap());
    }
}
