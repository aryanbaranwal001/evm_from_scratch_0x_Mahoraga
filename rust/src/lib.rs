use primitive_types::U256;
use hex::ToHex;


pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

pub fn evm(_code: impl AsRef<[u8]>) -> EvmResult {
    let mut stack: Vec<U256> = Vec::new();
    let mut pc = 0;

    let code = _code.as_ref();

    while pc < code.len() {
        let opcode = code[pc];
        pc += 1;

        if opcode == 0x00 {
            // STOP
            break;
        }

        if opcode == 0x5f {
            // PUSH0
            stack.push(U256([0, 0, 0, 0]));
        }

        if opcode == 0x60 {
            println!("Code in hex: {}", code.encode_hex::<String>());
            
        }






        // if opcode == 0x60 {
        //     // PUSH0
        //     stack.push(U256([0, 0, 0, 0]));
        // }
        // if opcode == 0x5f {
        //     // PUSH0
        //     stack.push(U256([0, 0, 0, 0]));
        // }
        // if opcode == 0x5f {
        //     // PUSH0
        //     stack.push(U256([0, 0, 0, 0]));
        // }
        // if opcode == 0x5f {
        //     // PUSH0
        //     stack.push(U256([0, 0, 0, 0]));
        // }
        // if opcode == 0x5f {
        //     // PUSH0
        //     stack.push(U256([0, 0, 0, 0]));
        // }
    }

    return EvmResult {
        stack: stack,
        success: true,
    };
}
