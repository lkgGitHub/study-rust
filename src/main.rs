use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};
use hell_oworld::helloWorld;
#[path="hello_world/helloWorld.rs"]
fn main() {
    println!("Hello, world!");
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

