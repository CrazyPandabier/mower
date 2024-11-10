use boundry::{BoundryConfig, BoundryInterface, BoundrySensor};

mod boundry;
mod motor;
mod mower;

fn main() {
    let mut boundry = BoundrySensor::new();
    let config =
        BoundryConfig::new("/dev/i2c-1", 0).set_channel(ads1x1x::ChannelSelection::SingleA0);
    boundry.init(&config).unwrap();

    loop {
        boundry.detected().unwrap();
    }
}
