use serde::{Deserialize, Serialize};
use std::result::Result;
mod bin6502;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Output {
    pub disassembly: Vec<String>,
}

// Only one binary kind for now, but here's where I would put the rest!
pub enum BinaryKind {
    Bin6502,
}

// Instead of using a String here it might be to use a custom error type, but I had a headache
// using Box<Error> with serde_json so let's leave it like this for now
pub fn disassemble(
    // I think it's more flexible to allow this function to allow a slice instead of a Vec
    data: &[u8],
    start_address: Option<u16>,
    end_address: Option<u16>,
    binary_kind: BinaryKind,
) -> Result<Output, String> {
    match binary_kind {
        BinaryKind::Bin6502 => {
            let disassembly = bin6502::disassemble(data, start_address, end_address);

            match disassembly {
                Ok(disassembly) => Ok(Output { disassembly }),
                Err(e) => Err(e),
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // These tests could very well live inside the bin6502.rs file but let's keep them here for now
    #[test]
    fn test_api_disassemble_ok() {
        let data = vec![0xa9, 0xbd, 0xa0, 0xbd, 0x20, 0x28, 0xba];
        let start_address = Some(0x0000);
        let end_address = None;

        let output = disassemble(
            data.as_slice(),
            start_address,
            end_address,
            BinaryKind::Bin6502,
        );

        let expected = vec![
            "0x0000 a9 bd LDA #$bd",
            "0x0002 a0 bd LDY #$bd",
            "0x0004 20 28 ba JSR $ba28",
        ];

        assert_eq!(output.unwrap().disassembly, expected);
    }

    #[test]
    fn test_first_test_binary() {
        let data = std::fs::read("./test-bin/test1.bin").unwrap();
        let start_address = Some(0x0000);
        let end_address = None;
        let output = disassemble(
            data.as_slice(),
            start_address,
            end_address,
            BinaryKind::Bin6502,
        );

        // Stolen from https://www.masswerk.at/6502/disassembler.html
        let expected = vec![
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
        ];

        assert_eq!(output.unwrap().disassembly, expected);
    }

    #[test]
    fn test_second_test_binary() {
        let data = std::fs::read("./test-bin/test2.bin").unwrap();
        let start_address = Some(0x0000);
        let end_address = None;
        let output = disassemble(
            data.as_slice(),
            start_address,
            end_address,
            BinaryKind::Bin6502,
        );

        let expected = vec![
            "0x0000 4e 56 ff LSR $ff56",
            "0x0003 f0 48 BEQ $4d",
            "0x0005 e7 ???",
            "0x0006 3e 3c 24 ROL $243c,x",
            "0x0009 6e 00 08 ROR $0800",
            "0x000C 26 6e ROL $6e",
            "0x000E 00 BRK",
            "0x000F 0c ???",
            "0x0010 24 3c BIT $3c",
            "0x0012 00 BRK",
            "0x0013 fc ???",
            "0x0014 44 ???",
            "0x0015 2c 26 3c BIT $3c26",
            "0x0018 00 BRK",
            "0x0019 fc ???",
            "0x001A 44 ???",
            "0x001B 1c ???",
            "0x001C 28 PLP",
            "0x001D 3c ???",
            "0x001E 00 BRK",
            "0x001F df ???",
            "0x0020 f0 9a BEQ $ffbc",
            "0x0022 42 ???",
            "0x0023 2a ROL a",
            "0x0024 00 BRK",
            "0x0025 1f ???",
            "0x0026 61 00 ADC ($00,x)",
            "0x0028 06 0e ASL $0e",
            "0x002A 0c ???",
            "0x002B 6a ROR a",
            "0x002C 00 BRK",
            "0x002D 20 00 1c JSR $1c00",
            "0x0030 66 42 ROR $42",
            "0x0032 2f ???",
            "0x0033 0b ???",
            "0x0034 2f ???",
            "0x0035 0a ASL a",
            "0x0036 61 00 ADC ($00,x)",
            "0x0038 fe 06 12 INC $1206,x",
            "0x003B 00 BRK",
            "0x003C 0c ???",
            "0x003D 00 BRK",
            "0x003E 00 BRK",
            "0x003F ff ???",
            "0x0040 50 8f BVC $ffd1",
            "0x0042 66 12 ROR $12",
            "0x0044 08 PHP",
            "0x0045 2a ROL a",
            "0x0046 00 BRK",
            "0x0047 06 00 ASL $00",
            "0x0049 1e 67 0a ASL $0a67,x",
            "0x004C 15 7c ORA $7c,x",
            "0x004E 00 BRK",
            "0x004F f5 00 SBC $00,x",
            "0x0051 1f ???",
            "0x0052 60 RTS",
            "0x0053 00 BRK",
            "0x0054 04 ???",
            "0x0055 74 ???",
            "0x0056 4a LSR a",
            "0x0057 01 6c ORA ($6c,x)",
            "0x0059 00 BRK",
            "0x005A 04 ???",
            "0x005B 6e 28 43 ROR $4328",
            "0x005E 4e 94 2f LSR $2f94",
            "0x0061 0a ASL a",
            "0x0062 48 PHA",
            "0x0063 6b ???",
            "0x0064 00 BRK",
            "0x0065 a6 61 LDX $61",
            "0x0067 00 BRK",
            "0x0068 06 4a ASL $4a",
            "0x006A 28 PLP",
            "0x006B 42 ???",
            "0x006C 4e 94 50 LSR $5094",
            "0x006F 8f ???",
            "0x0070 60 RTS",
            "0x0071 00 BRK",
            "0x0072 04 ???",
            "0x0073 7e 42 06 ROR $0642,x",
            "0x0076 2a ROL a",
            "0x0077 2a ROL a",
            "0x0078 00 BRK",
            "0x0079 18 CLC",
            "0x007A 60 RTS",
            "0x007B 00 BRK",
            "0x007C 04 ???",
            "0x007D 46 30 LSR $30",
            "0x007F 05 e5 ORA $e5",
            "0x0081 40 RTI",
            "0x0082 28 PLP",
            "0x0083 73 ???",
            "0x0084 00 BRK",
            "0x0085 28 PLP",
            "0x0086 30 2c BMI $b4",
            "0x0088 00 BRK",
            "0x0089 66 48 ROR $48",
            "0x008B c0 46 CPY #$46",
            "0x008D 80 ???",
            "0x008E ca DEX",
            "0x008F 80 ???",
            "0x0090 30 2a BMI $bc",
            "0x0092 00 BRK",
            "0x0093 20 20 6c JSR $6c20",
            "0x0096 00 BRK",
            "0x0097 5e b0 50 LSR $50b0,x",
            "0x009A 67 ???",
            "0x009B 5c ???",
            "0x009C 15 7c ORA $7c,x",
            "0x009E 00 BRK",
            "0x009F f6 00 INC $00,x",
            "0x00A1 1f ???",
            "0x00A2 30 2c BMI $d0",
            "0x00A4 00 BRK",
            "0x00A5 66 48 ROR $48",
            "0x00A7 c0 46 CPY #$46",
            "0x00A9 80 ???",
            "0x00AA c0 aa CPY #$aa",
            "0x00AC 00 BRK",
            "0x00AD 18 CLC",
            "0x00AE 25 40 AND $40",
            "0x00B0 00 BRK",
            "0x00B1 18 CLC",
            "0x00B2 30 2a BMI $de",
            "0x00B4 00 BRK",
            "0x00B5 1c ???",
            "0x00B6 d0 40 BNE $f8",
            "0x00B8 30 3b BMI $f5",
            "0x00BA 00 BRK",
            "0x00BB 02 ???",
            "0x00BC 4e fb 00 LSR $00fb",
            "0x00BF 02 ???",
        ];

        assert_eq!(output.unwrap().disassembly, expected);
    }
}
