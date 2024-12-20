use mlua::{Lua, Table, Value};
use serde::Serialize;
use serde_json::{json, Error, Value as JsonValue};
use std::{collections::HashMap, error::Error as StdErr, sync::Mutex};
use crate::{app, devices::device::Device, globals::{EVT_SCRIPT_ERROR, EVT_SCRIPT_LOG }};

#[derive(Serialize)]
pub struct Script {
    pub id: String,
    pub class: String,
    pub script: String,
    #[serde(skip_serializing)]
    lua: Mutex<Lua>,
    #[serde(skip_serializing)]
    test_bytes: Vec<u8> // used for testing the script from the frontend
}

impl Script {
    pub fn new(id: &str) -> Self {
        Script {
            id: String::from(id),
            class: String::from("script"),
            script: "".to_string(),
            lua: Mutex::new(Lua::new()),
            test_bytes: vec![]
        }
    }
}

// Function to stringify an unknown mlua Value
fn stringify_value(value: &Value, depth: u32) -> String {
    match value {
        Value::Nil => "nil".to_string(),
        Value::Boolean(b) => b.to_string(),
        Value::LightUserData(_) => "lightuserdata".to_string(),
        Value::Integer(i) => i.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.to_string_lossy(),
        Value::Table(t) => {
            let mut table_str = String::from("{");
            for pair in t.pairs::<Value, Value>() {
                let (key, value) = pair.unwrap();
                if depth < 10 {
                    table_str.push_str(&format!("{}: {}, ", stringify_value(&key, depth + 1), stringify_value(&value, depth + 1)));
                }
            }
            if table_str.ends_with(", ") {
                table_str.pop(); // Remove last space
                table_str.pop(); // Remove last comma
            }
            table_str.push('}');
            table_str
        }
        Value::Function(_) => "function".to_string(),
        Value::Thread(_) => "thread".to_string(),
        Value::UserData(_) => "userdata".to_string(),
        Value::Error(e) => format!("error: {}", e),
        _ => "".to_string()
    }
}

impl Device for Script {
    fn get_id(&self) -> &str {
        return &self.id;
    }
    fn get_class(&self) -> &str {
        return &self.class;
    }
    fn destroy(&mut self) {}
    fn get_data(&mut self, key: String) -> Result<Option<JsonValue>, String> {
        match key.as_str() {
            "globals" => {
                let mut result: HashMap<String, JsonValue> = HashMap::new();
                let lua = self.lua.lock().unwrap();
                let globals = lua.globals();
                for pair in globals.pairs::<String, Value>() {
                    let (key, value) = pair.unwrap();
                    let json_value = match value {
                        Value::Boolean(b) => JsonValue::Bool(b),
                        Value::Integer(i) => JsonValue::Number(i.into()),
                        Value::Number(n) => JsonValue::Number(serde_json::Number::from_f64(n).unwrap()),
                        Value::String(s) => JsonValue::String(s.to_string_lossy()),
                        _ => continue
                    };
                    result.insert(key, json_value);
                };
                let res:Value = globals.get("res").unwrap();
                result.insert("res".to_string(), json!(stringify_value(&res, 0)));
                Ok(Some(serde_json::to_value(result).expect("Failed to serialize hashmap")))
            }
            "test_result" => {
                let test_bytes = self.test_bytes.clone();
                let res = self.process(&test_bytes, "*", "*", "*", "*");
                Ok(Some(serde_json::to_value(res).expect("Failed to process bytes")))
            }
            _ => Ok(None)
        }
    }
    fn set_data(&mut self, key: String, data:JsonValue) -> Result<(), String> {
        match key.as_str() {
            "script" => {
                if let Some(script) = data.as_str() {
                    self.script = script.to_string();
                } else {
                    Err("Failed to set script")?
                }
            },
            "test_bytes" => {
                if let JsonValue::Array(arr) = data {
                    self.test_bytes = arr.into_iter()
                        .filter_map(|x| x.as_u64().map(|n| (n % 256) as u8))
                        .collect();
                }
            },
            "reset-state" => {
                let _ = self.init();
                app::emit(EVT_SCRIPT_LOG, json!({ "id": self.id, "message": "Script state reset" }));
            }
            _ => {}
        }

        Ok(())
    }
    fn delete_data(&mut self, _key: String) -> Result<(), String> { Ok(()) }

    fn init(&mut self) -> Result<(), Box<dyn StdErr>> {
        let lua = Lua::new();
        let id = self.id.clone();
        let globals = lua.globals();
        let fn_log = Some(lua.create_function(move |_, value: Value| {
            app::emit(EVT_SCRIPT_LOG, json!({ "id": id, "message": stringify_value(&value, 0) }));
            Ok(())
        }).unwrap());
        let _ = globals.set("log", fn_log);

        let mut lock = self.lua.lock().unwrap();
        *lock = lua;
        Ok(())
    }

    fn serialize(&self) -> Result<JsonValue, Error> {
        serde_json::to_value(self)
    }

    fn process(
        &mut self,
        bytes: &Vec<u8>,
        from: &str,
        to: &str,
        from_port: &str,
        to_port: &str
    ) -> Vec<(String, Vec<u8>)> {
        let lua = self.lua.lock().unwrap();
        let globals = lua.globals();
        let _ = globals.set("from", from);
        let _ = globals.set("to", to);
        let _ = globals.set("from_port", from_port);
        let _ = globals.set("to_port", to_port);
        let _ = globals.set("res", lua.create_table().unwrap());
        let bytes_table = lua.create_table().unwrap();
        for byte in bytes.iter() {
            let _ = bytes_table.push(*byte);
        }
        let _ = globals.set("bytes", bytes_table);

        let result = lua.load(&self.script)
            .exec()
            .map_err(|err| {
                app::emit(EVT_SCRIPT_ERROR, json!({
                    "id": self.id,
                    "error": format!("{}", err)
                }));
                format!("{}", err)
            });

        if result.is_ok() {
            let mut ret = vec![];
            let res:Table = globals.get("res").unwrap_or(lua.create_table().unwrap());
            for entry in res.sequence_values::<Table>() {
                let entry = entry.unwrap_or(lua.create_table().unwrap());
                let port: String = entry.get("port").unwrap_or("unknown".to_string());
                let values: Vec<u8> = entry
                    .get("bytes").unwrap_or(lua.create_table().unwrap())
                    .sequence_values()
                    .collect::<Result<Vec<u8>, _>>().unwrap_or(vec![]);
                ret.push((port, values));
            }
            ret
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_script () {
        let mut script = Script::new("");
        let _ = script.set_data("script".to_string(), serde_json::to_value(json!("1+1")).expect("fail"));
        assert_eq!(script.script, "1+1");
    }

    #[test]
    fn get_globals () {
        let mut script = Script::new("");
        {
            let lua = script.lua.lock().unwrap();
            lua.globals().set("prop", 123).expect("");
        }
        let globals = script.get_data("globals".to_string()).unwrap().unwrap();
        let prop = globals.get("prop").unwrap().as_u64().unwrap();
        assert_eq!(prop, 123);
    }

    #[test]
    fn process_script () {
        let mut script = Script::new("");
        let code = r#"
            x = 1
        "#;
        let _ = script.set_data("script".to_string(), serde_json::to_value(json!(code)).expect("fail"));
        let res = script.process(&vec![], "*", "*", "*", "*");
        assert_eq!(res, []);
        let globals = script.get_data("globals".to_string()).unwrap().unwrap();
        let prop = globals.get("x").unwrap().as_u64().unwrap();
        assert_eq!(prop, 1);
    }

    #[test]
    fn process_bytes () {
        let mut script = Script::new("");
        let code = r#"
            res = {
                { port = "1", bytes = {1, 2, 3} },
                { port = "2", bytes = {4, 5, 6} }
            }
        "#;
        let _ = script.set_data("script".to_string(), serde_json::to_value(json!(code)).expect("fail"));
        let res = script.process(&vec![], "*", "*", "*", "*");
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], ("1".to_string(), vec![1, 2, 3 ]));
        assert_eq!(res[1], ("2".to_string(), vec![4, 5, 6 ]));
    }

    #[test]
    fn process_bytes_twice () {
        let mut script = Script::new("");
        let code = r#"
            table.insert(res, { port = "1", bytes = {1, 2, 3} })
        "#;
        let _ = script.set_data("script".to_string(), serde_json::to_value(json!(code)).expect("fail"));
        let res = script.process(&vec![], "*", "*", "*", "*");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], ("1".to_string(), vec![1, 2, 3 ]));
        let res = script.process(&vec![], "*", "*", "*", "*");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], ("1".to_string(), vec![1, 2, 3 ]));
    }

    #[test]
    fn process_incorrect_res () {
        let mut script = Script::new("");
        let code = r#"
            table.insert(res, {})
        "#;
        let _ = script.set_data("script".to_string(), serde_json::to_value(json!(code)).expect("fail"));
        let res = script.process(&vec![], "*", "*", "*", "*");
        assert_eq!(res, vec![("unknown".to_string(), vec![])]);

        let code = r#"
            res = 1
        "#;
        let _ = script.set_data("script".to_string(), serde_json::to_value(json!(code)).expect("fail"));
        let res = script.process(&vec![], "*", "*", "*", "*");
        assert_eq!(res, vec![]);

        let code = r#""#;
        let _ = script.set_data("script".to_string(), serde_json::to_value(json!(code)).expect("fail"));
        let res = script.process(&vec![], "*", "*", "*", "*");
        assert_eq!(res, vec![]);
    }

    #[test]
    fn process_bad_script () {
        let mut script = Script::new("");
        let code = r#"
            sdfsf =sf=()))((=sd=f=s !!~df= s=dfs
        "#;
        let _ = script.set_data("script".to_string(), serde_json::to_value(json!(code)).expect("fail"));
        let res = script.process(&vec![], "*", "*", "*", "*");
        assert_eq!(res, vec![]);
    }

    #[test]
    fn calls_custom_fns () {
        let mut script = Script::new("");
        script.init().expect("");
        let code = r#"
            log("12345")
        "#;
        let _ = script.set_data("script".to_string(), serde_json::to_value(json!(code)).expect("fail"));
        script.process(&vec![], "*", "*", "*", "*");
    }
}