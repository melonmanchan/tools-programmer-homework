use lazy_static::lazy_static;
use serde_json;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct OpCode {
    pub instructions: String,
    pub is_relative: Option<bool>,
}

static OPCODE_FILE: &'static str = include_str!("./opcodes.json");

lazy_static! {
    pub static ref INSTRUCTION_MAP: HashMap<u8, OpCode> = create_instruction_map();
}

fn get_json_content() -> serde_json::Value {
    let as_json = serde_json::from_str(&OPCODE_FILE).unwrap();

    as_json
}

fn create_instruction_map() -> HashMap<u8, OpCode> {
    let json_content = get_json_content();

    let hashmap = json_content
        .as_object()
        .unwrap()
        .iter()
        .map(|(key, value)| {
            let instructions = value["ins"].as_str().unwrap();
            let is_relative = value["rel"].as_u64();

            let opcode = OpCode {
                instructions: instructions.to_string(),
                is_relative: is_relative.map(|x| x == 1),
            };

            (u8::from_str_radix(key, 16).unwrap(), opcode)
        })
        .collect();

    hashmap
}
