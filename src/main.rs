mod uart;

use std::time::Duration;
use rppal::gpio::Gpio;
use rppal::uart::{Parity, Queue, Uart};

fn main() {
    println!("Hello, world! use uart on ttyAMA0");
    let mut uart_result = Uart::with_path(
        // "/dev/ttyS0",
        "/dev/ttyAMA0",
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
    _ = uart.set_write_mode(true);
    println!("uart flush");
    uart.flush(Queue::Both).unwrap();
    let mut message = String::new();
    let mut message_to_send = String::new();
    loop {
        if uart.is_read_blocking() {
            let mut short_buf = [0u8; 1];

            match uart.read(&mut short_buf) {
                Ok(size) => {
                    println!("UART read size: {}", size);
                    println!("UART read buf: {:?}", short_buf);
                    match std::str::from_utf8(&short_buf) {
                        Ok(s) => {
                            message.push_str(s);
                            if message.contains("\n") {
                                println!("Message: {:?}", message);
                                
                                let key_code = get_key_code(&message);
                                if !key_code.is_empty() {
                                    message_to_send = format!("from pi. kc={}\r\n", key_code);
                                }
                                
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
                    match uart.flush(Queue::Output) {
                        Ok(_) => {}
                        Err(err) => {
                            println!("Flush queue output error: {:?}", err);
                        }
                    }
                }
                Err(err) => {
                    println!("Error reading from UART: {:?}", err);
                }
            }
        }
        println!("Message to send: {:?}", message_to_send);
        if uart.is_write_blocking() && !message_to_send.is_empty() {
            println!("Message to send: {:?}", message_to_send);
            
            match uart.write(message_to_send.as_bytes()) {
                Ok(size) => {
                    println!("UART write size: {}", size);
                    println!("UART write buf: {:?}", message_to_send);
                }
                Err(err) => {
                    println!("Error writing to UART: {:?}", err);
                }
            }
        }
    }
}
fn get_key_code(message: &String) -> String {
    println!("Message: {:?}", message);
    let parts = message.split('&').collect::<Vec<&str>>();
    println!("Parts: {:?}", parts);
    for part in parts {
        let key_value = part.split('=').into_iter().collect::<Vec<&str>>();
        println!("Key value: {:?}", key_value);
        if key_value[0] == "kc" {
            println!("Key code: {:?}", key_value[1]);
            return String::from(key_value[1]);
        }
        // for key in key_value {
        //     if key=="kc"{
        //         return String::from(key_value[1]);
        //     }
        //     println!("Key: {:?}", key);
        // }
    }
    return String::from("");
}

