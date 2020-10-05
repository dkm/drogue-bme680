#![deny(unsafe_code)]
#![no_main]
#![no_std]

/// Example for using the BME680 sensor on an STM32F723 DISCOVERY board

#[cfg(not(feature = "env_logging"))]
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[cfg(feature = "stm32f7xx")]
use cortex_m_rt::entry;
#[cfg(feature = "stm32f7xx")]
use stm32f7 as _;
#[cfg(feature = "stm32f7xx")]
use stm32f7xx_hal as hal;
#[cfg(feature = "stm32f7xx")]
use stm32f7xx_hal::{
    delay::Delay,
    device,
    i2c::{BlockingI2c, Mode},
    prelude::*,
    timer::Timer,
};

use drogue_bme680::{Address, Bme680Controller, Bme680Sensor, Configuration};
use drogue_embedded_timer::embedded_countdown;

use log::LevelFilter;
use rtt_logger::RTTLogger;

use embedded_hal::timer::CountDown;
use embedded_time::duration::{Duration, Milliseconds};

static LOGGER: RTTLogger = RTTLogger::new(LevelFilter::Info);

#[entry]
fn main() -> ! {
    rtt_init_print!(NoBlockSkip, 4096);
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
    log::info!("Starting up...");

    let p = device::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();
    let core = device::CorePeripherals::take().unwrap();
    let mut delay = Delay::new(core.SYST, clocks);

    // let gpioa = p.GPIOA.split();
    // let gpiob = p.GPIOB.split();
    // let gpioc = p.GPIOC.split();
    // let gpiod = p.GPIOD.split();
    // let gpiog = p.GPIOG.split();
    let gpioh = p.GPIOH.split();
    // let gpioi = p.GPIOI.split();

    // timer

    embedded_countdown!(MsToHertzCountDown,
                embedded_time::duration::Milliseconds,
                hal::time::Hertz
                 => (ms) {
                        let hz: embedded_time::rate::Hertz = ms.to_rate().unwrap();
                        hal::time::Hertz(hz.0)
                } );

    let hal_hz_timer = Timer::tim14(p.TIM14, 1.hz(), clocks, &mut rcc.apb1);
    let mut timer = MsToHertzCountDown::from(hal_hz_timer);

    // init

    let sda = gpioh.ph5.into_alternate_af4();
    let scl = gpioh.ph4.into_alternate_af4();

    // Initialize I2C

    let i2c = BlockingI2c::i2c2(
        p.I2C2,
        (scl, sda),
        Mode::standard(100_000.hz()),
        clocks,
        &mut rcc.apb1,
        100,
    );

    let bme680 = Bme680Sensor::from(i2c, Address::Secondary).unwrap();

    let mut controller =
        Bme680Controller::new(bme680, &mut timer, Configuration::standard(), || 25).unwrap();

    let mut cnt = 0;

    loop {
        // log::info!("{:?}", controller.state().unwrap());
        // log::info!("{:?}", controller.read_data().unwrap());

        if cnt == 0 {
            log::info!(
                "Measured: {:?}",
                controller.measure(5, Milliseconds(50)).unwrap()
            );
        }
        cnt += 1;
        if cnt > 30 {
            cnt = 0;
        }

        // delay(&mut timer, Milliseconds(1_000));
        delay.delay_ms(1_000u16);
    }
}