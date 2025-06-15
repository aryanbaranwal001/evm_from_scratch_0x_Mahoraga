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

        if opcode == 0x00 {
            // STOP
            break;
        }

        if opcode == 0x5f {
            // PUSH0
            stack.push(U256([0, 0, 0, 0]));
        }

        if opcode == 0x60 {
            let push1_data = code[pc + 1];


            let mut arr: [u64; 4] = [127, 121, 121, 121];

            // arr[3] = push1_data as u64;

            // println!(
            //     "------------------- \n code in hex: 0x{:02}\n-------------------",
            //     push1_data
            // );

            stack.push(U256(arr));
            stack.push(U256(arr));

            println!(
                "------------------- \n code in hex: {}\n-------------------",
                code.encode_hex::<String>()
            );
        }

        // program counter updates at the end of the all if statements
        pc += 1;
    }

    return EvmResult {
        stack: stack,
        success: true,
    };
}
