#![no_std]
#![no_main]

mod fmt;

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

    let spi2 = p.SPI2;
    let spi2_sck = p.PB13;
    let spi2_mosi = p.PB15;
    let spi2_config = spi::Config::default();
    let spi2_txdma = p.DMA1_CH0;
    let spi2_rxdma = p.DMA1_CH1;

    let spi = spi::Spi::new_txonly(
        spi2,
        spi2_sck,
        spi2_mosi,
        spi2_txdma,
        spi2_rxdma,
        spi2_config,
    );

    let mut dotstar = apa102_spi::Apa102::new(spi);

    const LED_NUM: usize = 1; // 60
    let mut data = [RGB8::default(); LED_NUM];

    loop {
        for j in 0..256 {
            for i in 0..LED_NUM {
                // rainbow cycle using HSV, where hue goes through all colors in circle
                // value sets the brightness
                let hsv = Hsv {
                    hue: ((i * 3 + j) % 256) as u8,
                    sat: 255,
                    val: 100,
                };

                data[i] = hsv2rgb(hsv);
            }
            // before writing, apply gamma correction for nicer rainbow
            dotstar.write(gamma(data.iter().cloned())).unwrap();

            // delay.delay_ms(10u8);
            Timer::after(Duration::from_millis(10)).await;
        }
    }

    // loop {
    //     info!("Hello, World!");
    //     led.set_high();
    //     Timer::after(Duration::from_millis(500)).await;
    //     led.set_low();
    //     Timer::after(Duration::from_millis(500)).await;
    // }
}
