use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0xba_tsx_move_stack_to_x() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let target_value = 0xF;

    let mut program = vec![];

    let end = CPU::STACK_RESET - target_value;

    for i in 1..=end {
        program.push(cpu_test_helper::push_accumulator_to_stack());
    }

    program.push(0xba);
    program.push(0x00);

    cpu.load_and_run(program);

    assert_eq!(cpu.register_x, target_value);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0xba_tsx_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let target_value = 0;

    let mut program = vec![];

    let end = CPU::STACK_RESET - target_value;

    for i in 1..=end {
        program.push(cpu_test_helper::push_accumulator_to_stack());
    }

    program.push(0xba);
    program.push(0x00);

    cpu.load_and_run(program);

    assert_eq!(cpu.register_x, target_value);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xba_tsx_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let target_value = 0xFC;

    let mut program = vec![];

    let end = CPU::STACK_RESET - target_value;

    for i in 1..=end {
        program.push(cpu_test_helper::push_accumulator_to_stack());
    }

    program.push(0xba);
    program.push(0x00);

    cpu.load_and_run(program);

    assert_eq!(cpu.register_x, target_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

