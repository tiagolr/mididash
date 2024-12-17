use crate::{app, devices::device::Device, globals::EVT_MIDI};
use std::{io, sync::{Arc, Mutex}};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Clone)]
pub struct MidiEvent {
    ts: u64,
    from: String,
    to: String,
    from_port: String,
    to_port: String,
    bytes: Vec<u8>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Connector {
    pub id: String,
    pub from: String,
    pub to: String,
    pub from_port: String,
    pub to_port: String
}

impl Connector {
    pub fn new(from: &str, to: &str, from_port: &str, to_port: &str) -> Self {
        let id = String::from(from) + "::" + to + "::" + from_port + "::" + to_port;
        Connector {
            id,
            from: String::from(from),
            to: String::from(to),
            from_port: String::from(from_port),
            to_port: String::from(to_port)
        }
    }
}

pub struct Hub {
    devices: Vec<Box<dyn Device>>,
    connectors: Vec<Connector>,
    paused: bool,
}

// Static singleton instance of Hub using Lazy for thread-safe initialization
static INSTANCE: Lazy<Arc<Mutex<Hub>>> = Lazy::new(|| Arc::new(Mutex::new(Hub::new())));

impl Hub {
    pub fn new() -> Self {
        Hub {
            devices: Vec::new(),
            connectors: Vec::new(),
            paused: false,
        }
    }

    // Singleton access method using `Lazy`
    pub fn get_instance() -> Arc<Mutex<Hub>> {
        INSTANCE.clone() // Return the singleton instance wrapped in an Arc<Mutex<Hub>>
    }

    pub fn destroy(&mut self) {
        for device in &mut self.devices {
            device.destroy();
        }
        self.devices.clear();
        self.connectors.clear();
    }

    pub fn add_device(&mut self, device: Box<dyn Device>) -> bool {
        if self.devices.iter().any(|d| d.get_id() == device.get_id()) {
            false
        } else {
            self.devices.push(device);
            true
        }
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    /**
     * Used to reconnect inputs and outputs to their respective midi ports
     */
    pub fn reconnect_device(&mut self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(device) = self.devices.iter_mut().find(|d| d.get_id() == id) {
            device.destroy();
            device.init()?;
        } else {
            return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, format!("Device not found {}", id))))?;
        }
        Ok(())
    }

    pub fn remove_device(&mut self, id: &str) -> bool {
        if let Some(index) = self.devices.iter().position(|d| d.get_id() == id) {
            self.devices.remove(index);
            return true;
        }
        false
    }

    pub fn set_device_data(&mut self, id: String, key: String, data: Value) -> Result<(), String> {
        if let Some(device) = self.devices.iter_mut().find(|d| d.get_id() == id) {
            device.as_mut().set_data(key, data)?;
            Ok(())
        } else {
            Err(format!("Failed to find device {}", id))?
        }
    }

    pub fn get_device_data(&mut self, id: String, key: String) -> Result<Option<Value>, String> {
        if let Some(device) = self.devices.iter_mut().find(|d| d.get_id() == id) {
            Ok(device.as_mut().get_data(key)?)
        } else {
            Err(format!("Failed to find device {}", id))?
        }
    }

    pub fn serialize_device(&self, id: &str) -> Option<Value> {
        if let Some(device) = self.devices.iter().find(|d| d.get_id() == id) {
            return device.serialize().ok();
        }
        None
    }

    pub fn serialize_devices(&self) -> Result<Vec<Value>, serde_json::Error> {
        let mut res = Vec::new();
        for device in &self.devices {
            res.push(device.serialize()?);
        }
        Ok(res)
    }

    pub fn serialize_connectors(&self) -> Result<Vec<Connector>, serde_json::Error> {
        let mut res = Vec::new();
        for connector in &self.connectors {
            res.push(connector.clone());
        }
        Ok(res)
    }

    pub fn has_device(&self, id: &str) -> bool {
        self.devices.iter().any(|d| d.get_id() == id)
    }

    pub fn connect(&mut self, from: &str, to: &str, from_port: &str, to_port: &str) -> Option<Connector> {
        let connector = Connector::new(from, to, from_port, to_port);
        if self.connectors.iter().any(|d| d.id == connector.id) {
            return None;
        }
        self.connectors.push(connector.clone());
        Some(connector.clone())
    }

    pub fn disconnect(&mut self, from: &str, to: &str, from_port: &str, to_port: &str) -> bool {
        if let Some(index) = self.connectors.iter().position(|c|
            c.from == from && c.to == to &&
            c.from_port == from_port && c.to_port == to_port
        ) {
            self.connectors.remove(index);
            return true;
        }
        false
    }
    /*
        Recursively processes a midi message inside each connnected device
     */
    pub fn process(&mut self, ts: u64, bytes: &Vec<u8>, from: &str, to: &str, from_port: &str, to_port: &str) {
        if self.paused {
            return
        }
        app::emit(EVT_MIDI, MidiEvent {
            ts,
            from: from.into(),
            to: to.into(),
            from_port: from_port.into(),
            to_port: to_port.into(),
            bytes: bytes.to_vec()
        });

        let mut results = Vec::new();
        // fetch connectors from this source to destinations
        for c in self.connectors.iter().filter(|c|
            c.from == from && (to == "*" || c.to == to) && // to == "*" means to any connected device
            c.from_port == from_port && c.to_port == to_port)
        {
            // fetch the matching device to this connector destination
            if let Some(device) = self.devices.iter_mut().find(|d| d.get_id() == c.to) {
                // process the midi message inside the device,
                let processed = device.process(&bytes.clone(), from, to, from_port, to_port);
                for result in processed {
                    let target_port = result.0;
                    let payload = result.1;
                    let mut has_connectors = false;
                    // repeat the process for all connectors from the processed device
                    for c in self.connectors.iter().filter(|cc| cc.from == c.to && cc.from_port == target_port) {
                        has_connectors = true;
                        results.push((payload.clone(), c.from.clone(), c.to.clone(), c.from_port.clone(), c.to_port.clone()));
                    }
                    // if there are no connections to these device ports
                    // emit a midi event anyway just so the frontend shows port activity
                    if !has_connectors {
                        app::emit(EVT_MIDI, MidiEvent {
                            ts,
                            from: device.get_id().into(),
                            to: "".to_string(),
                            from_port: target_port.into(),
                            to_port: "".to_string(),
                            bytes: payload.clone()
                        })
                    }
                }
            } else {
                eprintln!("hub::process() target device not found {} -> {}", c.from, c.to);
            }
        }
        for (payload, from, to, from_port, to_port) in results {
            self.process(ts, &payload, &from, &to, &from_port, &to_port);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_json::Error;
    use serial_test::serial;

    #[derive(Serialize, Deserialize)]
    pub struct MockDevice {
        id: String,
        bytes: Vec<u8>,
        class: String,
    }
    impl MockDevice {
        fn new(id: &str) -> Self {
            MockDevice {
                id: String::from(id),
                bytes: vec![],
                class: String::from("mock")
            }
        }
    }
    impl Device for MockDevice {
        fn get_id(&self) -> &str {
            return &self.id;
        }
        fn get_class(&self) -> &str {
            return &self.class;
        }
        fn process(&mut self, bytes: &Vec<u8>, _from: &str, _to: &str, _from_port: &str, _to_port: &str) -> Vec<(String, Vec<u8>)> {
            let mut result = vec![];
            let mut b = bytes.clone();
            b[0] = bytes[0] + 1;
            result.push(("*".to_string(), b.clone()));
            self.bytes = b;
            result
        }
        fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }
        fn destroy(&mut self) {}
        fn serialize(&self) -> Result<Value, Error> {
            serde_json::to_value(self)
        }
        fn get_data(&mut self, _key: String) -> Result<Option<Value>, String> { Ok(None) }
        fn set_data(&mut self, _key: String, _data:Value) -> Result<(), String> { Ok(()) }
        fn delete_data(&mut self, _key: String) -> Result<(), String> { Ok(()) }
    }

    #[test]
    #[serial]
    fn add_device() {
        let hub_instance = Hub::get_instance();
        let mut hub = hub_instance.lock().unwrap_or_else(|e| e.into_inner());
        hub.destroy();
        let d1: Box<dyn Device> = Box::new(MockDevice::new("1"));
        let d2: Box<dyn Device> = Box::new(MockDevice::new("1"));
        assert!(hub.add_device(d1));
        assert!(!hub.add_device(d2));
        assert!(hub.devices.len() == 1);
    }
   #[test]
   #[serial]
    fn remove_device() {
        let hub_instance = Hub::get_instance();
        let mut hub = hub_instance.lock().unwrap_or_else(|e| e.into_inner());
        hub.destroy();
        let d1: Box<dyn Device> = Box::new(MockDevice::new("1"));
        let d2: Box<dyn Device> = Box::new(MockDevice::new("2"));
        assert!(hub.add_device(d1));
        assert!(hub.add_device(d2));
        assert!(hub.remove_device("1"));
        assert!(!hub.remove_device("1"));
        assert!(hub.devices.len() == 1);
    }
    #[test]
    #[serial]
    fn connect() {
        let hub_instance = Hub::get_instance();
        let mut hub = hub_instance.lock().unwrap_or_else(|e| e.into_inner());
        hub.destroy();
        assert!(hub.connect("*", "*", "*", "*").is_some());
        assert!(hub.connect("*", "*", "*", "*").is_none());
    }
    #[test]
    #[serial]
    fn disconnect() {
        let hub_instance = Hub::get_instance();
        let mut hub = hub_instance.lock().unwrap_or_else(|e| e.into_inner());
        hub.destroy();
        hub.connect("*", "*", "*", "*");
        assert!(!hub.disconnect("*", "*", "*", "1"));
        assert!(hub.disconnect("*", "*", "*", "*"));
        assert!(!hub.disconnect("*", "*", "*", "*"));
    }
    #[test]
    #[serial]
    fn process() {
        let hub_instance = Hub::get_instance();
        let mut hub = hub_instance.lock().unwrap();
        hub.destroy();
        let d1: Box<dyn Device> = Box::new(MockDevice::new("1"));
        let d2: Box<dyn Device> = Box::new(MockDevice::new("2"));
        let d3: Box<dyn Device> = Box::new(MockDevice::new("3"));
        hub.add_device(d1);
        hub.add_device(d2);
        hub.add_device(d3);
        let bytes = vec![100, 100, 100];
        hub.connect("*", "1", "*", "*"); // connect non existing device to 1
        hub.connect("1", "2", "*", "*"); // connect 1 to 2
        hub.connect("2", "3", "*", "*"); // connect 2 to 3
        hub.process(0, &bytes, "*", "1", "*", "*");
        let get_device = |value: Option<Value>| serde_json::from_value(value.unwrap()).expect("Invalid JSON");
        let b1: MockDevice = get_device(hub.serialize_device("1"));
        let b2: MockDevice = get_device(hub.serialize_device("2"));
        let b3: MockDevice = get_device(hub.serialize_device("3"));
        assert_eq!(b1.bytes[0], 101);
        assert_eq!(b2.bytes[0], 102);
        assert_eq!(b3.bytes[0], 103);
        hub.process(0, &b3.bytes, "2", "3", "*", "*");
        let bb3: MockDevice = get_device(hub.serialize_device("3"));
        assert_eq!(bb3.bytes[0], 104);
    }
}