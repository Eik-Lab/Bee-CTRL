#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use bme280::BME280;
use linux_embedded_hal::{Delay, I2cdev};
use mlx9064x::{
    mlx90640::{Mlx90640, Mlx90640Calibration},
    CameraDriver, FrameRate, Mlx90640Driver,
};
use tmp117::TMP117;
extern crate tmp117;
use crate::models::Measurement;
use dotenv;
use image;
use std::str;

use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};
pub mod models;
pub mod schema;
use clap::{App, Arg, SubCommand};
use std::process::Command;

fn get_refresh_rate() -> FrameRate {
    let matches = App::new("Bee-CTRL")
        .version("0.0.1")
        .about("CTRL the Bees!")
        .author("Uzair Aftab, <uzaaft@outlook.com>")
        .arg(Arg::with_name("FRAMERATE")
        .short("rf")
        .value_name("MLX90640 framerate in HZ")
        .help("Set a custom framerate in HZ Defaults to 0.5HZ . Accepted HZ are 0.5, 1, 2,4,8,16,32,64"))
    .get_matches();

    let fps = matches.value_of("framerate").unwrap_or("O.5");
    println!("{}", fps);
    let framerate = match fps {
        "0.5" => FrameRate::Half,
        "1" => FrameRate::One,
        "2" => FrameRate::Two,
        "4" => FrameRate::Four,
        "8" => FrameRate::Eight,
        "16" => FrameRate::Sixteen,
        "32" => FrameRate::ThirtyTwo,
        "64" => FrameRate::SixtyFour,
        _ => FrameRate::Half,
    };
    println!("Refresh was set to: {}", fps);
    framerate
}

fn main() {
    dotenv::dotenv().ok();

    let v = get_refresh_rate();
    // Initialize database
    let sn = get_sn();
    println!("SN: {}", sn);
    // get_refresh_rate();

    let db_url = match std::env::var("DATABASE_URL") {
        Ok(db_url) => db_url,
        Err(_) => {
            println!("DATABASE_URL not set, creating empty container");
            panic!()
        }
    };
    let pool = crate::models::init_pool(&db_url);
    // This can be done more elegantly
    let (tmp1, tmp2, tmp3, tmp4, mut bme280_1, mut bme280_2, mut camera) = init_sensors();
    println!("set_framerate: {:?}", camera.set_frame_rate(v));

    // Potential one-liner?
    let bme_1_measurements = bme280_1.measure().unwrap();
    let bme_2_measurements = bme280_2.measure().unwrap();
    loop {
        // let mut temperatures = Vec::new();

        /* TODO:
                The camera code can be moved to a separate function. Something like:
                fn get_image(camera: Arc<Mutex<CameraDriver<...>>>) -> Vec<f32, Global>{}
        */
        /*
        TLDR of what happens below
        The sensor creates to vectors which are the same size, but where every other pixel is filled.
        I.e, image 1 is every odd pixel, and image 2 contains every even pixel.
        The two loops belov makes sure that we get both "pages", before combining them
        */
        let mut temperatures1 = vec![0f32; camera.height() * camera.width()];
        let mut temperatures2 = vec![0f32; camera.height() * camera.width()];
        let page = loop {
            if let Ok(Some(page)) = camera.data_available() {
                camera.generate_image_if_ready(&mut temperatures1).unwrap();
                break page;
            }
            sleep(Duration::from_millis(10));
        };

        loop {
            if let Ok(Some(new_page)) = camera.data_available() {
                if new_page != page {
                    camera.generate_image_if_ready(&mut temperatures2).unwrap();
                    break;
                }
            }
            sleep(Duration::from_millis(10));
        }

        let t_out = temperatures1
            .into_iter()
            .zip(temperatures2.into_iter())
            .map(|(v1, v2)| v1.max(v2))
            .collect();

        let now = chrono::Utc::now();
        // Save_image does not consume the image, but Measurement{} does
        save_image(
            &t_out,
            camera.width(),
            format!("/home/pi/images/{}.png", now).as_ref(),
        );

        let conn = pool.get().unwrap();
        /* TODO:
        Move this to a separeate method
        */
        let measurements = Measurement {
            pi_id: 1.to_string(),
            measurement_time: now,
            temp1: tmp1.read().unwrap(),
            temp2: tmp2.read().unwrap(),
            temp3: tmp3.read().unwrap(),
            temp4: tmp4.read().unwrap(),
            bme_temp1: bme_1_measurements.temperature,
            bme_temp2: bme_2_measurements.temperature,
            pressure1: bme_1_measurements.pressure,
            pressure2: bme_2_measurements.pressure,
            rh1: bme_2_measurements.humidity,
            rh2: bme_2_measurements.humidity,
            image_data: t_out,
        }
        .insert(&conn);
        println!("Inserted data!{:?}", measurements);

        sleep(Duration::from_millis(1000));
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

fn get_sn() -> String {
    let output = Command::new("cat")
        .arg("/sys/firmware/devicetree/base/serial-number")
        .output()
        .unwrap();

    let sn = str::from_utf8(&output.stdout).unwrap();
    return sn.to_string();
}

fn init_sensors() -> (
    TMP117<I2cdev>,
    TMP117<I2cdev>,
    TMP117<I2cdev>,
    TMP117<I2cdev>,
    BME280<I2cdev, Delay>,
    BME280<I2cdev, Delay>,
    CameraDriver<Mlx90640, Mlx90640Calibration, I2cdev, 24_usize, 32_usize, 1536_usize>,
) {
    let i2c_bus = Arc::new(Mutex::new(I2cdev::new("/dev/i2c-1").unwrap()));
    let tmp1 = tmp117::TMP117::primary_default(i2c_bus.clone());
    let tmp2 = tmp117::TMP117::secondary_default(i2c_bus.clone());
    let tmp3 = tmp117::TMP117::tertiary_default(i2c_bus.clone());
    let tmp4 = tmp117::TMP117::quaternary_default(i2c_bus.clone());
    let mut bme280_1 = BME280::new_primary(i2c_bus.clone(), Delay);
    let mut bme280_2 = BME280::new_primary(i2c_bus.clone(), Delay);
    bme280_1.init().unwrap();
    bme280_2.init().unwrap();
    let camera_address = 0x33;
    let mut camera = Mlx90640Driver::new(i2c_bus.clone(), camera_address).unwrap();
    (tmp1, tmp2, tmp3, tmp4, bme280_1, bme280_2, camera)
}

fn save_image(buffer: &Vec<f32>, width: usize, filename: &str) {
    let min = buffer.iter().cloned().fold(f32::NAN, f32::min);
    let max = buffer.iter().cloned().fold(f32::NAN, f32::max);
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
