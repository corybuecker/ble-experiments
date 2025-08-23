use anyhow::{Result, anyhow};
use embedded_hal::i2c::{self, I2c};
use linux_embedded_hal::I2cdev;
use std::{error::Error, thread::sleep, time::Duration};
use tracing::{debug, info};
use tracing_subscriber::layer::SubscriberExt;

fn main() -> Result<(), Box<dyn Error>> {
    tracing::subscriber::set_global_default(
        tracing_subscriber::registry().with(tracing_subscriber::fmt::layer().pretty()),
    )
    .expect("setting default subscriber failed");

    let i2c = I2cdev::new("/dev/i2c-1")?;
    let device_address: u8 = 0x40; // A0 and A1 tied to GND, 1000000, R/W low

    reset_device(i2c, device_address)?;

    Ok(())
}

fn reset_device(mut i2c: I2cdev, address: u8) -> Result<()> {
    let reset_register: u8 = 0x00; // 0000 0000
    let reset_device_command: [u8; 3] = [reset_register, 0x80, 0x00];

    let mut register_buffer = [0u8; 2];

    i2c.write(address, &reset_device_command)?;
    i2c.write_read(address, &[reset_register], &mut register_buffer)?;

    if register_buffer.eq(&[0x00, 0x00]) == false {
        return Err(anyhow!("Failed to reset device"));
    }

    std::thread::sleep(std::time::Duration::from_millis(10));

    info!("Device reset successfully");

    Ok(())
}
