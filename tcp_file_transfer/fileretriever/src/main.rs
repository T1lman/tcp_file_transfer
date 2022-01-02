use std::io::prelude::*;

fn main() {
    let stream = setup_server();
    handle_connections(stream);
}

fn retrieve_arguments() -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        0 | 1 => None,
        _ => return Some(args[1].clone()),
    }
}

fn setup_server() -> std::net::TcpListener {
    let args = retrieve_arguments();
    match args {
        Some(port) => {
            let ip: String = format!("0.0.0.0:{}", port);
            println!("starting server on {}", ip);
            let listener = std::net::TcpListener::bind(ip);
            match listener {
                Ok(stream) => {
                    println!("Server started sucessful!");
                    return stream;
                }
                Err(e) => {
                    println!("Could not start Server!\n{}", e);
                    std::process::exit(0);
                }
            }
        }
        None => {
            println!("No Port specified starting server on 0.0.0.0:3333");
            let listener = std::net::TcpListener::bind("0.0.0.0:3333");

            match listener {
                Ok(stream) => {
                    println!("Server started sucessful!");
                    return stream;
                }
                Err(e) => {
                    println!("Could not start Server!\n{}", e);
                    std::process::exit(0);
                }
            }
        }
    }
}

fn handle_connections(listener: std::net::TcpListener) {
    for connections in listener.incoming() {
        match connections {
            Ok(mut stream) => {
                println!("New Connection from {}", stream.peer_addr().unwrap());
                std::thread::spawn(move || {
                    handle_client(&mut stream);
                });
            }
            Err(e) => {
                println!("Error occured while connecting!\n{}", e)
            }
        }
    }

    drop(listener);
}

fn handle_client(stream: &mut std::net::TcpStream) {
    let mut lenbuffer: [u8; 8] = [0 as u8; 8];
    stream.read_exact(&mut lenbuffer).unwrap();
    let len = u64::from_be_bytes(lenbuffer) as usize;
    let mut filecontent = vec![0 as u8; len];
    stream.read(&mut filecontent).unwrap();
    let mut lenbuffername: [u8; 8] = [0 as u8; 8];
    stream.read(&mut lenbuffername).unwrap();
    let lenname = u64::from_be_bytes(lenbuffername) as usize;
    let mut filenamebytes = vec![0 as u8; lenname];
    stream.read_exact(&mut filenamebytes).unwrap();
    let filename = String::from_utf8(filenamebytes).unwrap();
    let mut file = std::fs::File::create(filename).unwrap();
    file.write_all(&filecontent).unwrap();
    stream.write_all(&((0 as u8).to_be_bytes())).unwrap();
}
