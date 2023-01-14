use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::{AddressingMode, CPU};
use crate::components::mem::Mem;
use crate::components::opcodes::{OpCode};

pub fn run() {
    let game_code = std::fs::read("nestest.nes").unwrap();
    let rom = Rom::new(&game_code).unwrap();
    let bus = Bus::new(rom);
    let mut cpu = CPU::new(bus);
    cpu.reset();
    cpu.program_counter = 0xC000; // set this for the code to initialize properly

    cpu.run_with_callback(move |cpu| {
        println!("{}",mytrace(cpu));
    });
}

fn mytrace(cpu: &mut CPU) ->String{
    let pc = cpu.program_counter;
    let pc_hex = format!("{:04X}", pc);

    let code = cpu.mem_read(cpu.program_counter);
    let op_code_data = cpu.op_codes.get(code);
    let mut operands = get_machine_code(cpu, pc, op_code_data.bytes as u16);

    let command_hex = get_command_hex(&operands);
    let command_assembly = get_command_assembly(cpu, op_code_data, &mut operands);
    let register_values = get_register_status(cpu);

    return format!("{}  {}  {}{}{} \t", pc_hex, command_hex, command_assembly, generate_padding(18), register_values);
}

fn get_register_status(p0: &mut CPU) -> String {
    format!("A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}", p0.register_a, p0.register_x, p0.register_y, p0.status, p0.stack_pointer)
}

fn get_command_hex(machine_code: &Vec<u8>) -> String {
    let mut command_hex = String::from("");
    let expected_length = 8 as i32;
    for (i, operand) in machine_code.iter().enumerate() {
        let formatted_operand = format!("{:02X}", operand);
        command_hex.push_str(&formatted_operand);
        if i != machine_code.len() - 1 {
            command_hex.push_str(" ");
        }
    }

    let padding = expected_length - command_hex.len() as i32;

    if padding > 0 {
        let pad = generate_padding(padding as u32);
        command_hex.push_str(pad.as_str());
    }
    command_hex
}

fn get_command_assembly(cpu: &mut CPU, op_code: OpCode, machine_code: &mut Vec<u8>) -> String {
    let mut command_string = format!("{}", op_code.command_name);

    let mut machine_code_to_work_on = vec![];

    for (pos, byte) in machine_code.iter().enumerate() {
        if pos > 0 {
            machine_code_to_work_on.push(*byte);
        }
    }

    let begin = cpu.program_counter;
    let (mem_addr, stored_value) = match op_code.addressing_mode {
        AddressingMode::Immediate | AddressingMode::NoneAddressing => (0, 0),
        _ => {
            let addr = cpu.get_absolute_address(&op_code.addressing_mode, begin + 1);
            (addr, cpu.mem_read(addr))
        }
    };


    let formatted_operands = match (op_code.addressing_mode, op_code.bytes) {
        (AddressingMode::Immediate, _) => {
            format!(" #${:02X}", machine_code_to_work_on[0])
        }
        (AddressingMode::ZeroPage, _) => {
            format!(" ${:02X} = {:02X}", machine_code_to_work_on[0], cpu.mem_read(machine_code_to_work_on[0] as u16))
        }
        (AddressingMode::ZeroPage_X, _) => {
            format!(" ${:02X},X @ {:02} = #${:02X}", machine_code_to_work_on[0], mem_addr, stored_value)
        }
        (AddressingMode::ZeroPage_Y, _) => {
            format!(" ${:02X},Y @ {:02} = #${:02X}", machine_code_to_work_on[0], mem_addr, stored_value)
        }
        (AddressingMode::Absolute, _) => {
            match op_code.command_name {
                "JSR" | "JMP" => format!(" ${:04X}", mem_addr),
                _ => format!(" ${:04X} = {:02X}", mem_addr, stored_value)
            }
        }
        (AddressingMode::Absolute_X, _) => {
            format!(" ${:04X},X", get_from_le_bytes(&machine_code_to_work_on))
        }
        (AddressingMode::Absolute_Y, _) => {
            format!(" ${:04X},Y", get_from_le_bytes(&machine_code_to_work_on))
        }
        (AddressingMode::Indirect_X, _) => {
            format!(" (${:02X},X)", machine_code_to_work_on[0])
        }

        (AddressingMode::Indirect_Y, _) => {
            format!(" (${:02X}),Y", machine_code_to_work_on[0])
        }
        (AddressingMode::NoneAddressing, 2..=3) => {
            let address_to_operate = match op_code.command_name {
                "JMP" => {
                    get_from_le_bytes(&machine_code_to_work_on)
                }
                _ => {
                    cpu.program_counter + 1 + 1 + machine_code_to_work_on[0] as u16
                }
            };
            format!(" ${:04X}", address_to_operate)
        }
        (AddressingMode::NoneAddressing, 1) => {
            match op_code.op_code {
                0x0A | 0x4A | 0x2A | 0x6A => {
                    " A".to_string()
                }
                _ => "".to_string()
            }
        }
        (_, _) => String::from(""),
    };

    command_string.push_str(&formatted_operands);

    let expected_length = 14 as i32;
    let padding = expected_length - command_string.len() as i32;

    if padding > 0 {
        let pad = generate_padding(padding as u32);
        command_string.push_str(pad.as_str());
    }

    // command_string.push_str(&formatted_operands);
    command_string
}

fn get_machine_code(cpu: &mut CPU, program_counter: u16, operand_count: u16) -> Vec<u8> {
    let mut operands = vec![];

    for i in 0..operand_count {
        operands.push(cpu.mem_read(program_counter + i));
    }

    return operands;
}

fn get_from_le_bytes(input: &Vec<u8>) -> u16 {
    let conversion: [u8; 2] = input[0..2].try_into().expect("Wrong input of array");
    return u16::from_le_bytes(conversion);
}

fn generate_padding(number_of_spaces: u32) -> String {
    (1..=number_of_spaces).map(|_| " ").collect::<String>()
}