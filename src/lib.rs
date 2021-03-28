use rand::distributions::{Distribution, Uniform};
use wasm_bindgen::prelude::*;
use rand::SeedableRng;

#[cfg(test)]
mod tests {
    use crate::get_random_numbers_in_range_fast;

    #[test]
    fn it_works() {
        let rand_nums_in_range_fast = get_random_numbers_in_range_fast(-180.0, 180.0, 10);
        println!("Random numbers in range fast: \n{:?}", rand_nums_in_range_fast);

        assert_eq!(2 + 2, 4);
    }
}

#[wasm_bindgen]
pub fn get_random_lon_coordinates_fast (count: i32) -> Vec<f32> {
    let between = Uniform::from(-180.0..=180.0);
    let mut rng = rand::rngs::SmallRng::from_entropy();
    let mut results: Vec<f32> = Vec::with_capacity(count as usize);

    for _ in 0..count {
        results.push(between.sample(&mut rng));
    };

    return results;
}

#[wasm_bindgen]
pub fn get_random_lat_coordinates_fast (count: i32) -> Vec<f32> {
    let between = Uniform::from(-90.0..=90.0);
    let mut rng = rand::rngs::SmallRng::from_entropy();
    let mut results: Vec<f32> = Vec::with_capacity(count as usize);

    for _ in 0..count {
        results.push(between.sample(&mut rng));
    };

    return results;
}

#[wasm_bindgen]
pub fn get_random_lon_coordinates_better_entropy (count: i32) -> Vec<f32> {
    let between = Uniform::from(-180.0..=180.0);
    let mut rng = rand::rngs::OsRng::default();
    let mut results: Vec<f32> = Vec::with_capacity(count as usize);

    for _ in 0..count {
        results.push(between.sample(&mut rng));
    };

    return results;
}

#[wasm_bindgen]
pub fn get_random_lat_coordinates_better_entropy (count: i32) -> Vec<f32> {
    let between = Uniform::from(-180.0..=180.0);
    let mut rng = rand::rngs::OsRng::default();
    let mut results: Vec<f32> = Vec::with_capacity(count as usize);

    for _ in 0..count {
        results.push(between.sample(&mut rng));
    };

    return results;
}

#[wasm_bindgen]
pub fn get_random_numbers_in_range_fast (from: f32, until: f32, count: i32) -> Vec<f32> {
    let between = Uniform::from(from..=until);
    let mut rng = rand::rngs::SmallRng::from_entropy();
    let mut results: Vec<f32> = Vec::with_capacity(count as usize);

    for _ in 0..count {
        results.push(between.sample(&mut rng));
    };

    return results;
}

#[wasm_bindgen]
pub fn get_random_numbers_in_range_better_entropy (from: f32, until: f32, count: i32) -> Vec<f32> {
    let between = Uniform::from(from..=until);
    let mut rng = rand::rngs::SmallRng::from_entropy();
    let mut results: Vec<f32> = Vec::with_capacity(count as usize);

    for _ in 0..count {
        results.push(between.sample(&mut rng));
    };

    return results;
}