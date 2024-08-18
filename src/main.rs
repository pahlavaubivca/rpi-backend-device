use std::time::Duration;
use rppal::gpio::Gpio;
use rppal::uart::{Parity, Queue, Uart};

fn main() {
    println!("Hello, world!");
    let mut uart_result = Uart::new(
        // 19_200,
        115_200,
        // 38400,
        Parity::Odd,
        8,
        1,
    );

    let mut uart;
    match uart_result {
        Ok(_uart) => {
            uart = _uart;
            println!("UART initialized");
        }
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    }

    println!("uart set read mode");
    _ = uart.set_read_mode(1, Duration::default());
    println!("uart flush");
    uart.flush(Queue::Both).unwrap();
    let mut message = String::new();
    loop {
        let mut short_buf = [0; 1];
        match uart.read(&mut short_buf) {
            Ok(size) => {
                println!("UART read size: {}", size);
                println!("UART read buf: {:?}", short_buf);
                match std::str::from_utf8(&short_buf) {
                    Ok(s) => {
                        message.push_str(s);
                        if message.contains("\r\n") {
                            println!("Message: {:?}", message);
                            message = String::new();
                            // match uart.flush(Queue::Both) {
                            //     Ok(res) => {
                            //         println!("Flush queue output result: {:?}", res);
                            //     }
                            //     Err(err) => {
                            //         println!("Flush queue output error: {:?}", err);
                            //     }
                            // }
                        }

                        println!("UART read: {:?}", s);
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
            }
            Err(err) => {
                println!("Error reading from UART: {:?}", err);
            }
        }
        
    }
}
