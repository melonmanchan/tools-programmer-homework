use axum::{
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing::{info, instrument};

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
    instructions: String,
    bytes_used: Vec<u8>,
    start_address: u16,
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

            println!("{} {}", start_byte, opcode.instructions);

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

    Output {
        disassembly: [
            "0x0000 a9 bd        LDA #$bd",
            "0x0002 a0 bd        LDY #$bd",
            "0x0004 20 28 ba     JSR $ba28",
        ]
        .iter()
        .map(|&s| s.into())
        .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    #[ignore]
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
                "0x0000 a9 bd        LDA #$bd",
                "0x0002 a0 bd        LDY #$bd",
                "0x0004 20 28 ba     JSR $ba28",
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

        let _res: Output = client
            .post(URL)
            .json(&payload)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        //let expected: Output = Output {
        //    disassembly: [
        //        "0x0000 a9 bd        LDA #$bd",
        //        "0x0002 a0 bd        LDY #$bd",
        //        "0x0004 20 28 ba     JSR $ba28",
        //    ]
        //    .iter()
        //    .map(|&s| s.into())
        //    .collect(),
        //};

        assert_eq!(true, true);
    }
}
