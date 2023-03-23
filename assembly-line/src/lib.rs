pub fn production_rate_per_hour(speed: u8) -> f64 {

    let speed_as_float = speed as f64;
    let success_rate = match speed {
        0=>0.0,
        1..=4=>1.0,
        5..=8=>0.9,
        9|10 =>0.77,
        11.. =>0.0,
    };

    speed_as_float * 221.0 * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {

    let production_rate_per_hour_as_int = production_rate_per_hour(speed) as u32;

    production_rate_per_hour_as_int / 60
}
