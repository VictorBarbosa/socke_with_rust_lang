use std::{
    io::{self, Read},
    net::TcpStream,
    str::from_utf8,
};

fn main() {
    println!("***************************************************************************");
    println!("*********This project is how to receive a message using Rust Lang *********");
    println!("***************************************************************************\n\n");

    std::thread::spawn(|| init_socket_server());

    loop {
        println!("Press enter to close!!");

        let mut _quit = String::new();
        io::stdin()
            .read_line(&mut _quit)
            .expect("Something wrong with variable _quit");

        if _quit.starts_with("\n") {
            println!("Close");
            break;
        }
    }
}

fn init_socket_server() {
    match socket_connection() {
        Ok(mut _sock) => {
            println!("Connected on {:?} \n\n", _sock);
            match read_message_size(_sock) {
                Some(size) => {
                    println!("Size - {}", size);
                    init_socket_server();
                }
                _ => {
                    println!("Erro")
                }
            }
        }
        Err(e) => {
            println!("**Error : {}", e);
        }
    }
}

fn socket_connection() -> Result<TcpStream, String> {
    let sock: Result<std::net::TcpListener, std::io::Error> =
        std::net::TcpListener::bind("0.0.0.0:9988");
    match sock {
        Ok(socket) => {
        
            for stream in socket.incoming() {
                match stream {
                    Ok(conn) => {
                       
                        println!("Socket connected !! \n {:?}", conn);
                        return Ok(conn);
                    }
                    Err(err) => {
                        let err = format!("\n\n\nSome thing wrong happend -> {:?} !!", err);
                        return Err(err);
                        // return None;
                    }
                }
            }
            Err("Erro".to_string())
        }
        Err(err) => {
            let err = format!("\n\n\nSome thing wrong happend -> {:?} !!", err);
            return Err(err);
            // return None;
        }
    }
}

fn read_message_size(mut sock: TcpStream) -> Option<String> {
    let mut data = Vec::new();
    match sock.read_to_end(&mut data) {
        Ok(_) => {
            let text = from_utf8(&data).unwrap();
            let text: String = text.to_string();

            println!("Message from  {:?}: {}", sock.peer_addr(), text);

            return Some(text);
        }
        Err(e) => {
            println!("## Error : {:?} ", e);
        }
    }
    None
}
