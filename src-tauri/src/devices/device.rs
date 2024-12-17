use serde_json::{Value, Error};
use std::error::Error as StdErr;

pub trait Device: Send + Sync {
    fn get_id(&self) -> &str;
    fn get_class(&self) -> &str;
    fn destroy(&mut self);
    fn serialize(&self) -> Result<Value, Error>;
    fn init(&mut self) -> Result<(), Box<dyn StdErr>>;
    fn get_data(&mut self, key: String) -> Result<Option<Value>, String>;
    fn set_data(&mut self, key: String, data:Value) -> Result<(), String>;
    fn delete_data(&mut self, key: String) -> Result<(), String>;
    fn process(
        &mut self,
        bytes: &Vec<u8>,
        from: &str,
        to: &str,
        from_port: &str,
        to_port: &str
    ) -> Vec<(String, Vec<u8>)>;
}