use serde::{Deserialize, Serialize};
use serde_json::{Value, Error};
use std::error::Error as StdErr;
use crate::devices::device::Device;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Rule {
    pub in_channel: i32,
    pub in_type: i32,
    pub in_data1_min: i32,
    pub in_data1_max: i32,
    pub in_data2_min: i32,
    pub in_data2_max: i32,
    pub out_channel: i32,
    pub out_type: i32,
    pub out_data1_min: i32,
    pub out_data1_max: i32,
    pub out_data2_min: i32,
    pub out_data2_max: i32,
    pub pull_data1: bool,
    pub pull_data2: bool,
}

#[derive(Serialize)]
pub struct Mapper {
    pub id: String,
    pub class: String,
    pub rule: Rule,
}

impl Mapper {
    pub fn new(id: &str) -> Self {
        Mapper {
            id: String::from(id),
            class: String::from("map"),
            rule: Rule {
                in_channel: -1,
                in_type: -1,
                in_data1_min: -1,
                in_data1_max: -1,
                in_data2_min: -1,
                in_data2_max: -1,
                out_channel: -1,
                out_type: -1,
                out_data1_min: -1,
                out_data1_max: -1,
                out_data2_min: -1,
                out_data2_max: -1,
                pull_data1: false, // use data2 value for data1 mapping
                pull_data2: false // use data1 value for data2 mapping
            }
        }
    }
}

fn map_linear(value: i32, min: i32, max: i32) -> i32 {
    let range = max - min;
    let mapped = min + (value * range / 127);
    mapped
}

impl Device for Mapper {
    fn get_id(&self) -> &str { &self.id }
    fn get_class(&self) -> &str { &self.class }
    fn destroy(&mut self) {}
    fn get_data(&mut self, _key: String) -> Result<Option<Value>, String> { Ok(None) }

    fn set_data(&mut self, _key: String, data:Value) -> Result<(), String> {
        let rule:Rule = serde_json::from_value(data).map_err(|err| format!("Failed to deserialize rule {}", err))?;
        self.rule = rule;
        Ok(())
    }

    fn delete_data(&mut self, _key: String) -> Result<(), String> { Ok(()) }
    fn init(&mut self) -> Result<(), Box<dyn StdErr>> { Ok(()) }

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
        if bytes.len() < 1 || bytes.len() > 3 {
            return vec![]
        }

        let mut evt_type = (bytes[0] & 0xF0) >> 4;
        let mut channel = bytes[0] & 0x0F;
        let mut data1 = if bytes.len() > 1 { bytes[1] as i32 } else { 0 };
        let mut data2 = if bytes.len() > 2 { bytes[2] as i32 } else { 0 };
        let rule = self.rule.clone();

        // if the event is outside the map boundaries ignore it
        if (rule.in_type > -1 && rule.in_type != evt_type as i32) ||
            (rule.in_channel > -1 && rule.in_channel != channel as i32) ||
            (bytes.len() > 1 && rule.in_data1_min > -1 && rule.in_data1_min > data1) ||
            (bytes.len() > 1 && rule.in_data1_max > -1 && rule.in_data1_max < data1) ||
            (bytes.len() > 2 && rule.in_data2_min > -1 && rule.in_data2_min > data2) ||
            (bytes.len() > 2 && rule.in_data2_max > -1 && rule.in_data2_max < data2)
        {
            return vec![];
        }

        if rule.out_type != -1 {
            evt_type = rule.out_type as u8
        }
        if rule.out_channel != -1 {
            channel = rule.out_channel as u8
        }

        let dt1 = data1.clone();
        if rule.out_data1_min != -1 || rule.out_data1_max != -1 {
            let min = rule.out_data1_min.max(0);
            let max = rule.out_data1_max.max(0).max(min);
            data1 = map_linear(if rule.pull_data1 { data2 } else { data1 }, min, max);
        } else if rule.pull_data1 {
            data1 = data2;
        }
        if rule.out_data2_min != -1 || rule.out_data2_max != -1 {
            let min = rule.out_data2_min.max(0);
            let max = rule.out_data2_max.max(0).max(min);
            data2 = map_linear(if rule.pull_data2 { dt1 } else { data2 }, min, max);
        } else if rule.pull_data2 {
            data2 = dt1;
        }

        let status_byte = (evt_type << 4) | (channel & 0x0F);

        if evt_type == 0x0C || evt_type == 0x0D { // single data byte messages for program and channelAT
            vec![("*".to_string(), vec![status_byte, data1 as u8])]
        } else { // two data byte messages for other events
            vec![("*".to_string(), vec![status_byte, data1 as u8, data2 as u8])]
        }
    }
}