use std::thread;
use std::time::Duration;
use rppal::gpio::Gpio;
use rppal::uart::{Parity, Queue, Uart};

fn main() {
    println!("Hello, world!");
    let mut pin = Gpio::new();
    let gpio = pin.unwrap();
    let txd_pin = gpio.get(14).unwrap().into_output();
    let rxd_pin = gpio.get(15).unwrap().into_input();

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
        19200,
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

    println!("UART initialized");
    uart.flush(Queue::Both).unwrap();

    println!("UART set write mode");
    uart.set_write_mode(true).unwrap();
    let str_to_send = "h";

    loop {
        println!("UART trying to send data 1");
        // uart.flush(Queue::Input).unwrap();
        // uart.flush(Queue::Output).unwrap();
        uart.flush(Queue::Both).unwrap();
        uart.send_start().unwrap();
        let res = uart.write(str_to_send.as_bytes());
        match res {
            Ok(bs) => {
                println!("UART sent data, size: {}", bs);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }

        uart.send_stop().unwrap();


        println!("Sent: {:?}", str_to_send);
        thread::sleep(Duration::from_secs(1));

        // let mut buf = [0; 1];
        // uart.read(&mut buf)?;
        // println!("Received: {:?}", buf[0] as char);
    }

    // uart.
}
