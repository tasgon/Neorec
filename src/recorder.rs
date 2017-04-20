use std::io::Read;
use std::fs::File;

const ACCELERATION_SENSOR: &'static str = "/sensors/accelerometer/data";

pub struct Acceleration {
    pub ax: i32,
    pub ay: i32,
    pub az: i32
}

pub fn acceleration() -> Acceleration {
    let mut file = File::open(ACCELERATION_SENSOR).unwrap();
    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Ok(_) => {
            data = data.replace("\n", "");
            let split = data.split(",");
            let input: Vec<i32> = split.map(|x| x.parse::<i32>().unwrap()).collect();
            Acceleration { ax: input[0], ay: input[1], az: input[2] }
        },
        Err(_) => {
            println!("Error reading input.");
            Acceleration { ax: 0, ay: 0, az: 0 }
        }
    }
}