use std::thread;
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
        //
        //
        // if let Ok(read_res) = uart.read(&mut short_buf) {
        //     if read_res > 0 {
        //         println!("UART read size: {}", read_res);
        //         println!("UART read buf: {:?}", short_buf);
        //         let slice = &short_buf[0..read_res];
        //         match std::str::from_utf8(&short_buf) {
        //             Ok(s) => {
        //                 message.push_str(s);
        //                 if message.contains("\r\n") {
        //                     println!("Message: {:?}", message);
        //                     message = String::new();
        //                     // match uart.flush(Queue::Both) {
        //                     //     Ok(res) => {
        //                     //         println!("Flush queue output result: {:?}", res);
        //                     //     }
        //                     //     Err(err) => {
        //                     //         println!("Flush queue output error: {:?}", err);
        //                     //     }
        //                     // }
        //                 }
        //
        //                 println!("UART read: {:?}", s);
        //             }
        //             Err(e) => {
        //                 println!("Error: {:?}", e);
        //             }
        //         }
        //
        //     } else {
        //         // println!("No data read from UART");
        //     }
        // } else {
        //     println!("Error reading from UART");
        // }


        // println!("UART trying to write data");
        // let raw_str = "top_row=127.0.0.1/100&bottom_row=<-modes/select/settings->;";
        // let let_of_raw_str = raw_str.len() as u32;
        // 
        // let mut str_len_in_bytes = let_of_raw_str.to_be_bytes().to_vec();
        // let raw_str_bytes = raw_str.as_bytes().to_vec();
        // str_len_in_bytes.extend(raw_str_bytes);
        // println!("str_len_in_bytes: {:?}", str_len_in_bytes);
        // // uart.flush(Queue::Input).unwrap();
        // 
        // let res = uart.write(str_len_in_bytes.as_slice());
        // match res {
        //     Ok(bs) => {
        //         println!("UART sent data, size: {}", bs);
        //     }
        //     Err(e) => {
        //         println!("Error: {:?}", e);
        //     }
        // }


        // uart.send_stop().unwrap();
        // println!("loop...");

        // println!("Sent: {:?}", str_to_send);


        // thread::sleep(Duration::from_millis(500));

        // let mut buf = [0; 1];
        // uart.read(&mut buf)?;
        // println!("Received: {:?}", buf[0] as char);
    }

    // uart.
}
