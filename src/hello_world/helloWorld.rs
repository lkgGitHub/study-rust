use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

mod hello_world {
    pub fn helloWorld() {
        println!("Hello, world!");
        let stdout = stdout();
        let message = String::from("Hello fellow Rustaceans!");
        let width = message.chars().count();

        let mut writer = BufWriter::new(stdout.lock());
        say(message.as_bytes(), width, &mut writer).unwrap();
    }

}



