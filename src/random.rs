

pub fn get_value_adv_seed(range: i32, seed: Option<&i32>) -> u32 { unsafe { get_random_number(range, seed) } }
pub fn get_value() -> u32 { unsafe { get_random_value() } }


#[skyline::from_offset(0x5bb630)]
fn get_random_number(range: i32, seed: Option<&i32>) -> u32;

#[skyline::from_offset(0x005bb600)]
fn get_random_value() -> u32;