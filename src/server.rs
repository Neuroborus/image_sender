use std::io::{Read, Result, Write};
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::{fs, str, sync::Arc, thread};

///Processing connection
fn incoming_connection(mut stream: TcpStream, img: Arc<Vec<u8>>) {
    let mut buf = vec![0 as u8; 256]; //Buffer for message from client
    match stream.read(&mut buf) {
        Ok(size) => match str::from_utf8(&buf[..size]) {
            Ok(incoming_str) => {
                if incoming_str == "body_of_water_planet_horizon_1920x1200.jpg" {
                    //If msg from client is equal to str
                    match stream.write_all(img.as_slice()) {
                        Ok(_o) => match stream.peer_addr() {
                            Ok(peer) => {
                                println!("Image sent to {}", peer);
                            }
                            Err(e) => {
                                eprintln!("Error: {}", e);
                            }
                        },
                        Err(e) => {
                            eprintln!("Can't send image: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

fn main() -> Result<()> {
    println!("Server running!");

    let listener = TcpListener::bind((Ipv4Addr::new(0, 0, 0, 0), 1337))?;
    let img = fs::read("body_of_water_planet_horizon_1920x1200.jpg")?;
    let img = Arc::new(img); //Arc for sharing image between threads

    for stream in listener.incoming() {
        //For every connection
        match stream {
            Ok(stream) => {
                let imj = img.clone(); //Creating another copy of shared ptr of image
                thread::spawn(move || {
                    //Processing connection in another thread
                    incoming_connection(stream, imj);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    Ok(())
}
