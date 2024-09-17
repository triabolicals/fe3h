

pub fn get_value_adv_seed(range: i32, seed: Option<&i32>) -> u32 { unsafe { get_random_number(range, seed) } }

#[skyline::from_offset(0x5bb630)]
fn get_random_number(range: i32, seed: Option<&i32>) -> u32;