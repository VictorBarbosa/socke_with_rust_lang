use std::io::{self, Write};

fn main() {
    try_connection();
}

fn try_connection() {
    let mut stream = std::net::TcpStream::connect("192.168.1.75:9988")
        .expect("Some thing wrong happend when we trying connecting");

    let mut msg = String::new();
    print!("Type here your message:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut msg).expect("");
    let msg2 = msg.as_bytes();

    match stream.write(msg2) {
        Ok(s) => {
            println!("Size - {}", s);
        }
        _ => {
            println!("test")
        }
    };
}
