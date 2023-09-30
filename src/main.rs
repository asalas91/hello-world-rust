use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();

    let age: u8 = 32;
    let smiley = "😍";
    println!("Hello {}, age {} {}!", "Rustaceans", age, smiley);

    struct Persona { name: String, age: u8, twitter: String, remote: bool }
    struct Hobbies(String, String, String);
    
    let persona = Persona {
        name: String::from("C0d3RX"),
        age: 32,
        twitter: String::from("@coder"),
        remote: true
    };
}