//extern crate rusqlite;

//use rusqlite::Connection;
use std::fs::File;
use std::thread::sleep;
use std::time::Duration;

mod recorder;

static INPUT: &'static str = "/sensors/accelerometer/data";
//static OUTPUT: &'static str = "data.db";

fn main() {
    /*let db = Connection::open(OUTPUT).unwrap();
    let create_result = db.execute("CREATE TABLE data (
            ax      INTEGER NOT NULL,
            ay      INTEGER NOT NULL,
            az      INTEGER NOT NULL,
            time    TEXT NOT NULL", &[]);*/
    let sleep_time = Duration::from_millis(100);
    loop {
        let file = File::open(INPUT).unwrap();
        let acc = recorder::read_acceleration(file);
        println!("ax: {}, ay: {}, az: {}", acc.ax, acc.ay, acc.az);
        //db.execute("INSERT INTO data (ax, ay, az, time)
        //    VALUES (?1, ?2, ?3, strftime('%f', 'now')", &[&acc.ax, &acc.ay, &acc.az]).unwrap();
        sleep(sleep_time);
    }
}
