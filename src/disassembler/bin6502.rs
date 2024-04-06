use lazy_static::lazy_static;
use serde::Deserialize;

use std::collections::HashMap;
use std::fmt;

enum InstructionArgumentLength {
    Zero,
    OneByte,
    TwoBytes,
}

#[derive(Clone, Debug, Deserialize)]
struct OpCode {
    #[serde(rename = "ins")]
    instructions: String,

    #[serde(rename = "rel")]
    is_relative: Option<bool>,
}

// These string replacements and "contains" aren't the most efficient way here
// but it's fine for now!
impl OpCode {
    fn format_instruction_high_byte(&self, low_byte: u8) -> String {
        self.instructions
            .replace("hh", &format!("{:02x}", low_byte))
    }

    fn format_instruction_low_and_high_byte(&self, low_byte: u8, high_byte: u8) -> String {
        self.instructions
            .replace("hh", &format!("{:02x}", high_byte))
            .replace("ll", &format!("{:02x}", low_byte))
    }

    fn get_intruction_byte_length(&self) -> InstructionArgumentLength {
        match self.instructions.as_str() {
            instr if instr.contains("hh") && instr.contains("ll") => {
                InstructionArgumentLength::TwoBytes
            }
            instr if instr.contains("hh") || instr.contains("ll") => {
                InstructionArgumentLength::OneByte
            }
            _ => InstructionArgumentLength::Zero,
        }
    }
}

#[derive(Debug)]
struct Disassembly {
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

// Lazily create the hashmap required for looking up opcodes
lazy_static! {
    static ref INSTRUCTION_MAP: HashMap<u8, OpCode> = create_instruction_map();
}

fn create_instruction_map() -> HashMap<u8, OpCode> {
    // This handy opcode file came from https://www.awsm.de/blog/pydisass/
    // in the beginnin the
    // `create_instruction_map` function was a massive mega-imperative pile fo HashMap.insert calls, so
    // I found this nice looking JSON file and decided to use it as a base for my own instead, even
    // though I had to fix some typos and double-check it against the official reference
    //
    // Initially I had an idea that the caller could pass in their custom illegal opcode map in the
    // request payload since there's seemingly so many flavors of 6502 out there, but didn't end up
    // implementing here.
    static OPCODE_FILE: &str = include_str!("./bin6502.json");

    let hashmap = serde_json::from_str::<HashMap<String, OpCode>>(OPCODE_FILE)
        .expect("Expected instruction map to match schema")
        .into_iter()
        .map(|(key, opcode)| (u8::from_str_radix(&key, 16).unwrap(), opcode))
        .collect();

    hashmap
}

// This is the main function that will be called from the outside
pub fn disassemble(
    data: Vec<u8>,
    start_address: Option<u16>,
    end_address: Option<u16>,
) -> Result<Vec<String>, String> {
    let mut disassembly = Vec::new();

    let mut program_counter = usize::from(start_address.unwrap_or(0));
    let end = usize::from(end_address.unwrap_or(data.len() as u16));

    // I wonder if there's a way to write this using an iterator?
    while program_counter < end {
        let start_address = program_counter as u16;
        let start_byte = data[program_counter];

        let possible_opcode = INSTRUCTION_MAP.get(&start_byte);

        match possible_opcode {
            Some(opcode) => {
                let instruction_length = opcode.get_intruction_byte_length();

                match instruction_length {
                    InstructionArgumentLength::Zero => {
                        let out = Disassembly {
                            instructions: opcode.instructions.to_string(),
                            bytes_used: vec![start_byte],
                            start_address,
                        };

                        disassembly.push(out);
                    }
                    InstructionArgumentLength::OneByte => {
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
                    InstructionArgumentLength::TwoBytes => {
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
