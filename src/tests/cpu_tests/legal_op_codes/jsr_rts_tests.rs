use std::io::Lines;

use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

/// Simple test where we increase X till 5 with subroutines
#[test]
fn test_0x20_jsr_0x60_rts_subroutines() {
    let pc_counter_start = 0x0600;
    let bus = Bus::new(test_rom(pc_counter_start,None));
    let mut cpu = CPU::new(bus);
    let target_value = 5;

    let program = build_program(target_value, pc_counter_start);

    cpu.load_and_run(program);

    assert_eq!(cpu.register_x, target_value);
}

fn build_program(target_value: u8, pc_start: u16) -> Vec<u8> {
    let program_parts = build_program_parts(target_value, pc_start);
    let mut program = Vec::new();

    for part in program_parts {
        for line in part {
            program.push(line);
        }
    }
    return program;
}

fn build_program_parts(target_value: u8, initial_program_counter: u16) -> Vec<Vec<u8>> {
    let init_offset = (initial_program_counter + 9 as u16).to_le_bytes();
    let loop_offset = (initial_program_counter + 12 as u16).to_le_bytes();
    let end_offset = (initial_program_counter + 18 as u16).to_le_bytes();

    let jsr: u8 = 0x20;
    let rts: u8 = 0x60;

    let jmp_init_subroutine = vec![jsr, init_offset[0], init_offset[1]];

    let jmp_loop_subroutine = vec![jsr, loop_offset[0], loop_offset[1]];

    let jmp_end_subroutine = vec![jsr, end_offset[0], end_offset[1]];

    let set_x = cpu_test_helper::set_register_x_to_value(0);
    let compare_x_to_5 = cpu_test_helper::compare_x_to_value(target_value);
    let branch_for_loop = cpu_test_helper::branch_not_equal((5 as u8).wrapping_neg());

    let init_subroutine = vec![set_x[0], set_x[1], rts];

    let loop_subroutine = vec![
        cpu_test_helper::increase_x_by_one(),
        compare_x_to_5[0],
        compare_x_to_5[1],
        branch_for_loop[0],
        branch_for_loop[1],
        rts,
    ];

    let end_subroutine = vec![0 as u8];

    let program_parts = vec![
        jmp_init_subroutine,
        jmp_loop_subroutine,
        jmp_end_subroutine,
        init_subroutine,
        loop_subroutine,
        end_subroutine,
    ];

    program_parts
}


