use bme280::BME280;
use embedded_hal as hal;
use linux_embedded_hal::{Delay, I2cdev};
use mlx9064x::{
    mlx90640::{Mlx90640, Mlx90640Calibration},
    CameraDriver, Mlx90640Driver,
};
use tmp117::TMP117;
extern crate tmp117;
use image::{self, ImageBuffer};
use palette::Srgb;
use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

fn main() {
    // using Linux I2C Bus #1 in this example
    let (tmp, mut bme280, mut camera) = init_sensors();
    let temp = tmp.read().unwrap();

    println!("Temperature: {}", temp);
    let measurements = bme280.measure().unwrap();
    println!("Relative Humidity = {}%", measurements.humidity);

    let mut temperatures = vec![0f32; camera.height() * camera.width()];
    let delay = Duration::from_millis(500);
    camera.generate_image_if_ready(&mut temperatures).unwrap();
    sleep(delay);
    camera.generate_image_if_ready(&mut temperatures).unwrap();
    save_image(&temperatures, camera.width())
}

fn print_temperatures(temperatures: &[f32], width: usize) {
    for (count, temperature) in temperatures.iter().enumerate() {
        if count % width == 0 {
            println!();
        }
        print!("{:4.2}  ", temperature);
    }
}

fn init_sensors() -> (
    TMP117<I2cdev>,
    BME280<I2cdev, Delay>,
    CameraDriver<Mlx90640, Mlx90640Calibration, I2cdev, 24_usize, 32_usize, 1536_usize>,
) {
    let i2c_bus = Arc::new(Mutex::new(I2cdev::new("/dev/i2c-1").unwrap()));
    let tmp = tmp117::TMP117::new_default(i2c_bus.clone());
    let mut bme280 = BME280::new_primary(i2c_bus.clone(), Delay);
    bme280.init().unwrap();
    let camera_address = 0x33;
    let mut camera = Mlx90640Driver::new(i2c_bus.clone(), camera_address).unwrap();
    return (tmp, bme280, camera);
}

fn save_image(buffer: &Vec<f32>, width: usize) {
    let min = buffer.iter().cloned().fold(0./0., f32::min);
    let max = buffer.iter().cloned().fold(0./0., f32::max);
    let mut out_image: Vec<u8> = Vec::new();
    for v in buffer {
        out_image.push((((v - min) / (max - min)) * 256_f32) as u8);
    }
    image::save_buffer(
        "./test.png",
        &out_image,
        width as u32,
        (buffer.len() / width) as u32,
        image::ColorType::L8,
    ).unwrap();
}
