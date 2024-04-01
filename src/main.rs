use axum::{
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing::info;

mod disassembler;

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
    start_address: Option<u16>,
    end_address: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Output {
    disassembly: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Error {
    message: String,
}

async fn handler(Json(payload): Json<Payload>) -> Response {
    let Payload {
        data,
        start_address,
        end_address,
    } = payload;

    match (start_address, end_address) {
        (Some(start), _) if start as usize >= data.len() => {
            return Json(Error {
                message: "Start address is out of bounds".to_string(),
            })
            .into_response();
        }
        (_, Some(end)) if end as usize >= data.len() => {
            return Json(Error {
                message: "End address is out of bounds".to_string(),
            })
            .into_response();
        }
        (Some(start), Some(end)) if start >= end => {
            return Json(Error {
                message: "Start address must be less than end address".to_string(),
            })
            .into_response();
        }
        _ => {}
    }

    let res = disassembler::disassemble(
        data,
        start_address,
        end_address,
        disassembler::BinaryKind::Bin6502,
    );

    match res.disassembly {
        Ok(disassembly) => Json(Output { disassembly }).into_response(),
        Err(e) => Json(Error { message: e }).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_invalid_start() {
        const URL: &'static str = "http://localhost:9999/";
        let client = reqwest::Client::builder().build().unwrap();

        let payload = Payload {
            data: [0xa9, 0xbd, 0xa0, 0xbd].to_vec(),
            start_address: Some(5),
            end_address: None,
        };

        let res: Error = client
            .post(URL)
            .json(&payload)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        let expected: Error = Error {
            message: "Start address is out of bounds".to_string(),
        };

        assert_eq!(expected, res);
    }

    #[tokio::test]
    async fn test_invalid_end() {
        const URL: &'static str = "http://localhost:9999/";
        let client = reqwest::Client::builder().build().unwrap();

        let payload = Payload {
            data: [0xa9, 0xbd, 0xa0, 0xbd].to_vec(),
            start_address: None,
            end_address: Some(5),
        };

        let res: Error = client
            .post(URL)
            .json(&payload)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        let expected: Error = Error {
            message: "End address is out of bounds".to_string(),
        };

        assert_eq!(expected, res);
    }
}
