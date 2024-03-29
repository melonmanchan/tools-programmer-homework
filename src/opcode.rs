use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct OpCode {
    pub instructions: String,
    is_relative: Option<bool>,
}

lazy_static! {
    pub static ref INSTRUCTION_MAP: HashMap<u8, OpCode> = create_instruction_map();
}

// TODO: Add rest of instructions, look into static hashmap
// https://docs.rs/phf/latest/phf/
fn create_instruction_map() -> HashMap<u8, OpCode> {
    let mut map: HashMap<u8, OpCode> = HashMap::new();

    map.insert(
        0x10,
        OpCode {
            instructions: "bpl $hh".into(),
            is_relative: Some(true),
        },
    );

    map.insert(
        0x11,
        OpCode {
            instructions: "ora ($hh),y".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x15,
        OpCode {
            instructions: "ora $hh,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x16,
        OpCode {
            instructions: "asl $hh,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x18,
        OpCode {
            instructions: "clc".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x19,
        OpCode {
            instructions: "ora $hhll,y".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x20,
        OpCode {
            instructions: "jsr $hhll".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x21,
        OpCode {
            instructions: "and ($hh,x)".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x24,
        OpCode {
            instructions: "bit $hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x25,
        OpCode {
            instructions: "and $hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x26,
        OpCode {
            instructions: "rol $hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x28,
        OpCode {
            instructions: "plp".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x29,
        OpCode {
            instructions: "and #$hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x30,
        OpCode {
            instructions: "bmi $hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x31,
        OpCode {
            instructions: "and ($hh),y".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x35,
        OpCode {
            instructions: "and $hh,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x36,
        OpCode {
            instructions: "rol $hh,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x38,
        OpCode {
            instructions: "sec".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x39,
        OpCode {
            instructions: "and $hhll,y".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x40,
        OpCode {
            instructions: "rti".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x41,
        OpCode {
            instructions: "eor ($hh,x)".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x45,
        OpCode {
            instructions: "eor $hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x46,
        OpCode {
            instructions: "lsr $hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x48,
        OpCode {
            instructions: "pha".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x49,
        OpCode {
            instructions: "eor #$hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x50,
        OpCode {
            instructions: "bvc $hh".into(),
            is_relative: Some(true),
        },
    );

    map.insert(
        0x51,
        OpCode {
            instructions: "eor ($hh),y".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x55,
        OpCode {
            instructions: "eor $hh,y".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x56,
        OpCode {
            instructions: "lsr $hh,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x58,
        OpCode {
            instructions: "cli".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x59,
        OpCode {
            instructions: "eor $hhll,y".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x60,
        OpCode {
            instructions: "rts".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x61,
        OpCode {
            instructions: "adc ($hh,x)".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x65,
        OpCode {
            instructions: "adc $hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x66,
        OpCode {
            instructions: "ror $hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x68,
        OpCode {
            instructions: "pla".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x69,
        OpCode {
            instructions: "adc #$hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x70,
        OpCode {
            instructions: "bvs $hh".into(),
            is_relative: Some(true),
        },
    );

    map.insert(
        0x71,
        OpCode {
            instructions: "adc ($hh),y".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x75,
        OpCode {
            instructions: "adc $hh,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x76,
        OpCode {
            instructions: "ror $hh,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x78,
        OpCode {
            instructions: "sei".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x79,
        OpCode {
            instructions: "adc $hhll,y".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x81,
        OpCode {
            instructions: "sta ($hh,x)".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x84,
        OpCode {
            instructions: "sty $hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x85,
        OpCode {
            instructions: "sta $hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x86,
        OpCode {
            instructions: "stx $hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x88,
        OpCode {
            instructions: "dey".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x90,
        OpCode {
            instructions: "bcc $hh".into(),
            is_relative: Some(true),
        },
    );

    map.insert(
        0x91,
        OpCode {
            instructions: "sta ($hh),y".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x94,
        OpCode {
            instructions: "sty $hh,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x95,
        OpCode {
            instructions: "sta $hh,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x96,
        OpCode {
            instructions: "stx $hh,y".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x98,
        OpCode {
            instructions: "tya".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x99,
        OpCode {
            instructions: "sta $hhll,y".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x00,
        OpCode {
            instructions: "brk".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x01,
        OpCode {
            instructions: "ora ($hh,x)".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x05,
        OpCode {
            instructions: "ora $hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x06,
        OpCode {
            instructions: "asl $hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x07,
        OpCode {
            instructions: "slo $hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x08,
        OpCode {
            instructions: "php".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x09,
        OpCode {
            instructions: "ora #$hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x0a,
        OpCode {
            instructions: "asl".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x0d,
        OpCode {
            instructions: "ora $hhll".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x0e,
        OpCode {
            instructions: "asl $hhll".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x1d,
        OpCode {
            instructions: "ora $hhll,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x1e,
        OpCode {
            instructions: "asl $hhll,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x2a,
        OpCode {
            instructions: "rol".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x2c,
        OpCode {
            instructions: "bit $hhll".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x2d,
        OpCode {
            instructions: "and $hhll".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x2e,
        OpCode {
            instructions: "rol $hhll".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x3e,
        OpCode {
            instructions: "rol $hhll,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x4a,
        OpCode {
            instructions: "lsr".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x4c,
        OpCode {
            instructions: "jmp $hhll".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x4d,
        OpCode {
            instructions: "eor $hhll".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x4e,
        OpCode {
            instructions: "lsr $hhll".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x5d,
        OpCode {
            instructions: "eor $hhll,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x5e,
        OpCode {
            instructions: "lsr $hhll,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x6a,
        OpCode {
            instructions: "ror".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x6c,
        OpCode {
            instructions: "jmp ($hhll)".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x6d,
        OpCode {
            instructions: "adc $hhll".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x6e,
        OpCode {
            instructions: "ror $hhll".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x7d,
        OpCode {
            instructions: "abc $hhll,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x7e,
        OpCode {
            instructions: "ror $hhll,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x8a,
        OpCode {
            instructions: "txa".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x8c,
        OpCode {
            instructions: "sty $hhll".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x8d,
        OpCode {
            instructions: "sta $hhll".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x8e,
        OpCode {
            instructions: "stx $hhll".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x9a,
        OpCode {
            instructions: "txs".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x9d,
        OpCode {
            instructions: "txs".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xa0,
        OpCode {
            instructions: "ldy #$hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xa1,
        OpCode {
            instructions: "lda ($hh,x)".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xa2,
        OpCode {
            instructions: "ldx #$hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xa4,
        OpCode {
            instructions: "ldy $hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xa5,
        OpCode {
            instructions: "lda $hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xa6,
        OpCode {
            instructions: "ldx $hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xa8,
        OpCode {
            instructions: "tay".into(),
            is_relative: None,
        },
    );
    map.insert(
        0xa9,
        OpCode {
            instructions: "lda #$hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xaa,
        OpCode {
            instructions: "tax".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xac,
        OpCode {
            instructions: "ldy $hhll".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xad,
        OpCode {
            instructions: "lda $hhll".into(),
            is_relative: None,
        },
    );
    map.insert(
        0xae,
        OpCode {
            instructions: "ldx $hhll".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xb0,
        OpCode {
            instructions: "bcs $hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xb1,
        OpCode {
            instructions: "lda $(hh),y".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xb4,
        OpCode {
            instructions: "ldy $hh,y".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xb5,
        OpCode {
            instructions: "lda $hh,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xb6,
        OpCode {
            instructions: "ldx $hh,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xb8,
        OpCode {
            instructions: "clv".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xb9,
        OpCode {
            instructions: "lda $hhll,y".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xba,
        OpCode {
            instructions: "tsx".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xbc,
        OpCode {
            instructions: "ldy $hhll,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xbd,
        OpCode {
            instructions: "lda $hhll,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xbe,
        OpCode {
            instructions: "ldx $hhll,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xc0,
        OpCode {
            instructions: "cpy #$hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xc1,
        OpCode {
            instructions: "cmp ($hh,x)".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xc4,
        OpCode {
            instructions: "cpy $hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xc5,
        OpCode {
            instructions: "cmp $hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xc6,
        OpCode {
            instructions: "dec $hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xc8,
        OpCode {
            instructions: "iny".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xc9,
        OpCode {
            instructions: "cmp #$hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xca,
        OpCode {
            instructions: "dex".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xcc,
        OpCode {
            instructions: "cpy $hhll".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xcd,
        OpCode {
            instructions: "cmp $hhll".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xce,
        OpCode {
            instructions: "dec $hhll".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xd0,
        OpCode {
            instructions: "bne $hh".into(),
            is_relative: Some(true),
        },
    );

    map.insert(
        0xd1,
        OpCode {
            instructions: "cmp ($hh),y".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xd5,
        OpCode {
            instructions: "cmp $hh,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xd6,
        OpCode {
            instructions: "dec $hh,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xd8,
        OpCode {
            instructions: "cld".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xd9,
        OpCode {
            instructions: "cmp $hhll,y".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xdd,
        OpCode {
            instructions: "cmp $hhll,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xde,
        OpCode {
            instructions: "dec $hhll,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xe0,
        OpCode {
            instructions: "cpx #$hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xe1,
        OpCode {
            instructions: "sbc ($hh,x)".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xe4,
        OpCode {
            instructions: "cpx $hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xe5,
        OpCode {
            instructions: "sbc $hh)".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xe6,
        OpCode {
            instructions: "inc $hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xe8,
        OpCode {
            instructions: "inx".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xe9,
        OpCode {
            instructions: "sbc #$hh".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xea,
        OpCode {
            instructions: "nop".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xec,
        OpCode {
            instructions: "cpx $hhll".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xed,
        OpCode {
            instructions: "sbc $hhll".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xee,
        OpCode {
            instructions: "inc $hhll".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xf0,
        OpCode {
            instructions: "beq $hh".into(),
            is_relative: Some(true),
        },
    );

    map.insert(
        0xf1,
        OpCode {
            instructions: "sbc ($hh),y".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xf5,
        OpCode {
            instructions: "sbc $hh,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xf6,
        OpCode {
            instructions: "inc $hh,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xf8,
        OpCode {
            instructions: "sed".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xf9,
        OpCode {
            instructions: "sbc $hhll,y".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xfd,
        OpCode {
            instructions: "sbc $hhll,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xfe,
        OpCode {
            instructions: "inc $hhll,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0x3d,
        OpCode {
            instructions: "and $hhll,x".into(),
            is_relative: None,
        },
    );

    map.insert(
        0xa9,
        OpCode {
            instructions: "lda".to_string(),
            is_relative: None,
        },
    );

    map
}
