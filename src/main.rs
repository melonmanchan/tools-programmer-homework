use axum::{
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing::info;

mod opcode;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let routes = Router::new().route("/", post(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 9999));
    info!("{:<15} - {addr}\n", "LISTENING");
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Output {
    disassembly: Vec<String>,
}

async fn handler(Json(payload): Json<Payload>) -> Response {
    let Payload { data } = payload;

    let res = disassemble(data);

    Json(res).into_response()
}

#[derive(Debug)]
struct Disassembly {
    start_address: u16,
    bytes_used: Vec<u8>,
    instructions: String,
}

fn disassemble(data: Vec<u8>) -> Output {
    // process the incoming data here and return type Output
    // loop over vector

    let mut disassembly = Vec::new();
    let mut pc = 0;
    let end = data.len();
    let map = opcode::INSTRUCTION_MAP.clone();

    while pc < end {
        let start_address = pc as u16;
        let start_byte = data[pc];

        if let Some(opcode) = map.get(&start_byte) {
            let code = opcode.instructions.to_string();

            // let mut bytes = Vec::new();

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
                            .replace("hh", &format!("{:04X}", target_address));

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

    println!("{:?}", output_disassembly);

    Output {
        disassembly: output_disassembly,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_api_disassemble_ok() {
        const URL: &'static str = "http://localhost:9999/";
        let client = reqwest::Client::builder().build().unwrap();

        let payload = Payload {
            data: [0xa9, 0xbd, 0xa0, 0xbd, 0x20, 0x28, 0xba].to_vec(),
        };

        let res: Output = client
            .post(URL)
            .json(&payload)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        let expected: Output = Output {
            disassembly: [
                "0x0000 a9 bd LDA #$bd",
                "0x0002 a0 bd LDY #$bd",
                "0x0004 20 28 ba JSR $ba28",
            ]
            .iter()
            .map(|&s| s.into())
            .collect(),
        };
        assert_eq!(expected, res);
    }

    #[tokio::test]
    async fn test_first_test_binary() {
        const URL: &'static str = "http://localhost:9999/";
        let client = reqwest::Client::builder().build().unwrap();

        let data = std::fs::read("./test-bin/test1.bin").unwrap();

        let payload = Payload { data };

        // Stolen from https://www.masswerk.at/6502/disassembler.html
        let expected: Output = Output {
            disassembly: [
                "0x0000 48 PHA",
                "0x0001 e7 ???",
                "0x0002 20 20 70 JSR $7020",
                "0x0005 21 61 AND ($61,x)",
                "0x0007 00 BRK",
                "0x0008 f8 SED",
                "0x0009 ee 61 e6 INC $e661",
                "0x000C 61 00 ADC ($00,x)",
                "0x000E 04 ???",
                "0x000F 02 ???",
                "0x0010 22 ???",
                "0x0011 6e 00 84 ROR $8400",
                "0x0014 41 e9 EOR ($e9,x)",
                "0x0016 00 BRK",
                "0x0017 16 74 ASL $74,x",
                "0x0019 07 ???",
                "0x001A 0c ???",
                "0x001B 00 BRK",
                "0x001C 00 BRK",
                "0x001D 44 ???",
                "0x001E 67 ???",
                "0x001F 18 CLC",
                "0x0020 41 e8 EOR ($e8,x)",
                "0x0022 00 BRK",
                "0x0023 20 74 06 JSR $0674",
                "0x0026 0c ???",
                "0x0027 00 BRK",
                "0x0028 00 BRK",
                "0x0029 41 67 EOR ($67,x)",
                "0x002B 0c ???",
                "0x002C 45 e9 EOR $e9",
                "0x002E 00 BRK",
                "0x002F 06 0c ASL $0c",
                "0x0031 00 BRK",
                "0x0032 00 BRK",
                "0x0033 55 67 EOR $67,x",
                "0x0035 1e 60 38 ASL $3860,x",
            ]
            .iter()
            .map(|&s| s.into())
            .collect(),
        };

        let res: Output = client
            .post(URL)
            .json(&payload)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        assert_eq!(res, expected);
    }

    #[tokio::test]
    #[ignore]
    async fn test_second_test_binary() {
        const URL: &'static str = "http://localhost:9999/";
        let client = reqwest::Client::builder().build().unwrap();

        let data = std::fs::read("./test-bin/test2.bin").unwrap();

        let payload = Payload { data };

        let _res: Output = client
            .post(URL)
            .json(&payload)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        assert_eq!(true, true);
    }
}
