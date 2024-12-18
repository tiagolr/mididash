use midir::{Ignore, MidiInput, MidiInputConnection};
use serde::Serialize;
use serde_json::{Value, Error};
use std::error::Error as StdErr;
use std::sync::Mutex;

use crate::hub::Hub;
use crate::devices::device::Device;

use crate::globals::PREFIX_INPUT;

/*
 * Input for midi hardware
 */

#[derive(Serialize)]
pub struct Input {
    pub id: String,
    pub class: String,
    #[serde(skip_serializing)]
    conn: Option<Mutex<MidiInputConnection<()>>>,
}

impl Input {
    pub fn new(id: &str) -> Self {
        let preid = if id.starts_with(PREFIX_INPUT) { id } else { &format!("{}{}", PREFIX_INPUT, id) };
        Input {
            id: String::from(preid),
            class: String::from("input"),
            conn: None,
        }
    }
}

impl Device for Input {
    fn get_id(&self) -> &str {
        return &self.id;
    }
    fn get_class(&self) -> &str {
        return &self.class;
    }
    fn destroy(&mut self) {
        if let Some(mutex) = self.conn.take() {
            let conn = mutex.into_inner().unwrap();
            conn.close();
        }
    }
    fn get_data(&mut self, _key: String) -> Result<Option<Value>, String> { Ok(None) }
    fn set_data(&mut self, _key: String, _data:Value) -> Result<(), String> { Ok(()) }
    fn delete_data(&mut self, _key: String) -> Result<(), String> { Ok(()) }

    fn init(&mut self) -> Result<(), Box<dyn StdErr>> {
        let mut input = MidiInput::new(&self.id)?;
        input.ignore(Ignore::None);
        let ports = &input.ports();
        let mut found = false;
        let mut idx = 0;
        for (i, p) in ports.iter().enumerate() {
            if &input.port_name(p)? == &self.id.strip_prefix(PREFIX_INPUT).unwrap() {
                found = true;
                idx = i;
                break;
            }
        }
        if !found {
            // let str = String::from("Device port not found ") + &self.id;
            if ports.len() > 0 {
                let str = format!("WOW {}", &input.port_name(&ports[0])?);
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, str)))?;
            } else {
                let str = format!("000 {}", ports.len());
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, str)))?;
            }
        }
        let port = &ports[idx];
        let id = self.id.clone();
        self.conn = Some(Mutex::new(input.connect(
            &port,
            &id.clone(),
            move |ts, bytes, _| {
                let hub_instance = Hub::get_instance();
                let mut hub = hub_instance.lock().unwrap();
                hub.process(ts, &bytes.to_vec(), &id, "*", "*", "*");
            },
            ()
        )?));

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

#[cfg(test)]
mod tests {
    use super::*;
    use midir::MidiInput;
    use serial_test::serial;

    #[test]
    #[serial]
    fn create_destroy() {
        let mut input = Input::new("Test1234");
        input.destroy();
    }

    #[test]
    #[serial]
    fn init() {
        let inp = MidiInput::new("Test").expect("Failed to create input");
        let ports = inp.ports();
        if ports.len() > 0 {
            let pname = inp.port_name(&ports[0]).expect("Get port name error");
            let mut input = Input::new(&pname);
            input.init().expect("Failed to connect");
            input.destroy();
        }
    }
}