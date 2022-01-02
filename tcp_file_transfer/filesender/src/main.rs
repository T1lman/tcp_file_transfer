use std::io::prelude::*;

fn main() {
    let main_args: (String, String) = retrieve_main_argument(retrieve_arguments());
    let filename: String = main_args.0;
    let ip: String = main_args.1;
    send_file(&filename, ip)
}

fn retrieve_arguments() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        0 | 1 | 2 => {
            println!("Not enough arguments! File and Adress need to be specified!");
            std::process::exit(0);
        }
        _ => return args,
    }
}

fn retrieve_main_argument(argvec: Vec<String>) -> (String, String) {
    return (argvec[1].clone(), argvec[2].clone());
}

fn retrieve_file_content(filename: &String) -> Vec<u8> {
    let filehandle = std::fs::read(filename);
    match filehandle {
        Err(e) => {
            println!("File was not found!\n{}", e);
            std::process::exit(0);
        }
        Ok(filecontent) => {
            return filecontent;
        }
    }
}

fn send_file(filename: &String, ip: String) -> () {
    let content = retrieve_file_content(&filename);
    let msglen = (content.len() as u64).to_be_bytes();
    let filenamelen = (filename.len() as u64).to_be_bytes();
    match std::net::TcpStream::connect(ip) {
        Ok(mut stream) => {
            println!("Connection established!");

            match stream.write_all(&msglen) {
                Ok(_) => {
                    stream.write_all(&content).unwrap();
                    stream.write_all(&filenamelen).unwrap();
                    println!("Sending {}", filename);
                    stream.write_all(&filename.as_bytes()).unwrap();
                    let mut statusbuffer = [0 as u8; 1];
                    stream.read_exact(&mut statusbuffer).unwrap();
                    match u8::from_be_bytes(statusbuffer) {
                        0 => println!("Sending sucessful!"),
                        _ => println!("Sending was not sucessful! Try later again!"),
                    }
                }
                Err(e) => {
                    println!("Could not send file!\n{}", e)
                }
            }
        }
        Err(e) => {
            println!("Connecting was not possible!\n{}", e);
            std::process::exit(0)
        }
    }
}
