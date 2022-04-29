// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const NUMBER_OF_PER_HOUR:u32 = 221;
pub fn production_rate_per_hour(speed: u8) -> f64 {
    // unimplemented!("calculate hourly production rate at speed: {}", speed)
    match speed {
      1..=4 => speed as f64 * NUMBER_OF_PER_HOUR as f64 * 1.0,
      5..=8 => speed as f64 * NUMBER_OF_PER_HOUR as f64 * 0.9,
      9..=10 => speed as f64 * NUMBER_OF_PER_HOUR as f64 * 0.77,
      _ => 0.0
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    // unimplemented!("calculate the amount of working items at speed: {}", speed)
    production_rate_per_hour(speed) as u32 / 60
}
