use jsonschema::JSONSchema;
use lazy_static::lazy_static;
use serde_json;
use std::collections::HashMap;
use std::fmt;

enum InstructionLength {
    Zero,
    OneByte,
    TwoBytes,
}

#[derive(Clone, Debug)]
struct OpCode {
    instructions: String,
    is_relative: Option<bool>,
}

trait OpCodeTrait {
    fn format_instruction_high_byte(&self, low_byte: u8) -> String;
    fn format_instruction_low_and_high_byte(&self, low_byte: u8, high_byte: u8) -> String;
    fn get_instruction_byte_length(&self) -> InstructionLength;
}

// These string replacements and "contains" aren't the most efficient way of doing this
// but it's fine for now!
impl OpCodeTrait for OpCode {
    fn format_instruction_high_byte(&self, low_byte: u8) -> String {
        self.instructions
            .replace("hh", &format!("{:02x}", low_byte))
    }

    fn format_instruction_low_and_high_byte(&self, low_byte: u8, high_byte: u8) -> String {
        self.instructions
            .replace("hh", &format!("{:02x}", high_byte))
            .replace("ll", &format!("{:02x}", low_byte))
    }

    fn get_instruction_byte_length(&self) -> InstructionLength {
        match self.instructions.as_str() {
            instr if instr.contains("hh") && instr.contains("ll") => InstructionLength::TwoBytes,
            instr if instr.contains("hh") || instr.contains("ll") => InstructionLength::OneByte,
            _ => InstructionLength::Zero,
        }
    }
}

#[derive(Debug)]
pub struct Disassembly {
    start_address: u16,
    bytes_used: Vec<u8>,
    instructions: String,
}

impl fmt::Display for Disassembly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hex_bytes = self
            .bytes_used
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<Vec<_>>()
            .join(" ");

        write!(
            f,
            "0x{:04X} {} {}",
            self.start_address, hex_bytes, self.instructions
        )
    }
}

// This handy opcode file came from https://www.awsm.de/blog/pydisass/
static OPCODE_FILE: &'static str = include_str!("./bin6502.json");
// I also generated a JSON schema for this file to validate it during build time
// Might be a bit overkill, but it's generally a good idea to validate JSON files
static OPCODE_SCHEMA: &'static str = include_str!("./bin6502.schema.json");

fn get_json_content() -> Result<serde_json::Value, serde_json::Error> {
    serde_json::from_str(OPCODE_FILE)
}

fn get_schema_content() -> Result<serde_json::Value, serde_json::Error> {
    serde_json::from_str(OPCODE_SCHEMA)
}

// Lazily create the hashmap required for looking up opcodes
lazy_static! {
    static ref INSTRUCTION_MAP: HashMap<u8, OpCode> = create_instruction_map();
}

fn create_instruction_map() -> HashMap<u8, OpCode> {
    let json_content = get_json_content().unwrap();
    let schema = get_schema_content().unwrap();

    let compiled_schema = JSONSchema::compile(&schema).unwrap();
    let result = compiled_schema.validate(&json_content);

    if let Err(errors) = result {
        for error in errors {
            println!("Validation error: {}", error);
            println!("Instance path: {}", error.instance_path);
        }

        panic!("Invalid JSON schema");
    }

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

pub fn disassemble(
    data: Vec<u8>,
    start_address: Option<u16>,
    end_address: Option<u16>,
) -> Result<Vec<String>, String> {
    let mut disassembly = Vec::new();

    let mut program_counter = usize::from(start_address.unwrap_or(0));
    let end = usize::from(end_address.unwrap_or(data.len() as u16));

    while program_counter < end {
        let start_address = program_counter as u16;
        let start_byte = data[program_counter];

        let possible_opcode = INSTRUCTION_MAP.get(&start_byte);

        match possible_opcode {
            Some(opcode) => {
                let instruction_length = opcode.get_instruction_byte_length();

                match instruction_length {
                    InstructionLength::Zero => {
                        let out = Disassembly {
                            instructions: opcode.instructions.to_string(),
                            bytes_used: vec![start_byte],
                            start_address,
                        };

                        disassembly.push(out);
                    }
                    InstructionLength::OneByte => {
                        program_counter += 1;

                        let high_byte = data[program_counter];
                        let is_relative = opcode.is_relative.unwrap_or(false);

                        if is_relative {
                            // Thanks chatgpt for this... Fucking off by ones
                            let signed_offset = high_byte as i8;
                            let target_address = program_counter as i16 + 1 + signed_offset as i16;

                            // A bit messy here...
                            let instr = opcode
                                .instructions
                                .replace("hh", &format!("{:02x}", target_address));

                            let bytes_used = vec![start_byte, high_byte];

                            let out = Disassembly {
                                instructions: instr,
                                bytes_used,
                                start_address,
                            };

                            disassembly.push(out);
                        } else {
                            let instr = opcode.format_instruction_high_byte(high_byte);

                            let bytes_used = vec![start_byte, high_byte];

                            let out = Disassembly {
                                instructions: instr,
                                bytes_used,
                                start_address,
                            };

                            disassembly.push(out);
                        }
                    }
                    InstructionLength::TwoBytes => {
                        program_counter += 2;
                        let low_byte = data[program_counter - 1];
                        let high_byte = data[program_counter];

                        let instr =
                            opcode.format_instruction_low_and_high_byte(low_byte, high_byte);

                        let bytes_used = vec![start_byte, low_byte, high_byte];

                        let out = Disassembly {
                            instructions: instr,
                            bytes_used,
                            start_address,
                        };

                        disassembly.push(out);
                    }
                    // Should never happen, but let's leave it here to demonstrate
                    _ => return Err("Invalid instruction length".to_string()),
                }
            }

            None => {
                // We're dealing with an unknown opcode
                let out = Disassembly {
                    instructions: "???".to_string(),
                    bytes_used: vec![start_byte],
                    start_address,
                };

                disassembly.push(out);
            }
        }

        // Finally, increment the program counter
        program_counter += 1;
    }

    let output_disassembly = disassembly.iter().map(|x| x.to_string()).collect();

    Ok(output_disassembly)
}
