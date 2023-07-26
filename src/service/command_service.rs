use crate::{CommandService, Hdel, Hexist, Hget, Hgetall, Hset, KvError, Value};

impl CommandService for Hget {
    fn execute(self, store: &impl crate::Storage) -> crate::CommandResponse {
        match store.get(&self.table, &self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => KvError::NotFound(self.table, self.key).into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hgetall {
    fn execute(self, store: &impl crate::Storage) -> crate::CommandResponse {
        match store.get_all(&self.table) {
            Ok(v) => v.into(),
            Err(e) => e.into(),
        }
    }
}
impl CommandService for Hset {
    fn execute(self, store: &impl crate::Storage) -> crate::CommandResponse {
        match self.pair {
            Some(v) => match store.set(&self.table, v.key, v.value.unwrap_or_default()) {
                Ok(Some(v)) => v.into(),
                Err(e) => e.into(),
                Ok(None) => Value::default().into(),
            },
            None => Value::default().into(),
        }
    }
}
impl CommandService for Hdel {
    fn execute(self, store: &impl crate::Storage) -> crate::CommandResponse {
        // 遍历keys
        if self.keys.is_empty() {
            return KvError::InvalidCommand("keys is empty".into()).into();
        }
        match store.del(&self.table, &self.keys[0]) {
            Ok(Some(v)) => v.into(),
            Ok(None) => KvError::NotFound(self.table, self.keys[0].clone()).into(),
            Err(e) => e.into(),
        }
    }
}
impl CommandService for Hexist {
    fn execute(self, store: &impl crate::Storage) -> crate::CommandResponse {
        match store.contains(&self.table, &self.key) {
            Ok(v) => <bool as Into<crate::Value>>::into(v).into(),
            Err(e) => e.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::{
        command_request::RequestData, CommandRequest, CommandResponse, Kvpair, MemTable, Storage,
    };
    #[test]
    fn hset_should_work() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hset("t1", "hello", "word".into());
        let res = dispatch(cmd.clone(), &store);
        assert_res_ok(res, &[Value::default()], &[]);
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &["word".into()], &[]);
    }
    #[test]
    fn hget_should_work() {
        let store = MemTable::new();

        let cmd = CommandRequest::new_hset("t1", "hello", "i am your father ".into());
        dispatch(cmd, &store);

        let cmd = CommandRequest::new_hget("t1", "hello");
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &["i am your father ".into()], &[]);
    }
    fn dispatch(cmd: CommandRequest, storage: &impl Storage) -> CommandResponse {
        match cmd.request_data.unwrap() {
            RequestData::Hget(v) => v.execute(storage),
            RequestData::Hgetall(v) => v.execute(storage),
            RequestData::Hset(v) => v.execute(storage),
            _ => todo!(),
        }
    }
    #[test]
    fn hget_with_non_exist_key_should_return_404() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hget("score", "u1");
        let res = dispatch(cmd, &store);
        assert_res_error(res, 404, "Not Found");
    }
    #[test]
    fn hget_all_should_work() {
        let store = MemTable::new();
        let cmds = vec![
            CommandRequest::new_hset("socre", "u1", 10.into()),
            CommandRequest::new_hset("score", "u2", 8.into()),
            CommandRequest::new_hset("score", "u3", 9.into()),
            CommandRequest::new_hset("score", "u1", 11.into()),
        ];
        for cmd in cmds {
            dispatch(cmd, &store);
        }
        let cmd = CommandRequest::new_hgetall("score");
        let res = dispatch(cmd, &store);
        let pairs = &[
            Kvpair::new("u1", 11.into()),
            Kvpair::new("u2", 8.into()),
            Kvpair::new("u3", 9.into()),
        ];
        assert_res_ok(res, &[], pairs)
    }
    #[cfg(test)]
    fn assert_res_ok(mut res: CommandResponse, values: &[Value], pair: &[Kvpair]) {
        res.pairs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(res.status, 200);
        assert_eq!(res.message, "");
        assert_eq!(res.values, values);
        assert_eq!(res.pairs, pair);
    }
    #[cfg(test)]
    fn assert_res_error(res: CommandResponse, code: u32, msg: &str) {
        assert_eq!(res.status, code);
        assert!(res.message.contains(msg));
        assert_eq!(res.values, &[]);
        assert_eq!(res.pairs, &[]);
    }
}
