use bme280::BME280;
use embedded_hal as hal;
use linux_embedded_hal::{Delay, I2cdev};
extern crate tmp117;
use std::sync::{Arc, Mutex};

fn main() {
    // using Linux I2C Bus #1 in this example
    let i2c_bus = Arc::new(Mutex::new(I2cdev::new("/dev/i2c-1").unwrap()));
    let tmp = tmp117::TMP117::new_default(i2c_bus.clone());
    let temp = tmp.read().unwrap();
    println!("Temperature: {}", temp);
    let mut bme280 = BME280::new_primary(i2c_bus.clone(), Delay);
    bme280.init().unwrap();
    let measurements = bme280.measure().unwrap();
    println!("Relative Humidity = {}%", measurements.humidity);
}
