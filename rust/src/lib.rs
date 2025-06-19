use primitive_types::U256;
// use hex::ToHex;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

pub fn evm(_code: impl AsRef<[u8]>) -> EvmResult {
    let mut stack: Vec<U256> = Vec::new();
    let mut pc = 0;

    let code = _code.as_ref();

    while pc < code.len() {
        // check all the opcodes and update the pc accn
        // @i there is in built overflowing add, sub, mul, div, etc
        let opcode = code[pc];

        // ----------------------------------------------------------------------//
        // ----------------------------------------------------------------------//

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

        if opcode == 0x09 {
            // MULMOD or MULLMOD(wrapped)

            let first = stack.remove(0);
            let second = stack.remove(0);
            let third = stack.remove(0);

            // @i see exactly how the fuck is this mod thingy implemented
            let result_arr = ((first % third) * (second % third)) % third;

            stack.insert(0, result_arr);
        }

        if opcode == 0x08 {
            // ADDMOD or ADDMOD(wrapped)
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

        if opcode == 0x06 {
            // MOD

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

        if opcode == 0x04 {
            // DIV
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

        if opcode == 0x03 {
            // SUB or SUB (underflow)
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

        if opcode == 0x02 {
            // MUL or MUL (overflow)
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

        if opcode == 0x01 {
            // ADD or ADD (overflow)
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

        // PUSH32
        if opcode == 0x7f {
            let size = opcode - 0x60 + 0x01;
            // println!("size 0x{:02X}", size);
            let mut arr: [u64; 4] = [0, 0, 0, 0];

            let mut push32_quater_data: u64 = 0;

            for j in 0..size {
                pc += 1;
                push32_quater_data += (code[pc] as u64) << ((8 - ((j + 1) % 8)) * 8) % 64;

                if (j + 1) % 8 == 0 {
                    // println!("first 0x{:02X}", push32_quater_data);
                    arr[(4 - (j + 1) / 8) as usize] = push32_quater_data as u64;
                    push32_quater_data = 0;
                }
            }

            stack.insert(0, U256(arr));
        }

        // PUSH11
        if opcode == 0x6a {
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
            stack.insert(0, U256(arr));
        }

        // PUSH10
        if opcode == 0x69 {
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
            stack.insert(0, U256(arr));
        }

        // PUSH6
        if opcode == 0x65 {
            // @q how exactly bytes, decimals, etc types getting converted
            // and how do they flow, like code, as_ref() etc
            // @i (improvement) implement the for loop
            // @i at the end of every opcode, you can pc + (amount you used
            // in that if block)

            let mut push2_data: u64 = 0;
            for i in 0..6 {
                pc += 1;
                push2_data += (code[pc] as u64) << ((6 - i - 1) * 8);
                println!("first 0x{:02X}", push2_data);
            }

            let mut arr: [u64; 4] = [0, 0, 0, 0];

            arr[0] = push2_data as u64;

            stack.push(U256(arr));
        }

        // PUSH4
        if opcode == 0x63 {
            let mut push2_data: u32 = 0;
            for i in 0..4 {
                pc += 1;
                push2_data += (code[pc] as u32) << ((4 - i - 1) * 8);
            }

            let mut arr: [u64; 4] = [0, 0, 0, 0];

            arr[0] = push2_data as u64;

            stack.insert(0, U256(arr));
            pc += 4;
        }

        // PUSH1
        if opcode == 0x60 {
            pc += 1;
            let push1_data = code[pc];

            let mut arr: [u64; 4] = [0, 0, 0, 0];

            arr[0] = push1_data as u64;
            stack.insert(0, U256(arr));
        }

        // PUSH2
        if opcode == 0x61 {
            pc += 1;
            let mut push2_data = (code[pc] as u16) << 8;
            pc += 1;
            let push2_data2 = code[pc];
            push2_data += push2_data2 as u16;

            let mut arr: [u64; 4] = [0, 0, 0, 0];

            arr[0] = push2_data as u64;
            stack.insert(0, U256(arr));
        }

        // PUSH0
        if opcode == 0x5f {
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
