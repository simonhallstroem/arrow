use actaeon::{config::Config, Center, Interface, ToAddress, Topic};
use core::fmt;
use sodiumoxide::crypto::box_;
use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
    sync::Arc,
};

use crate::lisptype::LispType;

/// Wrapper around Actaeon.
#[derive(Clone)]
pub struct Actaeon {
    pub center: Arc<Center>,
    pub interface: Arc<Interface>,
    pub topic: Arc<RefCell<Topic>>,
}

impl Actaeon {
    /// Connect to an actaeon network. This function is used internally
    /// and everything is handled by the library.
    pub fn new(center: &str, remote: &str, port: usize, topic: &str) -> LispType {
        let (_, secret) = box_::gen_keypair();
        let config = Config::new(20, 1, 100, remote.to_string(), port);
        let center = Center::new(secret, center.to_string(), port);
        let interface = Interface::new(config, center.clone()).unwrap();

        let topic = interface.subscribe(&topic.to_string().to_address());

        LispType::Actaeon(Self {
            center: Arc::new(center),
            interface: Arc::new(interface),
            topic: Arc::new(RefCell::new(topic)),
        })
    }

    /// Receive data from actaeon.
    pub fn receive(&mut self) -> LispType {
        let mut topic = self.topic.borrow_mut();
        let msg = topic.try_recv();

        if let Some(m) = msg {
            LispType::String(String::from_utf8_lossy(&m.message.body.as_bytes()).to_string())
        } else {
            LispType::Bool(false)
        }
    }

    /// Send data to actaeon.
    pub fn send(&mut self, send: &str) -> LispType {
        let mut topic = self.topic.borrow_mut();

        let _ = topic.broadcast(send.as_bytes().to_vec());

        LispType::Bool(true)
    }
}

impl Debug for Actaeon {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}
