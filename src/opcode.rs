use std::collections::HashMap;

pub struct OpCode {
    instructions: String,
}

// TODO: Add rest of instructions, look into static hashmap
// https://docs.rs/phf/latest/phf/
pub fn create_instruction_map() -> HashMap<u8, OpCode> {
    let mut map: HashMap<u8, OpCode> = HashMap::new();
    map.insert(
        0x00,
        OpCode {
            instructions: "brk".to_string(),
        },
    );
    map
}
