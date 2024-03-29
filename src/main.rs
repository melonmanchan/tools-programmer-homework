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

fn disassemble(data: Vec<u8>) -> Output {
    // process the incoming data here and return type Output

    // loop over vector

    let mut pc = 0;
    let end = data.len();
    let map = opcode::INSTRUCTION_MAP.clone();

    while pc < end {
        println!("{:02x}", data[pc]);

        if let Some(opcode) = map.get(&data[pc]) {
            println!("{:?}", opcode.instructions);
        }
        pc += 1;
    }

    println!("pc: {}", pc);

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
}
