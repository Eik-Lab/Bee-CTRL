#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use bme280::{Measurements, BME280};
use linux_embedded_hal::{Delay, I2cdev};
use mlx9064x::{CameraDriver, FrameRate, Mlx90640Driver, mlx90640::{Mlx90640, Mlx90640Calibration}};
use tmp117::TMP117;
extern crate tmp117;
use crate::models::Measurement;
use chrono;
use dotenv;
use image::{self, ImageBuffer};
use palette::Srgb;
use std::{
    sync::{Arc, Mutex},
    thread::{sleep, sleep_ms},
    time::Duration,
};
pub mod models;
pub mod schema;

fn main() {
    dotenv::dotenv().ok();
    // Initialize database
    let db_url = match std::env::var("DATABASE_URL") {
        Ok(db_url) => db_url,
        Err(_) => {
            println!("DATABASE_URL not set, creating empty container");
            panic!()
        }
    };
    let pool = crate::models::init_pool(&db_url);

    let (tmp, mut bme280, mut camera) = init_sensors();
    let temp = tmp.read().unwrap();
    let v = FrameRate::Half;
    println!("set_framerate: {:?}", 
    camera.set_frame_rate(v));

    println!("Temperature: {}", temp);
    let measurements = bme280.measure().unwrap();
    println!("Relative Humidity = {}%", measurements.humidity);

    loop {
        // let mut temperatures = Vec::new();
        let mut temperatures1 = vec![0f32; camera.height() * camera.width()];
        let mut temperatures2 = vec![0f32; camera.height() * camera.width()];
        let page = loop {
            if let Ok(Some(page)) = camera.data_available() {
                camera.generate_image_if_ready(&mut temperatures1).unwrap();
                break page
            }
            sleep(Duration::from_millis(100));
        };

        loop {
            if let Ok(Some(new_page) ) = camera.data_available() {
                if new_page != page {
                    camera.generate_image_if_ready(&mut temperatures2).unwrap();
                    break;
                }
            }
            sleep(Duration::from_millis(100));
        }
        
        let t_out = temperatures1.into_iter().zip(temperatures2.into_iter()).map( |(v1, v2)| {
            v1.max(v2)
        }).collect();
        
        let now = chrono::Utc::now();
        // Save_image does not consume the image, but Measurement{} does
        save_image(
            &t_out,
            camera.width(),
            format!("/home/pi/images/{}.png", now).as_ref(),
        );

        let conn = pool.get().unwrap();

        let measurements = Measurement {
            pi_id: 1,
            measurement_time: now,
            temp1: 1.0,
            temp2: 0.2,
            temp3: 0.3,
            temp4: 0.4,
            bme_temp1: 0.5,
            bme_temp2: 0.6,
            pressure1: 0.1013,
            pressure2: 0.1023,
            rh1: 69.420,
            rh2: 420.69,
            altitude1: 0.100,
            altitude2: 0.200,
            image_name: t_out,
        }
        .insert(&conn);
        println!("Inserted data!{:?}", measurements);

        let temp = tmp.read().unwrap();
        println!("Temperature: {}", temp);
        sleep_ms(1000);
    }
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

fn save_image(buffer: &Vec<f32>, width: usize, filename: &str) {
    let min = buffer.iter().cloned().fold(0. / 0., f32::min);
    let max = buffer.iter().cloned().fold(0. / 0., f32::max);
    let mut out_image: Vec<u8> = Vec::new();
    for v in buffer {
        out_image.push((((v - min) / (max - min)) * 256_f32) as u8);
    }
    image::save_buffer(
        filename,
        &out_image,
        width as u32,
        (buffer.len() / width) as u32,
        image::ColorType::L8,
    )
    .unwrap();
}
