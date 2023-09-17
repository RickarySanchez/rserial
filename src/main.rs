use serialport;

fn main() {
    println!("Hello, world!");
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    let com1 = serialport::new("COM1", 9600).open().expect("Failed to open port");

}
