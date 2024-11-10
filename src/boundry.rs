extern crate i2c_linux;
use std::{fs::File, io::Error};

use i2c_linux::I2c;

pub struct BoundryConfig {
    port: String,
    address: u16,
    threshold: u8,
}

impl BoundryConfig {
    pub fn new(port: &str, address: u16, threshold: u8) -> Self {
        BoundryConfig {
            port: port.to_string(),
            address,
            threshold,
        }
    }
}

pub trait BoundryInterface {
    fn init(&mut self, config: &BoundryConfig) -> Result<(), Error>;
    fn detected(&mut self) -> Result<bool, Error>;
}

pub struct BoundrySensor {
    i2c: Option<I2c<File>>,
    threshold: u8,
}

impl BoundrySensor {
    pub fn new() -> BoundrySensor {
        BoundrySensor {
            i2c: None,
            threshold: 0,
        }
    }
}

impl BoundryInterface for BoundrySensor {
    fn init(&mut self, config: &BoundryConfig) -> Result<(), Error> {
        self.threshold = config.threshold;

        let mut i2c = I2c::from_path(&config.port)?;
        i2c.smbus_set_slave_address(config.address, false)?;

        self.i2c = Some(i2c);
        Ok(())
    }

    fn detected(&mut self) -> Result<bool, Error> {
        if let Some(i2c) = &mut self.i2c {
            let byte = i2c.smbus_read_byte()?;
            println!("data: {}", byte);
            return Ok(false);
        }

        Err(Error::new(
            std::io::ErrorKind::NotConnected,
            "no connection with i2c device",
        ))
    }
}
