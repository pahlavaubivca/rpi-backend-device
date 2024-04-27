use std::thread;
use std::time::Duration;
use rppal::gpio::Gpio;
use rppal::uart::{Parity, Uart};

fn main() {
    println!("Hello, world!");
    let mut pin = Gpio::new();
    let gpio = pin.unwrap();
    let txd_pin = gpio.get(14).unwrap().into_output();
    let rxd_pin = gpio.get(15).unwrap().into_input();

    // let mut serial = serial::Serial::new(txd_pin, rxd_pin, 9600);

    let mut uart_result = Uart::new(19_200, Parity::None, 8, 1);
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

    println!("UART set write mode");
    uart.set_write_mode(false).unwrap();
    let str_to_send = "Hello, world!";

    loop {
        println!("UART trying to send data");
        let res = uart.write(str_to_send.as_bytes());
        if res.is_err() {
            println!("Error send data: {:?}", res.err());
        }

        println!("Sent: {:?}", str_to_send);
        thread::sleep(Duration::from_secs(1));

        // let mut buf = [0; 1];
        // uart.read(&mut buf)?;
        // println!("Received: {:?}", buf[0] as char);
    }

    // uart.
}
