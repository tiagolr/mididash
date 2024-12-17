use serde::Serialize;
use serde_json::{Value, Error};
use std::error::Error as StdErr;
use crate::devices::device::Device;

/*
 * Monitors midi inputs and outputs to display on the viewport
 */

#[derive(Serialize)]
pub struct Monitor {
    pub id: String,
    pub class: String,
}

impl Monitor {
    pub fn new(id: &str) -> Self {
        Monitor {
            id: String::from(id),
            class: String::from("monitor"),
        }
    }
}

impl Device for Monitor {
    fn get_id(&self) -> &str {
        return &self.id;
    }
    fn get_class(&self) -> &str {
        return &self.class;
    }
    fn destroy(&mut self) {}
    fn get_data(&mut self, _key: String) -> Result<Option<Value>, String> { Ok(None) }
    fn set_data(&mut self, _key: String, _data:Value) -> Result<(), String> { Ok(()) }
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
        let mut res = vec![];
        res.push(("*".to_string(), bytes.clone()));
        res
    }
}