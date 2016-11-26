use std::io;
use std::io::{BufReader, BufRead, BufWriter, Write};
use std::net::TcpStream;
use std::thread;

fn tx_th(socket: TcpStream) {
    let rd = BufReader::new(io::stdin());
    let mut wr = BufWriter::new(socket);
    for line in rd.lines() {
        match line {
            Ok(line) => {
                match write!(&mut wr, "{}\n", line) {
                    Ok(_) => {},
                    Err(e) => {println!("Transmission error: {}", e);}
                }
                wr.flush().unwrap();
            },
            Err(e) => {
                println!("Reading error: {}", e);
            }
        }
    }
    std::process::exit(1);
}

fn rx_th(socket: TcpStream) {
    let rd = BufReader::new(socket);
    let mut wr = BufWriter::new(io::stdout());
    for line in rd.lines() {
        match line {
            Ok(line) => {
                match write!(&mut wr, "{}\n", line) {
                    Ok(_) => {},
                    Err(e) => {println!("Writing error: {}", e);}
                }
                wr.flush().unwrap();
            },
            Err(e) => {
                println!("Receiving error: {}", e);
            }
        }
    }
    std::process::exit(0);
}

fn main() {
    let addr = std::env::args().nth(1).expect("Usage: chat_client address\n");
    let sock = TcpStream::connect((addr.as_ref(), 65521)).unwrap();
    let sock2 = sock.try_clone().unwrap();
    thread::spawn(move || rx_th(sock));
    tx_th(sock2);
}
