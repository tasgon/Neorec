use std::io::Read;
use std::fs::File;

pub struct Acceleration {
    pub ax: i32,
    pub ay: i32,
    pub az: i32
}

pub fn read_acceleration(file: &mut File) -> Acceleration {
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let input: Vec<i32> = data.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    return Acceleration { ax: input[0], ay: input[1], az: input [2] };
}