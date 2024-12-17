use serde::Serialize;
use serde_json::{Value, Error};
use std::error::Error as StdErr;
use super::device::Device;

/*
 * Delays midi inputs and outputs to display on the viewport
 */

#[derive(Serialize)]
pub struct Trigger {
    pub id: String,
    pub class: String,
}

impl Trigger {
    pub fn new(id: &str) -> Self {
        Trigger {
            id: String::from(id),
            class: String::from("trigger"),
        }
    }
}

impl Device for Trigger {
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
        _bytes: &Vec<u8>,
        _from: &str,
        _to: &str,
        _from_port: &str,
        _to_port: &str
    ) -> Vec<(String, Vec<u8>)> {
        vec![]
    }
}