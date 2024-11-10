use std::{io::Error, thread::sleep, time::Duration};

use rand::Rng;

use crate::{boundry::BoundryInterface, motor::MotorInterface};

const MAX_ROT_TIME: f32 = 4.0;
const MAX_SPEED: f32 = 0.5;

pub struct MowerController<M: MotorInterface, B: BoundryInterface> {
    mow_motor: Box<M>,
    left_motor: Box<M>,
    right_motor: Box<M>,
    boundry_sensor: Box<B>,
}

impl<M: MotorInterface, B: BoundryInterface> MowerController<M, B> {
    pub fn new(
        left_motor: Box<M>,
        right_motor: Box<M>,
        mow_motor: Box<M>,
        boundry_sensor: Box<B>,
    ) -> Self {
        MowerController {
            mow_motor,
            left_motor,
            right_motor,
            boundry_sensor,
        }
    }

    pub fn init(
        &mut self,
        mow_config: &<M as MotorInterface>::Config,
        left_config: &<M as MotorInterface>::Config,
        right_config: &<M as MotorInterface>::Config,
        boundry_config: &crate::boundry::BoundryConfig,
    ) -> Result<(), Error> {
        self.mow_motor.init(mow_config)?;
        self.left_motor.init(left_config)?;
        self.right_motor.init(right_config)?;
        self.boundry_sensor.init(boundry_config)?;
        Ok(())
    }

    pub fn update(&mut self) {
        self.set_speed(MAX_SPEED).unwrap();

        self.forward().unwrap();

        if self.boundry_sensor.detected().unwrap() {
            self.backward().unwrap();
            sleep(Duration::from_millis(2000));

            let mut rng = rand::thread_rng();
            let rot_time = rng.gen_range(1.0..MAX_ROT_TIME);

            if rot_time >= MAX_ROT_TIME / 2.0 {
                self.left().unwrap();
            } else {
                self.right().unwrap();
            }

            sleep(Duration::from_secs(rot_time as u64));
        }
    }

    fn set_speed(&mut self, speed: f32) -> Result<(), Error> {
        self.left_motor.set_speed(speed)?;
        self.right_motor.set_speed(speed)?;
        Ok(())
    }

    fn forward(&mut self) -> Result<(), Error> {
        self.left_motor.rotate_left()?;
        self.right_motor.rotate_right()?;
        Ok(())
    }

    fn backward(&mut self) -> Result<(), Error> {
        self.left_motor.rotate_right()?;
        self.right_motor.rotate_left()?;
        Ok(())
    }

    fn left(&mut self) -> Result<(), Error> {
        self.left_motor.rotate_left()?;
        self.right_motor.rotate_left()?;
        Ok(())
    }

    fn right(&mut self) -> Result<(), Error> {
        self.left_motor.rotate_right()?;
        self.right_motor.rotate_right()?;
        Ok(())
    }
}
