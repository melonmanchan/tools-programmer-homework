use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Disassembly {
    start_address: u16,
    bytes_used: Vec<u8>,
    instructions: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Output {
    disassembly: Vec<String>,
}

pub fn disassemble(data: Vec<u8>, start_address: Option<u16>, end_address: Option<u16>) -> Output {
    let mut disassembly = Vec::new();

    let mut pc = usize::from(start_address.unwrap_or(0));
    let end = usize::from(end_address.unwrap_or(data.len() as u16));
    let map = &opcode::INSTRUCTION_MAP;

    while pc < end {
        let start_address = pc as u16;
        let start_byte = data[pc];

        if let Some(opcode) = map.get(&start_byte) {
            let code = &opcode.instructions;
            let mut instructions_len = 0;

            if code.contains("hh") {
                instructions_len += 1;
                pc += 1;
            }

            if code.contains("ll") {
                instructions_len += 1;
                pc += 1;
            }

            match instructions_len {
                0 => {
                    let out = Disassembly {
                        instructions: opcode.instructions.to_string(),
                        bytes_used: vec![start_byte],
                        start_address,
                    };

                    disassembly.push(out);
                }
                1 => {
                    let high_byte = data[pc];
                    let is_relative = opcode.is_relative.unwrap_or(false);

                    /*
                                         * Relative

                    *Relative addressing on the 6502 is only used for branch operations. The byte after the opcode is
                    *the branch offset. If the branch is taken, the new address will the the current PC plus the offset.
                    *The offset is a signed byte, so it can jump a maximum of 127 bytes forward, or 128 bytes backward.
                    *(For more info about signed numbers, check here.)
                    */

                    if is_relative {
                        // Thanks chatgpt for this... Fucking off by ones
                        let signed_offset = high_byte as i8;
                        let target_address = pc as i16 + 1 + signed_offset as i16;

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
                        pc += 1; // Move past the operand for the next iteration
                        continue;
                    }

                    let instr = opcode
                        .instructions
                        .replace("hh", &format!("{:02x}", high_byte));

                    let bytes_used = vec![start_byte, high_byte];

                    let out = Disassembly {
                        instructions: instr,
                        bytes_used,
                        start_address,
                    };

                    disassembly.push(out);
                }
                2 => {
                    let low_byte = data[pc - 1];
                    let high_byte = data[pc];

                    let instr = opcode
                        .instructions
                        .replace("hh", &format!("{:02x}", high_byte))
                        .replace("ll", &format!("{:02x}", low_byte));

                    let bytes_used = vec![start_byte, low_byte, high_byte];

                    let out = Disassembly {
                        instructions: instr,
                        bytes_used,
                        start_address,
                    };

                    disassembly.push(out);
                }
                _ => panic!("Invalid instruction length"),
            }
        } else {
            // Unknown opcode
            let out = Disassembly {
                instructions: "???".to_string(),
                bytes_used: vec![start_byte],
                start_address,
            };

            disassembly.push(out);
        }

        pc += 1;
    }

    println!("{:x?}", data);

    for i in disassembly.iter() {
        println!(
            "0x{:04X} {:X?} {}",
            i.start_address, i.bytes_used, i.instructions
        );
    }

    let output_disassembly = disassembly
        .iter()
        .map(|x| {
            format!(
                "0x{:04X} {} {}",
                x.start_address,
                x.bytes_used
                    .iter()
                    .map(|byte| format!("{:02x}", byte))
                    .collect::<Vec<_>>()
                    .join(" "),
                x.instructions
            )
        })
        .collect();

    Output {
        disassembly: output_disassembly,
    }
}
