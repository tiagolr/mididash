use serde::Serialize;
use serde_json::{Value, Error};
use tokio::time::sleep;
use std::{error::Error as StdErr, time::Duration};
use crate::{app::TOKIO_RUNTIME, devices::device::Device, hub::Hub};

/*
 * Delays midi inputs and outputs to display on the viewport
 */

#[derive(Serialize)]
pub struct Delay {
    pub id: String,
    pub class: String,
    pub delay: u64,
}

impl Delay {
    pub fn new(id: &str) -> Self {
        Delay {
            id: String::from(id),
            class: String::from("delay"),
            delay: 1000,
        }
    }
}

impl Device for Delay {
    fn get_id(&self) -> &str {
        return &self.id;
    }
    fn get_class(&self) -> &str {
        return &self.class;
    }
    fn destroy(&mut self) {}
    fn get_data(&mut self, _key: String) -> Result<Option<Value>, String> { Ok(None) }
    fn set_data(&mut self, key: String, data:Value) -> Result<(), String> {
        if key == "delay" {
            self.delay = data.as_u64().unwrap();
        }
        Ok(())
    }
    fn delete_data(&mut self, _key: String) -> Result<(), String> { Ok(()) }

    fn init(&mut self) -> Result<(), Box<dyn StdErr>> {
        Ok(())
    }
    fn serialize(&self) -> Result<Value, Error> {
        serde_json::to_value(self)
    }
    fn process(
        &mut self,
        bytes: &Vec<u8>,
        _from: &str,
        _to: &str,
        _from_port: &str,
        _to_port: &str
    ) -> Vec<(String, Vec<u8>)> {
        let delay = self.delay;
        let bytes = bytes.clone();
        let id = self.id.clone();
        let runtime = TOKIO_RUNTIME.lock().unwrap();

        runtime.spawn(async move {
            sleep(Duration::from_millis(delay)).await;
            let hub_instance = Hub::get_instance();
            let mut hub = hub_instance.lock().unwrap();
            hub.process(0, &bytes, &id, "*", "*", "*");
        });

        vec![]
    }
}