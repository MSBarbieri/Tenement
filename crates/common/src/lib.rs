#[cfg(feature = "db")]
pub mod db;

#[cfg(feature = "k8s")]
pub mod k8s;

pub mod models;

#[inline(always)]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
