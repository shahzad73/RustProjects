pub fn multiply(left: usize, right: usize) -> usize {
    left * right
}

pub fn divide(left: usize, right: usize) -> usize {
    left / right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = multiply(2, 2);
        assert_eq!(result, 4);
    }
}
