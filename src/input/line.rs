use std::io::{self, Write};

pub fn read_input<S>(prompt: Option<S>) -> String
where
    S: Into<String>,
{
    let mut input = String::new();

    if let Some(p) = prompt {
        print!("{}", p.into());
        io::stdout().flush().unwrap();
    }

    io::stdin().read_line(&mut input).expect("read failure");

    return String::from(input.trim());
}
