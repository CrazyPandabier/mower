use std::io;

use ads1x1x::{
    channel,
    ic::{Ads1115, Resolution16Bit},
    interface::I2cInterface,
    mode::OneShot,
    Ads1x1x, ChannelSelection, DynamicOneShot, Error, SlaveAddr,
};
use linux_embedded_hal::{i2cdev::linux::LinuxI2CError, nb::block, I2cdev};

pub struct BoundryConfig {
    port: String,
    address: SlaveAddr,
    threshold: i16,
    channel: ChannelSelection,
}

impl BoundryConfig {
    pub fn new(port: &str, threshold: i16) -> Self {
        BoundryConfig {
            port: port.to_string(),
            address: SlaveAddr::default(),
            channel: ChannelSelection::SingleA0,
            threshold,
        }
    }

    pub fn set_addr(mut self, addr: SlaveAddr) -> Self {
        self.address = addr;
        self
    }

    pub fn set_channel(mut self, channel: ChannelSelection) -> Self {
        self.channel = channel;
        self
    }
}

pub trait BoundryInterface {
    fn init(&mut self, config: &BoundryConfig) -> Result<(), LinuxI2CError>;
    fn detected(&mut self) -> Result<bool, Error<LinuxI2CError>>;
}

pub struct BoundrySensor {
    adc: Option<Ads1x1x<I2cInterface<I2cdev>, Ads1115, Resolution16Bit, OneShot>>,
    threshold: i16,
    channel: ChannelSelection,
}

impl BoundrySensor {
    pub fn new() -> BoundrySensor {
        BoundrySensor {
            adc: None,
            threshold: 0,
            channel: ChannelSelection::SingleA0,
        }
    }
}

impl BoundryInterface for BoundrySensor {
    fn init(&mut self, config: &BoundryConfig) -> Result<(), LinuxI2CError> {
        self.threshold = config.threshold;

        let dev = I2cdev::new(&config.port)?;
        self.adc = Some(Ads1x1x::new_ads1115(dev, config.address));

        Ok(())
    }

    fn detected(&mut self) -> Result<bool, Error<LinuxI2CError>> {
        if let Some(adc) = &mut self.adc {
            let measurement = block!(adc.read(self.channel))?;
            println!("data: {}", measurement);
            if measurement > self.threshold {
                return Ok(true);
            }

            return Ok(false);
        }

        Err(Error::I2C(LinuxI2CError::Io(io::Error::new(
            io::ErrorKind::BrokenPipe,
            "Issue with I2C connection",
        ))))
    }
}
