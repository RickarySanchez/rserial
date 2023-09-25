use serialport;
use std::io::{self, Write};
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Welcome to rserial !")
        .about("Rust based command line serial monitor tool")
        .disable_version_flag(true)
        .arg(
            Arg::new("port")
                .help("The device path to a serial port")
                .use_value_delimiter(false)
                .required(true),
        )
        .arg(
            Arg::new("baud")
                .help("The baud rate to connect at")
                .use_value_delimiter(false)
                .required(true)
                .validator(valid_baud),
        )
        .get_matches();
    
        
    let port_name = matches.value_of("port").unwrap();
    let baud_rate = matches.value_of("baud").unwrap().parse::<u32>().unwrap();

    let port = serialport::new("COM4", 115200).open();

    match port {
        Ok(mut port) => {
            let mut buffer: Vec<u8> = vec![0; 1024];
            loop {
                match port.read(buffer.as_mut_slice()) {
                    Ok(t) => io::stdout().write_all(&buffer[..t]).unwrap(),
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", "COM4", e);
            std::process::exit(1);
        }
    }
}

fn valid_baud(val: &str) -> Result<(), String> {
    val.parse::<u32>()
        .map(|_| ())
        .map_err(|_| format!("Invalid baud rate '{}' specified", val))
}
