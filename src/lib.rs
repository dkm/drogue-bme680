#![deny(unsafe_code)]
#![no_std]

//! `drogue-bme680` is a crate to interface with the BME680 sensor.
//!
//! The BME680 sensor is a "low power gas, pressure, temperature & humidity sensor" from
//! Bosch Sensortec.
//!
//! The crate provides two layers of access to the sensor. [`Bme680Sensor`] provides general
//! purpose access to the sensor. While [`Bme680Controller`] takes care of most steps of the
//! default sensor workflow for taking measurements, providing a simpler, more user-friendly
//! interface.
//!
//! # Example
//!
//! ~~~no_run
//! use drogue_bme680::*;
//!
//! fn main() -> ! {
//!   let i2c = mock::blocking_i2c();
//!   let mut timer = mock::create_timer();
//!
//!   let bme680 = Bme680Sensor::from(i2c, Address::Secondary).unwrap();
//!
//!   let mut controller = Bme680Controller::new(
//!     bme680,
//!     &mut timer,
//!     Configuration::standard(),
//!     || 25,  // fixed 25 degrees Celsius ambient temperature
//!   ).unwrap();
//!   
//!   loop {
//!     let result = controller.measure_default().unwrap();
//!     log::info!("Measured: {:?}", result);
//!   }
//! }
//! ~~~
//!
//! # Known limitations
//!
//! * Currently, the crate only provide access to the sensor using I2C.

mod control;
mod data;
mod error;
#[doc(hidden)]
pub mod mock;
mod sensor;

pub use control::*;
pub use data::*;
pub use error::*;
pub use sensor::*;

#[cfg(test)]
mod test {

    pub(crate) fn init() {
        #[cfg(feature = "env_logging")]
        let _ = env_logger::builder().is_test(true).try_init();
    }
}
