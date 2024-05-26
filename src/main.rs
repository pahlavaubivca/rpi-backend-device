use std::thread;
use std::time::Duration;
use rppal::gpio::Gpio;
use rppal::uart::{Parity, Queue, Uart};

fn main() {
    println!("Hello, world!");
    let mut pin = Gpio::new();
    let gpio = pin.unwrap();
    // let txd_pin = gpio.get(14).unwrap().into_output();
    // let rxd_pin = gpio.get(15).unwrap().into_input();

    // let mut serial = serial::Serial::new(txd_pin, rxd_pin, 9600);

    // let mut uart_result = Uart::with_path(
    //     "/dev/ttyAMA0",
    //     // "/dev/ttyS0",
    //     19200,
    //     Parity::None,
    //     8,
    //     1,
    // );

    let mut uart_result = Uart::new(
        19_200,
        // 38400,
        Parity::None,
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
    _ = uart.set_read_mode(1, Duration::from_millis(100));
    println!("uart flush");
    uart.flush(Queue::Both).unwrap();
    let mut buf = [0; 25];
    loop {
 

        if uart.is_read_blocking() {
            println!("UART trying to read data");
            // if !first {
            //     uart.flush(Queue::Both).unwrap();
            // }
            // first = false;
            buf = [0; 25];
            let read_res = uart.read(&mut buf).unwrap();
            if read_res > 0 {
                println!("UART read: {:?}", std::str::from_utf8(&buf).unwrap());
            }
            println!("UART read. empty");
        }
        // let res = uart.write(str_to_send.as_bytes());
        // match res {
        //     Ok(bs) => {
        //         println!("UART sent data, size: {}", bs);
        //     }
        //     Err(e) => {
        //         println!("Error: {:?}", e);
        //     }
        // }    
        // 
        // uart.send_stop().unwrap();
        println!("loop...");

        // println!("Sent: {:?}", str_to_send);
        thread::sleep(Duration::from_secs(1));

        // let mut buf = [0; 1];
        // uart.read(&mut buf)?;
        // println!("Received: {:?}", buf[0] as char);
    }

    // uart.
}
