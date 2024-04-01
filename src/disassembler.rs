use serde::{Deserialize, Serialize};
mod bin6502;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Output {
    disassembly: Vec<String>,
}

pub enum BinaryKind {
    Bin6502,
}

pub fn disassemble(
    data: Vec<u8>,
    start_address: Option<u16>,
    end_address: Option<u16>,
    binary_kind: BinaryKind,
) -> Output {
    match binary_kind {
        BinaryKind::Bin6502 => Output {
            disassembly: bin6502::disassemble(data, start_address, end_address),
        },
    }
}
