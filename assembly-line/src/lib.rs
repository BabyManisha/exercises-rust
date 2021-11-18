// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    //unimplemented!("calculate hourly production rate at speed: {}", speed)
  //  if speed == 9 || speed == 10 {
  //   ((speed as f64 * 221.0 / 100.0) * 77.0) as f64
  // }else if speed < 9 && speed > 4 {
  //   ((speed as f64 * 221.0 / 100.0) * 90.0) as f64
  // }else{
  //   (speed as usize * 221) as f64
  // }
    221.0 * (speed as f64) * match speed {
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => 0.0,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
   // unimplemented!("calculate the amount of working items at speed: {}", speed)
   (production_rate_per_hour(speed) / 60.0) as u32
}
