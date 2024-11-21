use async_std::io::BufReader;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::sync::{Arc, Mutex};
use chat::utils::{self, ChatResult};
use chat::{Client, Server};
use crate::chats_map::ChatTracker;

pub struct Leaving(Mutex<TcpStream>);
impl