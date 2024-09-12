use std::{fs::OpenOptions, io::{self, Write}, thread::sleep};
use std::fs::read_to_string;
use std::time::Duration;
use chrono::Local;

fn main() -> io::Result<()> {
    let path = "atlasshit.txt";

    loop {
        let input = (|| -> String {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            input.trim().to_string()
        })();

        OpenOptions::new().append(true).create(true).open(path)?.write_all(format!("'{}' - Atlas, {}\n", input, Local::now().to_rfc3339()).as_bytes())?;

        let num_quotes: usize = read_to_string(path)?.lines().count();
        println!("Wrote the quote to {}, currently at {} dumbass quotes and counting!!", path, num_quotes);
        sleep(Duration::from_secs(3));
    }
}