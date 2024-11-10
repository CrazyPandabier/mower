use boundry::{BoundryConfig, BoundryInterface, BoundrySensor};

mod boundry;
mod motor;
mod mower;

fn main() {
    let mut boundry = BoundrySensor::new();
    let config = BoundryConfig::new("/dev/i2c-1", 0x00, 0);
    boundry.init(&config).unwrap();

    loop {
        boundry.detected().unwrap();
    }
}
