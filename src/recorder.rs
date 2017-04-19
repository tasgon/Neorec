use std::io::Read;
use std::fs::File;

pub struct Acceleration {
    pub ax: i32,
    pub ay: i32,
    pub az: i32
}

pub fn read_acceleration(mut file: File) -> Acceleration {

    let mut data = String::new();
    let result = file.read_to_string(&mut data);
    if result.is_ok() {
        result.unwrap();
        println!("{}", data);
        let split: Vec<&str> = data.split(",").collect();
        println!("{}", split[0]);
        //let input: Vec<i32> = split.map(|x| x.parse::<i32>().unwrap_or(0)).collect();
        //return Acceleration { ax: input[0], ay: input[1], az: input[2] };
        return Acceleration { ax: 0, ay: 0, az: 0};
    } else {
        println!("Error reading input.");
        return Acceleration { ax: 0, ay: 0, az: 0};
    }
}