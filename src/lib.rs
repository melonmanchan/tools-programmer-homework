use axum::{
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing::info;

mod disassembler;

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub data: Vec<u8>,
    pub start_address: Option<u16>,
    pub end_address: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Output {
    disassembly: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Error {
    pub message: String,
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

    match res {
        Ok(res) => Json(Output {
            disassembly: res.disassembly,
        })
        .into_response(),

        Err(e) => Json(Error { message: e }).into_response(),
    }
}

pub async fn run() {
    tracing_subscriber::fmt().init();

    let routes = Router::new().route("/", post(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 9999));
    info!("{:<15} - {addr}\n", "LISTENING");

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}
