
use std::time::Duration;

use tokio_serial::available_ports;
use tokio_serial::SerialPortBuilderExt;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::time::timeout;

#[tokio::main]
async fn main() -> io::Result<()> {
    // List all available serial ports on this device.
    // These could include TTYs, Bluetooth, and a few others
    match available_ports() {
        Err(e) => {
            println!("No ports attached to the system! '{e}'");
            return Ok(())
        },
        Ok(ports) => {
            // Ok, we've found some ports connected to the system.
            // Loop through each of them and configure them with some
            // standard parameters. After each one is configured,
            // send a short string then wait for some data, with a timeout.
            println!("{ports:?}");
            println!("Separates:");
            for port in ports {
                println!("{port:?}");

                // We've now found a port.
                // Now initialize it
                let mut port = tokio_serial::new(port.port_name, 19200)
                    .flow_control(tokio_serial::FlowControl::None)
                    .data_bits(tokio_serial::DataBits::Eight)
                    .parity(tokio_serial::Parity::None)
                    .stop_bits(tokio_serial::StopBits::Two)
                    .open_native_async()
                    .unwrap_or_else ( |e| {
                        println!("Error opening port: '{e:?}'");
                        std::process::exit(-1);
                    });

                // Let other programs access the serial port while we have it
                #[cfg(unix)]
                port.set_exclusive(false)
                    .expect("Unable to set serial port exclusive to false");

                // Loopback testing
                let mut buf = [0; 8];
                let _num_bytes_written = port.write(b"01234567").await;
                match timeout(Duration::from_millis(500), port.read(&mut buf)).await {
                    Err(e) => println!("Timeout! '{e}'"),
                    Ok(read_result) => match read_result {
                        Err(e) => println!("Error! '{e}'"),
                        Ok(num_bytes_read) => println!("Received {num_bytes_read} bytes:\n{buf:?}")
                    },
                };

            }
        },
    }
    Ok(())
}
