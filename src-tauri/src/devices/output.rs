use midir::{MidiOutput, MidiOutputConnection};
use serde::Serialize;
use serde_json::{Value, Error};
use std::error::Error as StdErr;
use std::sync::Mutex;

use crate::devices::device::Device;
use crate::globals::{PREFIX_OUTPUT, SUFFIX_SEPARATOR, split_device_id};

/*
 * Output for midi hardware
 */

#[derive(Serialize)]
pub struct Output {
    pub id: String,
    pub class: String,
    #[serde(skip_serializing)]
    conn: Option<Mutex<MidiOutputConnection>>,
}

impl Output {
    pub fn new(id: &str) -> Self {
        let mut preid =
            if id.starts_with(PREFIX_OUTPUT) { id.to_string() }
            else { format!("{}{}", PREFIX_OUTPUT, id) };

        let has_suffix = preid.rsplit_once(SUFFIX_SEPARATOR)
            .and_then(|(_, n)| n.parse::<usize>().ok())
            .is_some();

        if !has_suffix {
            preid.push_str(SUFFIX_SEPARATOR);
            preid.push_str("1");
        }

        Output {
            id: String::from(preid),
            class: String::from("output"),
            conn: None,
        }
    }
}

impl Device for Output {
    fn init(&mut self) -> Result<(), Box<dyn StdErr>> {
        let output = MidiOutput::new(&self.id)?;
        let ports = &output.ports();
        let mut found = false;
        let mut idx = 0;
        let mut count = 0;
        let raw_id = self.id.strip_prefix(PREFIX_OUTPUT).unwrap();
        let (target_name, target_instance) = split_device_id(raw_id);

        for (i, p) in ports.iter().enumerate() {
            let name = output.port_name(p)?;
            if name == target_name {
                count += 1;
                if count == target_instance {
                    found = true;
                    idx = i;
                    break;
                }
            }
        }

        if !found {
            let str = String::from("Device port not found ") + &self.id;
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, str)));
        }

        let port = &ports[idx];
        let id = self.id.clone();
        self.conn = Some(Mutex::new(output.connect(port, &id)?));
        Ok(())
    }
    fn get_id(&self) -> &str {
        return &self.id;
    }
    fn get_class(&self) -> &str {
        return &self.class;
    }
    fn get_data(&mut self, _key: String) -> Result<Option<Value>, String> { Ok(None) }
    fn set_data(&mut self, _key: String, _data:Value) -> Result<(), String> { Ok(()) }
    fn delete_data(&mut self, _key: String) -> Result<(), String> { Ok(()) }
    fn destroy(&mut self) {
        if let Some(mutex) = self.conn.take() {
            let conn = mutex.into_inner().unwrap();
            conn.close();
        }
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
        if let Some(ref mutex) = self.conn {
            let mut conn = mutex.lock().unwrap();
            if let Err(e) = conn.send(bytes) { // send message to midi channel this output is connected to
                eprintln!("Error sending bytes from output {} {}", self.id, e);
            }
        }
        vec![] // ignore forward processing
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use midir::MidiOutput;
    use serial_test::serial;

    #[test]
    #[serial]
    fn create_destroy() {
        let mut input = Output::new("Test1234");
        input.destroy();
    }

    #[test]
    #[serial]
    fn init() {
        let inp = MidiOutput::new("Test").expect("Failed to create input");
        let ports = inp.ports();
        if ports.len() > 0 {
            let pname = inp.port_name(&ports[0]).expect("Get port name error");
            let mut input = Output::new(&pname);
            input.init().expect("Failed to connect");
            input.destroy();
        }
    }
}