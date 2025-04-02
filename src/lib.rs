use num::Num;

pub fn add<T: Num>(left: T, right: T) -> T {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

        let result = add(2.0, 2.5);
        assert_eq!(result, 4.5);
    }
}
