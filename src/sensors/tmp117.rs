use bme280::BME280;
use linux_embedded_hal as hal;
use linux_embedded_hal::{Delay, I2cdev};
extern crate tmp117;

fn main() {
    // using Linux I2C Bus #1 in this example
    let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
    let mut tmp = tmp117::TMP117::new_default(i2c_bus);
    let temp = tmp.read().unwrap();
    println!("temp :{:.1}", temp);
}
