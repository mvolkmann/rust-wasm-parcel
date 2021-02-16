//use wasm_bindgen::prelude::*;

#[no_mangle]
pub fn add(n1: f64, n2: f64) -> f64 {
    n1 + n2
}

#[no_mangle]
pub fn sum_f64s(numbers: &[f64]) -> f64 {
    println!("sum_f64s: numbers = {:?}", numbers);
    let mut sum = 0.0;
    for n in numbers {
        sum += n;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        assert_eq!(add(5, 4), 9);
    }
}
