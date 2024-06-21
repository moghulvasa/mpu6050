#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

const MPU6050_ADDR: u8 = 0x68;
const PWR_MGMT_1: u8 = 0x6B;
const ACCEL_XOUT_H: u8 = 0x3B;
const TEMP_OUT_H: u8 = 0x41;
const GYRO_XOUT_H: u8 = 0x43;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );

    // Wake up MPU-6050 as it starts in sleep mode
    i2c.write(MPU6050_ADDR, &[PWR_MGMT_1, 0x00]).unwrap();

    loop {
        // Read the accelerometer data
        let mut accel_buf = [0; 6];
        i2c.write_read(MPU6050_ADDR, &[ACCEL_XOUT_H], &mut accel_buf).unwrap();

        let accel_x = ((accel_buf[0] as i16) << 8) | accel_buf[1] as i16;
        let accel_y = ((accel_buf[2] as i16) << 8) | accel_buf[3] as i16;
        let accel_z = ((accel_buf[4] as i16) << 8) | accel_buf[5] as i16;

        // Read the temperature data
        let mut temp_buf = [0; 2];
        i2c.write_read(MPU6050_ADDR, &[TEMP_OUT_H], &mut temp_buf).unwrap();

        let temp_raw = ((temp_buf[0] as i16) << 8) | temp_buf[1] as i16;
        let temp_c_int = (temp_raw as i32 * 100 / 340) + 3653;

        // Read the gyroscope data
        let mut gyro_buf = [0; 6];
        i2c.write_read(MPU6050_ADDR, &[GYRO_XOUT_H], &mut gyro_buf).unwrap();

        let gyro_x = ((gyro_buf[0] as i16) << 8) | gyro_buf[1] as i16;
        let gyro_y = ((gyro_buf[2] as i16) << 8) | gyro_buf[3] as i16;
        let gyro_z = ((gyro_buf[4] as i16) << 8) | gyro_buf[5] as i16;

        // Print the sensor data
        ufmt::uwriteln!(
            &mut serial,
            "Accel X: {}, Y: {}, Z: {}\r",
            accel_x,
            accel_y,
            accel_z
        )
        .unwrap_infallible();

        // Print the temperature data as an integer
        ufmt::uwriteln!(
            &mut serial,
            "Temp: {}.{} C\r",
            temp_c_int / 100,
            temp_c_int % 100
        )
        .unwrap_infallible();

        ufmt::uwriteln!(
            &mut serial,
            "Gyro X: {}, Y: {}, Z: {}\r",
            gyro_x,
            gyro_y,
            gyro_z
        )
        .unwrap_infallible();

        arduino_hal::delay_ms(1000);
    }
}
