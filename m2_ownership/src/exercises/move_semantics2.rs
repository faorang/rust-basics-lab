// Make the test pass by finding a way to keep both Vecs separate!

#[cfg(feature = "skip")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let vec0 = vec![22, 44, 66];

        let mut vec1 = fill_vec(vec0);

        assert_eq!(vec0, vec![22, 44, 66]);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }

    fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
        let mut vec = vec;

        vec.push(88);

        vec
    }
}
