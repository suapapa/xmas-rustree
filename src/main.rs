#![no_std]
#![no_main]

mod fmt;
mod random;
use random::RNG;
mod star;
use star::Star;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
// use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::spi;
use embassy_time::{Duration, Timer};
// use fmt::info;
use smart_leds::{gamma, hsv::hsv2rgb, SmartLedsWrite, RGB8};

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
    let mut start = [Star::default(); LED_NUM];
    let mut data = [RGB8::default(); LED_NUM];
    let mut rng = RNG::new();
    loop {
        for i in 0..LED_NUM {
            if !start[i].is_alive {
                start[i] = make_star(&mut rng);
            }
            start[i].update();
            data[i] = hsv2rgb(start[i].get_hsv());
        }
        // before writing, apply gamma correction for nicer rainbow
        dotstar.write(gamma(data.iter().cloned())).unwrap();

        Timer::after(Duration::from_millis(10)).await;
    }
}

fn make_star(rng: &mut RNG) -> Star {
    let color = rng.next_u8();
    let health = 100;
    let age = Duration::from_secs((rng.next_u8() % 8 + 3) as u64);
    Star::new(color, health, age)
}
