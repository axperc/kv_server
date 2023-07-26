mod memory;
use crate::{KvError, Kvpair, Value};
pub use memory::MemTable;
pub trait Storage {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;
    // 返回hashtable 的迭代器
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}

#[cfg(test)]
mod tests {

    use std::vec;

    use super::*;
    #[test]
    fn memtable_basic_interface_should_work() {
        let store = MemTable::new();
        test_basic_interface(store);
    }

    #[test]
    fn memtable_get_all_should_work() {
        let store = MemTable::new();
        test_get_all(store);
    }

    fn test_basic_interface(store: impl Storage) {
        // 第一次 set 会创建table 插入key 并返回 None
        let v = store.set("t1", "hello".into(), "world".into());
        assert!(v.unwrap().is_none());
        // 再次set 同样的key  会更新  并返回之前的值
        let v1: Result<Option<Value>, KvError> = store.set("t1", "hello".into(), "world1".into());
        assert_eq!(v1, Ok(Some("world".into())));
        //get 存在的值会获得最新的值
        let v = store.get("t1", "hello".into());
        assert_eq!(v, Ok(Some("world1".into())));
        // get 不存在的会获得None
        assert_eq!(Ok(None), store.get("t1", "hello world"));
        assert!(store.get("t2", "hello111").unwrap().is_none());
        //contains 不存在的key 返回true 否则 false
        assert_eq!(store.contains("t1", "hello"), Ok(true));
        assert_eq!(store.contains("t1", "hello1"), Ok(false));
        assert_eq!(store.contains("t2", "hello"), Ok(false));
        let v = store.del("t1", "hello");
        assert_eq!(v, Ok(Some("world1".into())));
        // del 不存在的
        assert_eq!(Ok(None), store.del("t1", "hello1"));
        assert_eq!(Ok(None), store.del("t2", "hello"));
        test_hdel(store);
    }

    fn test_get_all(store: impl Storage) {
        store.set("t2", "k1".into(), "v1".into()).unwrap();
        store.set("t2", "k2".into(), "v2".into()).unwrap();
        let mut data = store.get_all("t2").unwrap();
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            data,
            vec![
                Kvpair::new("k1", "v1".into()),
                Kvpair::new("k2", "v2".into())
            ]
        )
    }
    fn test_hdel(store: impl Storage) {
        store.set("texits", "t1".into(), "hello".into()).unwrap();
        store.set("texits", "t2".into(), "hello".into()).unwrap();
        if let Err(e) = store.del("texits", "t1") {
            panic!("del error :{}", e);
        }
        assert_eq!(store.contains("texits", "t1"), Ok(false));
    }
}
