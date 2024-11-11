use boundry::{BoundryConfig, BoundrySensor};
use motor::{Motor, MotorConfig};
use mower::MowerController;
use vesc_api::BaudRate::Baud115200;

mod boundry;
mod motor;
mod mower;

fn main() {
    let boundry = BoundrySensor::new();
    let left_motor = Motor::new();
    let right_motor = Motor::new();
    let mow_motor = Motor::new();
    let mut mower = MowerController::new(
        Box::new(left_motor),
        Box::new(right_motor),
        Box::new(mow_motor),
        Box::new(boundry),
    );

    let boundry_config = BoundryConfig::new("/dev/i2c-1", 900)
        .set_channel_1(ads1x1x::ChannelSelection::SingleA0)
        .set_channel_2(ads1x1x::ChannelSelection::SingleA1);
    let left_config = MotorConfig::new("/dev/ttyAMA2", Baud115200);
    let right_config = MotorConfig::new("/dev/ttyAMA3", Baud115200);
    let mow_config = MotorConfig::new("/dev/ttyAMA4", Baud115200);
    mower
        .init(&mow_config, &left_config, &right_config, &boundry_config)
        .unwrap();

    loop {
        mower.update();
    }
}
