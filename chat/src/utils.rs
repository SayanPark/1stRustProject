use async_std::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error;
use std::marker::Unpin;

pub type ChatError = Box<dyn Error + Send + Sync + 'static>;
pub type ChatResult<T> = Result<T, ChatError>;

pub async fn send_json<O, P>(leaving: &mut 0, packet: &P) -> ChatResult<()>
where
    O: async_std::io::Write + Unpin,