use std::fs::File;
use std::io::{Read, Result, Write};
use std::net::TcpStream;

fn main() -> Result<()> {
    let mut stream = TcpStream::connect(("localhost", 1337))?;
    stream.write(b"body_of_water_planet_horizon_1920x1200.jpg")?;

    let mut buf = vec![0 as u8; 2560000];
    match stream.read(&mut buf) {
        Ok(size) => match File::create("body_of_water_planet_horizon_1920x1200.jpg") {
            Ok(mut f) => match f.write_all(&buf[..size]) {
                Ok(_o) => {
                    println!("Done!");
                }
                Err(e) => {
                    eprintln!("Can't write to file: {}", e);
                }
            },
            Err(e) => {
                eprintln!("Can't create file: {}", e);
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
