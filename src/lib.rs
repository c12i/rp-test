//! # rp-test

pub fn add(left: f64, right: f64) -> f64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2.0, 2.0);
        assert_eq!(result, 4.0);
    }
}
