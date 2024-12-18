use std::sync::{Arc, Mutex};

use midir::{os::unix::{VirtualInput, VirtualOutput}, Ignore, MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};
use serde::Serialize;
use serde_json::{Error, Value};
use crate::hub::Hub;
use regex::Regex;

use super::device::Device;
use crate::globals::PREFIX_I;
use crate::globals::PREFIX_O;
use crate::globals::PREFIX_VI;
use crate::globals::PREFIX_VO;

/**
 * Virtual channel with midi through and listening ports
 * Output -> VirtualIn -> VirtualOut -> Input -> Hub
 */

#[derive(Serialize)]
pub struct VirtualC {
    pub id: String,
    pub in_id: String,
    pub out_id: String,
    pub class: String,
    #[serde(skip_serializing)]
    iconn: Option<Mutex<MidiInputConnection<()>>>,
    #[serde(skip_serializing)]
    viconn: Option<Mutex<MidiInputConnection<()>>>,
    #[serde(skip_serializing)]
    oconn: Option<Mutex<MidiOutputConnection>>,
    #[serde(skip_serializing)]
    voconn: Option<Arc<Mutex<MidiOutputConnection>>>,
}

impl VirtualC {
    pub fn new(id: &str) -> Self {
        VirtualC {
            id: String::from(id),
            in_id: String::from(PREFIX_I) + id,
            out_id: String::from(PREFIX_O) + id,
            class: String::from("virtual"),
            iconn: None,
            viconn: None,
            oconn: None,
            voconn: None,
        }
    }
}

impl Device for VirtualC {
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut input = MidiInput::new(&self.in_id)?;
        input.ignore(Ignore::None);
        let output = MidiOutput::new(&self.out_id)?;

        // Virtual Output
        let voutput = MidiOutput::new(PREFIX_VO)?;
        let mvoconn = Arc::new(Mutex::new(voutput.create_virtual(&self.id)?));
        self.voconn = Some(Arc::clone(&mvoconn));

        // Virtual Input
        let mut vinput = MidiInput::new(PREFIX_VI)?;
        vinput.ignore(Ignore::None);

        // Virtual Input -> Virtual Output (midi through)
        let mvoconn_clone = Arc::clone(&mvoconn);
        self.viconn = Some(Mutex::new(vinput.create_virtual(
            &self.id,
            move |_, bytes, _| {
                let mut conn = mvoconn_clone.lock().unwrap();
                if let Err(e) = conn.send(bytes) { // midi through
                    eprintln!("Error sending bytes virtual midi through {}", e)
                }
            },
            ()
        ).unwrap()));


        let in_ports = input.ports();
        let mut in_port = None;
        let out_ports = output.ports();
        let mut out_port = None;

        #[cfg(target_os = "linux")]
        let re = Regex::new(&format!(r"^{}.*{}.*", PREFIX_VO, &self.id)).unwrap();
        #[cfg(target_os = "macos")]
        let re = Regex::new(&format!(r"^{}$", &self.id)).unwrap();
        for p in in_ports.iter() {
            if re.is_match(&input.port_name(p)?) {
                in_port = Some(p.clone());
                break;
            }
        }

        #[cfg(target_os = "linux")]
        let re = Regex::new(&format!(r"^{}.*{}.*", PREFIX_VI, &self.id)).unwrap();
        #[cfg(target_os = "macos")]
        let re = Regex::new(&format!(r"^{}$", &self.id)).unwrap();
        for p in out_ports.iter() {
            if re.is_match(&output.port_name(p)?) {
                out_port = Some(p.clone());
                break;
            }
        }

        if in_port.is_none() {
            let mut pname = "".to_string();
            if in_ports.len() > 0 {
                let p = &input.port_name(&in_ports[0])?;
                pname = p.to_string();
            }
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("VC Output not found {} {}",in_ports.len(), pname))))?
            // Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Virtual cable init failed virtual output port not found")))?
        }

        if out_port.is_none() {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Virtual cable init failed virtual input port not found")))?
        }

        // Virtual Out -> Input -> Hub
        let id = self.id.clone();
        self.iconn = Some(Mutex::new(input.connect(
            &in_port.unwrap(),
            &self.id,
            move |ts, bytes, _| {
                let hub_instance = Hub::get_instance();
                let mut hub = hub_instance.lock().unwrap();
                hub.process(ts, &bytes.to_vec(), &id, "*", "*", "*");
            },
            ()
        ).unwrap()));

        // Output -> Virtual Input
        self.oconn = Some(Mutex::new(output.connect(&out_port.unwrap(), &self.id).unwrap()));
        Ok(())
    }
    fn destroy(&mut self) {
        if let Some(mutex) = self.iconn.take() {
            let conn = mutex.into_inner().unwrap();
            conn.close();
        }
        if let Some(mutex) = self.oconn.take() {
            let conn = mutex.into_inner().unwrap();
            conn.close();
        }
        if let Some(mutex) = self.viconn.take() {
            let conn = mutex.into_inner().unwrap();
            conn.close();
        }
        if let Some(mutex) = self.voconn.take() {
            match Arc::try_unwrap(mutex) {
                Ok(mutex) => {
                    let conn = mutex.into_inner().unwrap();
                    conn.close();
                }
                Err(_) => {
                    eprintln!("Cannot unwrap Arc as there are still other references to it.");
                }
            }
        }
    }
    fn get_id(&self) -> &str {
        return &self.id
    }
    fn get_class(&self) -> &str {
        return &self.class;
    }
    fn serialize(&self) -> Result<Value, Error> {
        serde_json::to_value(self)
    }
    fn get_data(&mut self, _key: String) -> Result<Option<Value>, String> { Ok(None) }
    fn set_data(&mut self, _key: String, _data:Value) -> Result<(), String> { Ok(()) }
    fn delete_data(&mut self, _key: String) -> Result<(), String> { Ok(()) }
    fn process(
            &mut self,
            bytes: &Vec<u8>,
            _from: &str,
            _to: &str,
            _from_port: &str,
            _to_port: &str
        ) -> Vec<(String, Vec<u8>)>
    {
        if let Some(ref mutex) = self.oconn {
            let mut conn = mutex.lock().unwrap();
            if let Err(e) = conn.send(bytes) { // send message to virtual midi channel
                eprintln!("Error sending bytes from virtual output {} {}", self.out_id, e);
            }
        }
        vec![] // ignore forward processing
    }
}

#[cfg(all(test, not(windows)))]
mod tests {
    use super::*;
    use std::thread;
    use serde::Deserialize;
    use serial_test::serial;

    #[derive(Serialize, Deserialize)]
    pub struct MockDevice {
        id: String,
        bytes: Vec<u8>,
        class: String
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
    fn create_destroy () {
        let input = MidiInput::new("i").expect("Failed to create input");
        let output = MidiOutput::new("o").expect("Failed to create output");
        let i_count = input.port_count();
        let o_count = output.port_count();
        {
            let mut c = VirtualC::new("Test virtual channel");
            c.init().expect("Failed to initialize virtual");
            assert_eq!(i_count + 2, input.port_count());
            assert_eq!(o_count + 2, output.port_count());
        }
        assert_eq!(i_count, input.port_count());
        assert_eq!(o_count, output.port_count());
    }

    #[test]
    #[serial]
    fn midi_through () {
        // Handle async midi through in a new thread
        let handle = thread::spawn(move || {
            let mut c = VirtualC::new("Test virtual channel");
            c.init().expect("Failed to initialize virtual");
            let hub_instance = Hub::get_instance();
            let mut hub = hub_instance.lock().unwrap_or_else(|e| e.into_inner());
            hub.destroy();
            // create mock device to listen to hub events
            let d1: Box<dyn Device> = Box::new(MockDevice::new("1"));
            hub.add_device(d1);
            let bytes = vec![0x80, 0, 0];
            hub.connect("Test virtual channel", "1", "*", "*");

            if let Some(mutex) = c.oconn {
                let mut conn = mutex.lock().unwrap();
                conn.send(&bytes.clone()).expect("Failed to send bytes");
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        });
        handle.join().unwrap();
        let hub_instance = Hub::get_instance();
        let hub = hub_instance.lock().unwrap_or_else(|e| e.into_inner());
        let get_device = |value: Option<Value>| serde_json::from_value(value.unwrap()).expect("Invalid JSON");
        let b1: MockDevice = get_device(hub.serialize_device("1"));
        assert_eq!(b1.bytes[0], 0x81);
    }
}