#![no_std]
#![no_main]

mod fmt;
mod random;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::spi;
use embassy_time::{Duration, Timer};
use fmt::info;
use smart_leds::{gamma, hsv::hsv2rgb, hsv::Hsv, SmartLedsWrite, RGB8};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    // let mut led = Output::new(p.PA5, Level::High, Speed::Low);

    let spi1 = p.SPI1;
    let spi1_sck = p.PA5;
    let spi1_mosi = p.PA7;
    let spi = spi::Spi::new_txonly(
        spi1,
        spi1_sck,
        spi1_mosi,
        p.DMA1_CH0,
        p.DMA1_CH1,
        spi::Config::default(),
    );

    let mut dotstar = apa102_spi::Apa102::new(spi);

    const LED_NUM: usize = 100;
    let mut data = [RGB8::default(); LED_NUM];

    let mut rng = random::RNG::new();

    loop {
        // for j in 0..256 {
        for i in 0..LED_NUM {
            let rn = rng.next_u8();
            info!("Random number: {}", rn);

            // rainbow cycle using HSV, where hue goes through all colors in circle
            // value sets the brightness
            let hsv = Hsv {
                // hue: ((i * 3 + j) % 256) as u8,
                hue: rn,
                sat: 255,
                val: 100,
            };

            data[i] = hsv2rgb(hsv);
        }
        // before writing, apply gamma correction for nicer rainbow
        dotstar.write(gamma(data.iter().cloned())).unwrap();

        // delay.delay_ms(10u8);
        Timer::after(Duration::from_millis(1000)).await;
    }
    // }

    // loop {
    //     info!("Hello, World!");
    //     led.set_high();
    //     Timer::after(Duration::from_millis(500)).await;
    //     led.set_low();
    //     Timer::after(Duration::from_millis(500)).await;
    // }
}
