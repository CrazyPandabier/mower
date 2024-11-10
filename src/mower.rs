use std::io::Error;

use crate::{boundry::BoundryInterface, motor::MotorInterface};

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
        mow_config: <M as MotorInterface>::Config,
        left_config: <M as MotorInterface>::Config,
        right_config: <M as MotorInterface>::Config,
    ) -> Result<(), Error> {
        self.mow_motor.init(mow_config)?;
        self.left_motor.init(left_config)?;
        self.right_motor.init(right_config)?;
        Ok(())
    }

    pub fn update() {}
}
