use std::sync::Arc;

use tracing::debug;

use crate::{command_request::RequestData, *};
mod command_service;

pub trait CommandService {
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

pub struct Service<Store = MemTable> {
    inner: Arc<ServiceInner<Store>>,
}
impl<Store> Clone for Service<Store> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

pub struct ServiceInner<Store> {
    store: Store,
}
impl<Store: Storage> Service<Store> {
    pub fn new(store: Store) -> Self {
        Self {
            inner: Arc::new(ServiceInner { store }),
        }
    }
    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse {
        debug!("got request :{:?}", cmd);
        let res = dispatch(cmd, &self.inner.store);
        debug!("Executed response :{:?}", res);
        res
    }
}
pub fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
    match cmd.request_data {
        Some(RequestData::Hget(param)) => param.execute(store),
        Some(RequestData::Hset(param)) => param.execute(store),
        Some(RequestData::Hexist(param)) => param.execute(store),
        // Some(RequestData::Hdel(param)) => p
        None => KvError::InvalidCommand("request has no data".into()).into(),
        _ => KvError::Internal("not implemented".into()).into(),
    }
}
