use std::io::Error;

use vesc_api::{BaudRate, Vesc};

pub trait MotorInterface {
    type Config;

    fn init(&mut self, config: &Self::Config) -> Result<(), Error>;
    fn rotate_left(&mut self) -> Result<(), Error>;
    fn rotate_right(&mut self) -> Result<(), Error>;
    fn set_speed(&mut self, speed: f32) -> Result<(), Error>;
    fn stop(&mut self) -> Result<(), Error>;
}

pub struct MotorConfig {
    port: String,
    baud: BaudRate,
}

impl MotorConfig {
    pub fn new(port: &str, baud: BaudRate) -> Self {
        MotorConfig {
            port: port.to_string(),
            baud,
        }
    }
}

pub struct Motor {
    vesc: Option<Vesc>,
    curr_speed: f32,
    rotating_left: bool,
}

impl Motor {
    pub fn new() -> Self {
        Motor {
            vesc: None,
            curr_speed: 0.0,
            rotating_left: false,
        }
    }
}

impl MotorInterface for Motor {
    type Config = MotorConfig;

    fn init(&mut self, config: &MotorConfig) -> Result<(), Error> {
        self.vesc = Some(Vesc::new(&config.port, config.baud)?);
        Ok(())
    }

    fn rotate_left(&mut self) -> Result<(), Error> {
        if let Some(vesc) = &mut self.vesc {
            self.rotating_left = true;
            vesc.set_duty_cycle(self.curr_speed)?;

            return Ok(());
        }

        Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "vesc not initialized",
        ))
    }

    fn rotate_right(&mut self) -> Result<(), Error> {
        if let Some(vesc) = &mut self.vesc {
            self.rotating_left = false;
            vesc.set_duty_cycle(-self.curr_speed)?;

            return Ok(());
        }

        Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "vesc not initialized",
        ))
    }

    fn set_speed(&mut self, speed: f32) -> Result<(), Error> {
        let speed = speed.abs();
        if let Some(vesc) = &mut self.vesc {
            if self.rotating_left {
                vesc.set_duty_cycle(speed)?;
            } else {
                vesc.set_duty_cycle(-speed)?;
            }

            return Ok(());
        }

        Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "vesc not initialized",
        ))
    }

    fn stop(&mut self) -> Result<(), Error> {
        if let Some(vesc) = &mut self.vesc {
            vesc.set_duty_cycle(0.0)?;
            return Ok(());
        }

        Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "vesc not initialized",
        ))
    }
}
