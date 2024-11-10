use std::io::Error;

use vesc_api::{BaudRate, Vesc};

pub trait MotorInterface {
    type Config;

    fn init(&mut self, config: Self::Config) -> Result<(), Error>;
    fn RotateLeft(&mut self) -> Result<(), Error>;
    fn RotateRight(&mut self) -> Result<(), Error>;
    fn SetSpeed(&mut self, speed: f32) -> Result<(), Error>;
}

pub struct MotorConfig {
    port: String,
    baud: BaudRate,
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

    fn init(&mut self, config: MotorConfig) -> Result<(), Error> {
        self.vesc = Some(Vesc::new(&config.port, config.baud)?);
        Ok(())
    }

    fn RotateLeft(&mut self) -> Result<(), Error> {
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

    fn RotateRight(&mut self) -> Result<(), Error> {
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

    fn SetSpeed(&mut self, speed: f32) -> Result<(), Error> {
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
}
