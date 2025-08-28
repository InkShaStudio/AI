mod body;
mod input;
mod thought;

use body::Body;

fn main() {
    let mut body = Body::new(100);
    while true {
        let i = input::line::read_input(Some("hello?"));
        body.say(i);
    }
}
