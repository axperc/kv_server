pub mod abi;

use std::vec;

use abi::*;
use hyper::StatusCode;

use crate::KvError;

impl CommandRequest {
    pub fn new_hget(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(command_request::RequestData::Hget(Hget {
                table: table.into(),
                key: key.into(),
            })),
        }
    }
    pub fn new_hgetall(table: impl Into<String>) -> Self {
        Self {
            request_data: Some(command_request::RequestData::Hgetall(Hgetall {
                table: table.into(),
            })),
        }
    }
    /// 创建 HSET 命令    
    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        Self {
            request_data: Some(command_request::RequestData::Hset(Hset {
                table: table.into(),
                pair: Some(Kvpair::new(key, value)),
            })),
        }
    }
    pub fn new_hexist(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(command_request::RequestData::Hexist(Hexist {
                table: table.into(),
                key: key.into(),
            })),
        }
    }
    pub fn new_hdel(table: impl Into<String>, keys: Vec<String>) -> Self {
        Self {
            request_data: Some(command_request::RequestData::Hdel(Hdel {
                table: table.into(),
                keys,
            })),
        }
    }
}
impl Kvpair {
    pub fn new<K>(key: K, val: Value) -> Self
    where
        K: Into<String>,
    {
        Self {
            key: key.into(),
            value: Some(val),
        }
    }
}
impl From<String> for Value {
    fn from(s: String) -> Self {
        Self {
            value: Some(value::Value::String(s)),
        }
    }
}
impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self {
            value: Some(value::Value::String(s.into())),
        }
    }
}
impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self {
            value: Some(value::Value::Integer(value)),
        }
    }
}
impl From<Value> for CommandResponse {
    fn from(value: Value) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: vec![value],
            ..Default::default()
        }
    }
}
impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self {
            value: Some(value::Value::Bool(value)),
        }
    }
}
impl From<Vec<Kvpair>> for CommandResponse {
    fn from(value: Vec<Kvpair>) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            pairs: value,
            ..Default::default()
        }
    }
}
impl From<KvError> for CommandResponse {
    fn from(value: KvError) -> Self {
        let mut result = Self {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as _,
            message: value.to_string(),
            values: vec![],
            pairs: vec![],
        };
        match value {
            KvError::NotFound(_, _) => result.status = StatusCode::NOT_FOUND.as_u16() as _,
            KvError::InvalidCommand(_) => result.status = StatusCode::BAD_REQUEST.as_u16() as _,
            _ => {}
        }
        result
    }
}
