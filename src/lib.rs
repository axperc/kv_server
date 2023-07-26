mod errors;
pub use errors::KvError;

mod pb;
mod storage;
pub use pb::abi::*;
pub use storage::*;
mod service;
pub use service::*;

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
