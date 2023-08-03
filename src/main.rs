use ctrlc::set_handler;
use serialport::SerialPort;
use std::io;
use std::io::Read;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

fn define_port() -> Arc<Mutex<Box<dyn SerialPort>>> {
    let port = serialport::new("COM8", 9600)
        .open()
        .expect("Failed to open serial port");
    Arc::new(Mutex::new(port))
}

fn user_input_task(port: Arc<Mutex<Box<dyn SerialPort>>>) {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut mtx_port = match port.try_lock() {
        Ok(guard) => guard,
        Err(_) => {
            eprintln!("Failed to acquire lock. Another thread might be using the port.");
            // TODO elvenni a lock-ot a másik száltól ugyanis most nekem kell
            return;
        }
    };

    mtx_port.write_all(input.as_bytes()).expect("Write failed!");
}

fn recieve_data(port: Arc<Mutex<Box<dyn SerialPort>>>) {
    let mut buffer: [u8; 1] = [0; 1];
    let mut string_buffer = String::new();

    let mut mtx_port = port.lock().unwrap();

    loop {
        match mtx_port.read(&mut buffer) {
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
    let port = define_port();
    let port_thread1 = port.clone();
    let port_thread2 = port.clone();

    set_handler(|| {
        println!("\nCtrl + C detected. Exit...\n");
        std::process::exit(0);
    })
    .expect("\nError with Ctrl + C detection.\n");

    thread::spawn(move || loop {
        thread::sleep(std::time::Duration::from_millis(2000));
        recieve_data(port_thread1.clone());
    });

    thread::spawn(move || loop {
        user_input_task(port_thread2.clone());
    });

    loop {
        thread::park();
    }
}
