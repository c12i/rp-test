use num::Num;

pub fn add<T: Num>(left: T, right: T) -> T {
    left + right
}

pub fn multiply<T: Num>(left: T, right: T) -> T {
    left * right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

        let result = add(2.0, 2.5);
        assert_eq!(result, 4.5);
    }

    #[test]
    fn multiply_works() {
        let result = multiply(2, 3);
        assert_eq!(result, 6);

        let result = multiply(2.0, 2.5);
        assert_eq!(result, 5.0);
    }
}
