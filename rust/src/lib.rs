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





        
        if opcode == 0x6a {
            // PUSH11
            // @i see the hint and try to do something with it
            let size = opcode - 0x60 + 0x01;
            // println!("size 0x{:02X}", size);

            let mut push_data: u64 = 0;

            for j in 0..3 {
                pc += 1;
                // println!("number {}", (size - 2 - i - 1) * 8);
                push_data += (code[pc] as u64) << ((2 - j) * 8);
                // println!("first 0x{:02X}", push_data);
            }

            let mut arr: [u64; 4] = [0, 0, 0, 0];

            arr[1] = push_data as u64;
            push_data = 0;

            for i in 0..size - 3 {
                pc += 1;
                // println!("number {}", (size - 2 - i - 1) * 8);
                push_data += (code[pc] as u64) << ((size - 2 - i - 1 - 1) * 8);
                // println!("first 0x{:02X}", push_data);
            }
            arr[0] = push_data as u64;
            stack.push(U256(arr));
        }

        if opcode == 0x69 {
            // PUSH10
            let size = opcode - 0x60 + 0x01;
            // println!("size 0x{:02X}", size);

            let mut push_data: u64 = 0;

            for j in 0..2 {
                pc += 1;
                // println!("number {}", (size - 2 - i - 1) * 8);
                push_data += (code[pc] as u64) << ((1 - j) * 8);
                // println!("first 0x{:02X}", push_data);
            }

            let mut arr: [u64; 4] = [0, 0, 0, 0];

            arr[1] = push_data as u64;
            push_data = 0;

            for i in 0..size - 2 {
                pc += 1;
                // println!("number {}", (size - 2 - i - 1) * 8);
                push_data += (code[pc] as u64) << ((size - 2 - i - 1) * 8);
                // println!("first 0x{:02X}", push_data);
            }
            arr[0] = push_data as u64;
            stack.push(U256(arr));
        }

        if opcode == 0x65 {
            // PUSH2

            // @q how exactly bytes, decimals, etc types getting converted
            // and how do they flow, like code, as_ref() etc
            // @i (improvement) implement the for loop
            // @i at the end of every opcode, you can pc + (amount you used
            // in that if block)

            let mut push2_data = (code[pc + 1] as u64) << 40;
            let push2_data2 = (code[pc + 2] as u64) << 32;
            let push2_data3 = (code[pc + 3] as u64) << 24;
            let push2_data4 = (code[pc + 4] as u64) << 16;
            let push2_data5 = (code[pc + 5] as u64) << 8;
            let push2_data6 = code[pc + 6] as u64;

            push2_data += push2_data2 as u64;
            push2_data += push2_data3 as u64;
            push2_data += push2_data4 as u64;
            push2_data += push2_data5 as u64;
            push2_data += push2_data6 as u64;

            let mut arr: [u64; 4] = [0, 0, 0, 0];

            arr[0] = push2_data as u64;

            stack.push(U256(arr));
        }

        if opcode == 0x63 {
            // PUSH2

            let mut push2_data = (code[pc + 1] as u32) << 24;
            let push2_data2 = (code[pc + 2] as u32) << 16;
            let push2_data3 = (code[pc + 3] as u32) << 8;
            let push2_data4 = code[pc + 4] as u32;

            push2_data += push2_data2 as u32;
            push2_data += push2_data3 as u32;
            push2_data += push2_data4 as u32;

            let mut arr: [u64; 4] = [0, 0, 0, 0];

            arr[0] = push2_data as u64;

            stack.push(U256(arr));
        }

        if opcode == 0x61 {
            // PUSH2

            let mut push2_data = (code[pc + 1] as u16) << 8;
            let push2_data2 = code[pc + 2];
            push2_data += push2_data2 as u16;

            let mut arr: [u64; 4] = [0, 0, 0, 0];

            arr[0] = push2_data as u64;

            stack.push(U256(arr));
        }

        if opcode == 0x60 {
            // PUSH1
            let push1_data = code[pc + 1];

            let mut arr: [u64; 4] = [0, 0, 0, 0];

            arr[0] = push1_data as u64;

            stack.push(U256(arr));
        }

        if opcode == 0x5f {
            // PUSH0
            stack.push(U256([0, 0, 0, 0]));
        }

        if opcode == 0x00 {
            // STOP
            break;
        }

        // program counter updates at the end of the all if statements
        pc += 1;
    }

    return EvmResult {
        stack: stack,
        success: true,
    };
}
