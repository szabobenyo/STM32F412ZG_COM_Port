//use rayon::prelude::*;
use serialport::SerialPort;
use std::io;
use std::io::Read;
use std::io::Write;

fn define_port() -> Box<dyn SerialPort> {
    return serialport::new("COM8", 9600)
        .open()
        .expect("Failed to open serial port");
}

fn send_input_data(port: &mut Box<dyn SerialPort>) {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    port.write_all(input.as_bytes()).expect("Write failed!");
}

fn read_recieved_buffers(port: &mut Box<dyn SerialPort>) {
    let mut buffer: [u8; 1] = [0; 1];
    let mut string_buffer = String::new();

    loop {
        match port.read(&mut buffer) {
            Ok(bytes) => {
                if bytes == 1 {
                    let c = buffer[0] as char;
                    string_buffer.push(c);
                    if c == '\n' {
                        println!("{}", string_buffer.trim());
                        string_buffer.clear();
                        break;
                    }
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}

fn main() {
    let mut port = define_port();
    loop {
        send_input_data(&mut port);
        read_recieved_buffers(&mut port);
    }
}
