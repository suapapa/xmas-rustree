// use crate::random::RNG;

use embassy_time::{Duration, Instant};
use smart_leds::hsv::Hsv;
// use static_cell::StaticCell;

// static RNG: StaticCell<RNG> = StaticCell::new();

#[derive(Clone, Copy)]
pub struct Star {
    color: u8,  // hue
    health: u8, // val (brightness)
    max_health: u8,
    age: Duration,
    birth: Instant,
    pub is_alive: bool,
}

impl Star {
    pub fn new(color: u8, health: u8, age: Duration) -> Self {
        Self {
            color,
            health,
            max_health: health,
            age,
            birth: Instant::now(),
            is_alive: true,
        }
    }

    pub fn update(&mut self) {
        let age = Instant::now() - self.birth;
        if age > self.age {
            self.is_alive = false;
            return;
        }

        self.health = self.max_health
            - (age.as_micros() as f32 / self.age.as_micros() as f32 * self.max_health as f32) as u8;
    }

    pub fn get_hsv(&self) -> Hsv {
        Hsv {
            hue: self.color,
            sat: 255,
            val: self.health,
        }
    }
}

impl Default for Star {
    fn default() -> Self {
        Self::new(0, 100, Duration::from_secs(1))
    }
}
