use std::io;

pub fn string_to_i32(string_to_be_converted : String) -> i32 {
    let integer: i32 = string_to_be_converted.trim().parse().expect("Converting error.");
    integer
}

pub fn input() -> String {
    let mut to_return = String::new();

    io::stdin()
        .read_line(&mut to_return)
        .expect("Input error.");

    to_return
}
