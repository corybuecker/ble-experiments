use embedded_hal::i2c::I2c;
use linux_embedded_hal::I2cdev;
use std::error::Error;
use tracing::debug;
use tracing_subscriber::layer::SubscriberExt;

fn main() -> Result<(), Box<dyn Error>> {
    let stdout_log = tracing_subscriber::fmt::layer().pretty();
    let subscriber = tracing_subscriber::registry().with(stdout_log);
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let mut i2c = I2cdev::new("/dev/i2c-1")?;
    let device_address: u16 = 0x40;

    match i2c.write(device_address, &[0x80]) {
        Ok(_) => {
            let result = i2c.write(device_address, &[0x00, 0x80, 0x00]);
            debug!("Write result: {:?}", result);
        }
        Err(e) => {
            debug!("Failed to write to device: {}", e);
            return Err(Box::new(e));
        }
    }

    // debug!("Reading from device at address: 0x{:02X}", device_address);
    // let mut buffer = [0u8; 2];
    // match i2c.read(device_address, &mut buffer) {
    //     Ok(_) => {
    //         debug!("Read data: {:02X?}", buffer);
    //     }
    //     Err(e) => {
    //         debug!("Failed to read from device: {}", e);
    //         return Err(Box::new(e));
    //     }
    // }
    Ok(())
}
