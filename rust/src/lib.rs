use primitive_types::U256;
use hex::ToHex;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

pub fn evm(_code: impl AsRef<[u8]>) -> EvmResult {
    let mut stack: Vec<U256> = Vec::new();
    let mut pc = 0;
    let mut jpc = 0;

    let code = _code.as_ref();

    let mut jump_arr: Vec<u32> = Vec::new();

    // memory start
    let mut memory_m: Vec<u8> = Vec::new();

    // memory end

    // jump thing start
    while jpc < code.len() {
        let opcodej = code[jpc];

        if opcodej == 0x5b {
            jump_arr.push(jpc as u32);
        }

        if 0x60 <= opcodej && opcodej <= 0x7f {
            let size = opcodej - 0x60 + 0x01;
            jpc += size as usize;
        }

        jpc += 1;
    }

    fn check_valid_jump_location(location: u32, jump_arr: &Vec<u32>) -> bool {
        jump_arr.contains(&location)
    }
    // jump thing end

    // main code base
    while pc < code.len() {
        // check all the opcodes and update the pc accn
        // @i there is in built overflowing add, sub, mul, div, etc
        // @i YOU MUST UNDERSTAND THAT PUSH9->PUSH32 CODE
        // MUUUUUUUUUUSSSSSSSSSSSSSSSSSSSSSSTTTTTTTTTTTTTTTTTT
        // MUUUUUUUUUUSSSSSSSSSSSSSSSSSSSSSSTTTTTTTTTTTTTTTTTT
        // MUUUUUUUUUUSSSSSSSSSSSSSSSSSSSSSSTTTTTTTTTTTTTTTTTT
        // MUUUUUUUUUUSSSSSSSSSSSSSSSSSSSSSSTTTTTTTTTTTTTTTTTT

        let opcode = code[pc];

        // ----------------------------------------------------------------------//
        // ----------------------------------------------------------------------//
        fn expand_memory_to_32_byte_chunks(bytes_needed: usize) -> usize {
            ((bytes_needed + 31) / 32) * 32
        }

        // MLOAD
        if opcode == 0x51 {
            let memory_address = stack.remove(0);
            let address = memory_address.as_usize();

            let bytes_needed = address + 32;
            let memory_size_needed = expand_memory_to_32_byte_chunks(bytes_needed);

            if memory_m.len() < memory_size_needed {
                memory_m.resize(memory_size_needed, 0);
            }

            let mut data = [0u8; 32];

            for i in 0..32 {
                let memory_position = address + i;
                data[i] = memory_m[memory_position];
            }

            let number = U256::from_big_endian(&data);
            stack.insert(0, number);
        }

        // MSTORE
        if opcode == 0x52 {
            let memory_address = stack.remove(0);
            let value_to_store = stack.remove(0);
            let address = memory_address.as_usize();

            let bytes_needed = address + 32;
            let memory_size_needed = expand_memory_to_32_byte_chunks(bytes_needed);

            if memory_m.len() < memory_size_needed {
                memory_m.resize(memory_size_needed, 0);
            }

            let mut bytes = [0u8; 32];

            // I need to understand this in depth
            value_to_store.to_big_endian(&mut bytes);
            
            for i in 0..32 {
                memory_m[address + i] = bytes[i];
            }
        }

        // MSTORE8
        if opcode == 0x53 {
            let memory_address = stack.remove(0);
            let value_to_store = stack.remove(0);
            let address = memory_address.as_usize();

            let bytes_needed = address + 1;
            let memory_size_needed = expand_memory_to_32_byte_chunks(bytes_needed);

            if memory_m.len() < memory_size_needed {
                memory_m.resize(memory_size_needed, 0);
            }

            let single_byte = (value_to_store.low_u64() & 0xff) as u8;

            memory_m[address] = single_byte;
        }

        // MSIZE
        if opcode == 0x59 {

            let current_size = memory_m.len();
            let size_as_number = U256::from(current_size);

            stack.insert(0, size_as_number);
        }

        // ===============================================
        // EXAMPLE WALKTHROUGH OF THE FAILING TEST:
        // ===============================================
        //
        // Test: PUSH1 0x39, MLOAD, POP, MSIZE
        //
        // 1. PUSH1 0x39 → Stack: [57]
        // 2. MLOAD → Need to read 32 bytes starting at address 57
        //    → Bytes needed: 57 + 32 = 89
        //    → Memory expansion: (89 + 31) / 32 * 32 = 96 bytes
        //    → Memory is now 96 bytes long
        //    → Stack: [0] (32 zero bytes as a number)
        // 3. POP → Stack: []
        // 4. MSIZE → Stack: [96]
        //    → 96 in hex is 0x60 ✓
        // ===============================================

        // JUMPI
        if opcode == 0x57 {
            let index = stack.remove(0).0[0];
            let bool_ean = stack.remove(0).bit(0);
            let temp_pc = index as usize;

            let valid_position = check_valid_jump_location(temp_pc as u32, &jump_arr);

            if bool_ean == true {
                if valid_position == true {
                    pc = index as usize;
                    pc += 1;
                    continue;
                } else {
                    return EvmResult {
                        stack: stack,
                        success: false,
                    };
                }
            } else {
                pc += 1;
                code[pc];
                continue;
            }
        }

        // JUMP
        if opcode == 0x56 {
            let index = stack.remove(0).0[0];
            pc = index as usize;

            let valid_position = check_valid_jump_location(pc as u32, &jump_arr);

            if code[pc] == 0x5b && valid_position == true {
                pc += 1;
                continue;
            } else {
                return EvmResult {
                    stack: stack,
                    success: false,
                };
            }
        }

        // GAS
        if opcode == 0x5a {
            stack.insert(0, U256::MAX);
        }

        // PC
        if opcode == 0x58 {
            // what if pc is big number ??
            stack.insert(0, U256::from(pc));
        }

        // INVALID
        if opcode == 0xfe {
            return EvmResult {
                stack: stack,
                success: false,
            };
        }

        // SWAP ALL IN ONE
        if 0x90 <= opcode && opcode <= 0x9f {
            let index = opcode - 0x90;

            let first = stack[0].clone();
            let second = stack[(index + 1) as usize].clone();

            stack[0] = second;
            stack[(index + 1) as usize] = first;
        }

        // DUP ALL IN ONE
        if (opcode & 0xf0) == 0x80 {
            let index = opcode - 0x80;
            let first = stack[index as usize].clone();

            stack.insert(0, first);
        }

        // BYTE
        if opcode == 0x1a {
            let i = stack.remove(0);
            let value = stack.remove(0);

            let result = if i >= U256::from(32) {
                U256::zero()
            } else {
                let byte_value =
                    (value >> (U256::from(8) * (U256::from(31) - i))) & U256::from(0xff);
                byte_value
            };

            stack.insert(0, result);
        }

        // SAR
        if opcode == 0x1d {
            let shift = stack.remove(0);
            let value = stack.remove(0);

            let value_negative = value.bit(255);

            let result = if value_negative == false {
                if shift >= U256::from(256) { U256::zero() } else { value >> shift }
            } else {
                if shift >= U256::from(256) {
                    U256::MAX
                } else {
                    let shifted = value >> shift;
                    let mask = U256::MAX << (U256::from(256) - shift);
                    mask | shifted
                }
            };

            stack.insert(0, result);
        }

        // SHR
        if opcode == 0x1c {
            let shift = stack.remove(0);
            let value = stack.remove(0);

            let result = if shift >= U256::from(256) { U256::zero() } else { value >> shift };

            stack.insert(0, result);
        }

        // SHL
        if opcode == 0x1b {
            let shift = stack.remove(0);
            let value = stack.remove(0);

            let result = if shift >= U256::from(256) { U256::zero() } else { value << shift };

            stack.insert(0, result);
        }

        // NOT
        if opcode == 0x19 {
            let first = stack.remove(0);
            let result = !first;
            stack.insert(0, result);
        }

        // AND
        if opcode == 0x16 {
            let first = stack.remove(0);
            let second = stack.remove(0);
            let result = first & second;
            stack.insert(0, result);
        }

        // OR
        if opcode == 0x17 {
            let first = stack.remove(0);
            let second = stack.remove(0);
            let result = first | second;
            stack.insert(0, result);
        }

        // XOR
        if opcode == 0x18 {
            let first = stack.remove(0);
            let second = stack.remove(0);
            let result = first ^ second;
            stack.insert(0, result);
        }

        // ISZERO
        if opcode == 0x15 {
            let first = stack.remove(0);

            if first != U256::zero() {
                stack.insert(0, U256::zero());
            } else {
                stack.insert(0, U256::one());
            }
        }

        // EQ
        if opcode == 0x14 {
            let first = stack.remove(0);
            let second = stack.remove(0);

            let result = if first == second { U256::one() } else { U256::zero() };

            stack.insert(0, result);
        }

        // SLT
        if opcode == 0x12 {
            let first = stack.remove(0);
            let second = stack.remove(0);

            let first_negative = first.bit(255);
            let second_negative = second.bit(255);

            let result = match (first_negative, second_negative) {
                (true, false) => U256::one(),
                (false, true) => U256::zero(),
                _ => {
                    // use unsigned comparison is possible !!!
                    if first < second {
                        U256::one()
                    } else {
                        U256::zero()
                    }
                }
            };

            stack.insert(0, result);
        }

        // SGT
        if opcode == 0x13 {
            let first = stack.remove(0);
            let second = stack.remove(0);

            let first_negative = first.bit(255);
            let second_negative = second.bit(255);

            let result = match (first_negative, second_negative) {
                (false, true) => U256::one(),
                (true, false) => U256::zero(),
                _ => {
                    if first > second { U256::one() } else { U256::zero() }
                }
            };

            stack.insert(0, result);
        }

        // LT
        if opcode == 0x10 {
            let first = stack.remove(0);
            let second = stack.remove(0);

            let result = if first < second { U256::one() } else { U256::zero() };

            stack.insert(0, result);
        }

        // GT
        if opcode == 0x11 {
            let first = stack.remove(0);
            let second = stack.remove(0);

            let result = if first > second { U256::one() } else { U256::zero() };

            stack.insert(0, result);
        }

        if opcode == 0x07 {
            let first = stack.remove(0);
            let second = stack.remove(0);

            if second == U256::zero() {
                stack.insert(0, U256::zero());
                return EvmResult {
                    stack: stack,
                    success: true,
                };
            }

            let first_negative = first.bit(255);
            let second_negative = second.bit(255);

            let abs_first = if first_negative {
                (!first).overflowing_add(U256::one()).0
            } else {
                first
            };

            let abs_second = if second_negative {
                (!second).overflowing_add(U256::one()).0
            } else {
                second
            };

            let remainder = abs_first % abs_second;

            let result = if first_negative {
                (!remainder).overflowing_add(U256::one()).0
            } else {
                remainder
            };

            stack.insert(0, result);
        }

        if opcode == 0x05 {
            let first = stack.remove(0);
            let second = stack.remove(0);

            let result = if second == U256::zero() {
                U256::zero()
            } else {
                let a_negative = first.bit(255);
                let b_negative = second.bit(255);

                let a_abs = if a_negative {
                    (!first).overflowing_add(U256::one()).0
                } else {
                    first
                };

                let b_abs = if b_negative {
                    (!second).overflowing_add(U256::one()).0
                } else {
                    second
                };

                let quotient = a_abs / b_abs;

                let result_negative = a_negative ^ b_negative;

                if result_negative {
                    (!quotient).overflowing_add(U256::one()).0
                } else {
                    quotient
                }
            };

            stack.insert(0, result);
        }

        if opcode == 0x0b {
            // @i try to do this with whole u64; 4
            let push1_opcode_value = stack.remove(0).0[0];
            let value = stack.remove(0).0[0];

            // @i assuming its always push1
            if push1_opcode_value == 0x00 {
                if (value & 0x80) != 0 {
                    let extended = U256::MAX - U256::from(255 - value);
                    stack.push(extended);
                } else {
                    stack.push(U256::from(value));
                }
            }
        }

        if opcode == 0x0a {
            let base = stack.remove(0).0[0];
            let exp = stack.remove(0).0[0];

            // @i u are not using whole all of the numbers

            let mut num = base;
            for _i in 0..exp - 1 {
                num *= base;
            }

            // @i there is a flaw here

            stack.insert(0, U256::from(num));
        }

        // MULMOD or MULLMOD(wrapped)
        if opcode == 0x09 {
            let first = stack.remove(0);
            let second = stack.remove(0);
            let third = stack.remove(0);

            // @i see exactly how the fuck is this mod thingy implemented
            let result_arr = ((first % third) * (second % third)) % third;

            stack.insert(0, result_arr);
        }

        // ADDMOD or ADDMOD(wrapped)
        if opcode == 0x08 {
            let first = stack.remove(0);
            let second = stack.remove(0);
            let third = stack.remove(0);

            let first_arr = first.0;
            let second_arr = second.0;

            let mut result_arr = [0u64; 4];
            let mut carry = 0u64;

            for i in 0..4 {
                let sum = (first_arr[i] as u128) + (second_arr[i] as u128) + (carry as u128);

                result_arr[i] = sum as u64;
                carry = (sum >> 64) as u64;
            }

            let sum = U256(result_arr);

            let mod_value = sum % third;
            stack.insert(0, mod_value);
        }

        // MOD
        if opcode == 0x06 {
            let first = stack.remove(0);
            let second = stack.remove(0);

            // first / second

            // @i implement this in your own way

            if second == U256::zero() {
                stack.insert(0, U256::zero());
            } else {
                let div = first % second;

                stack.insert(0, div);
            }
        }

        // DIV
        if opcode == 0x04 {
            let first = stack.remove(0);
            let second = stack.remove(0);

            // @i implement this in your own way

            if second == U256::zero() {
                stack.insert(0, U256::zero());
            } else {
                let div = first / second;

                stack.insert(0, div);
            }
        }

        // SUB or SUB (underflow)
        if opcode == 0x03 {
            let first = stack.remove(0);
            let second = stack.remove(0);

            let first_arr = first.0;
            let second_arr = second.0;

            // first - second

            let mut result_arr = [0u128; 4];

            for i in 0..4 {
                let mod_first_element_arry = (first_arr[i] as u128) + (0x01 << 64);
                result_arr[i] = mod_first_element_arry - (second_arr[i] as u128);

                // println!("mod_first_element_arry {:02x?}", mod_first_element_arry);
                // println!("seoncd_arr {:02x?}", (second_arr[i] as u128));
                // println!("result_arr {:02x?}", result_arr[i]);
            }

            // println!("{:02x?}", result_arr);

            let mut borrow: u128 = 0;

            let mut final_result_arr = [0u128; 4];

            for i in 0..4 {
                final_result_arr[i] = result_arr[i] - borrow;

                // println!("resularry{i} {:02x?}", result_arr[i]);

                // println!("final_result_arr {:02x?}", final_result_arr[i]);
                // println!("-------------------------");
                // println!("final_result_arr {:02x?}", ((result_arr[i]) - (borrow as u128)) as u128);

                borrow = final_result_arr[i] >> 64;

                // println!("borrow {:02x?}", borrow);

                if borrow == 0 {
                    borrow = 1;
                } else if borrow == 1 {
                    borrow = 0;
                }
            }

            let mut finalf_result_arr = [0u64; 4];

            for k in 0..4 {
                finalf_result_arr[k] = final_result_arr[k] as u64;
            }

            stack.insert(0, U256(finalf_result_arr));
        }

        // MUL or MUL (overflow)
        if opcode == 0x02 {
            let first = stack.remove(0);
            let second = stack.remove(0);

            let first_arr = first.0;
            let second_arr = second.0;

            // println!("{:02x?}", first_arr);
            // println!("{:02x?}", second_arr);

            let mut temp_result = [0u128; 7];

            for i in 0..4 {
                for j in 0..4 {
                    let product = (first_arr[i] as u128) * (second_arr[j] as u128);
                    temp_result[i + j] += product;
                }
            }
            // println!("{:02x?}", temp_result);

            let mut borrow: u64 = 0;
            for i in 0..7 {
                temp_result[i] = temp_result[i] + (borrow as u128);
                borrow = (temp_result[i] >> 64) as u64;
                // println!("borrow {:02x?}", borrow);
            }
            // println!("{:02x?}", temp_result);

            let mut result_arr = [0u64; 4];
            for i in 0..4 {
                result_arr[i] = temp_result[i] as u64;
            }

            let result_num = U256(result_arr);
            stack.insert(0, result_num);
        }

        // ADD or ADD (overflow)
        if opcode == 0x01 {
            let first = stack.remove(0);
            let second = stack.remove(0);

            let first_arr = first.0;
            let second_arr = second.0;

            let mut result_arr = [0u64; 4];
            let mut carry = 0u64;

            for i in 0..4 {
                let sum = (first_arr[i] as u128) + (second_arr[i] as u128) + (carry as u128);

                result_arr[i] = sum as u64;
                carry = (sum >> 64) as u64;
            }

            let sum = U256(result_arr);

            stack.insert(0, sum);
        }

        if opcode == 0x50 {
            stack.remove(0);
        }

        // PUSH9 --> PUSH32
        if 0x68 <= opcode && opcode <= 0x7f {
            let size = opcode - 0x60 + 0x01;

            let mut arr: [u64; 4] = [0, 0, 0, 0];

            // Read all bytes and place them right-aligned (least significant bits)
            for i in 0..size {
                pc += 1;
                let byte = code[pc] as u64;
                let bit_position = (size - 1 - i) * 8;
                let arr_index = (bit_position / 64) as usize;
                let bit_shift = bit_position % 64;
                arr[arr_index] |= byte << bit_shift;
            }

            stack.insert(0, U256(arr));
        }

        // PUSH1 --> PUSH8
        if 0x60 <= opcode && opcode <= 0x67 {
            let size = opcode - 0x60 + 0x01;

            let mut push2_data: u64 = 0;
            for i in 0..size {
                pc += 1;
                push2_data += (code[pc] as u64) << ((size - i - 1) * 8);
            }

            let mut arr: [u64; 4] = [0, 0, 0, 0];

            arr[0] = push2_data as u64;

            stack.insert(0, U256(arr));
        }

        // PUSH0
        if opcode == 0x5f {
            stack.push(U256([0, 0, 0, 0]));
        }

        // STOP
        if opcode == 0x00 {
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
