use crate::components::bus::Bus;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0xbb_las_absolute_y() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_value = 0x55;
    let mem_pos: u16 = 0x1000;
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    let cpu_stack_pointer = CPU::STACK_RESET - 1;
    cpu.mem_write(mem_pos, mem_value);

    let push_accumulator_to_stack = 0x48;
    cpu.load_and_run(vec![
        push_accumulator_to_stack,
        cpu_test_helper::increase_y_by_one(),
        0xbb,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value & cpu_stack_pointer);
    assert_eq!(cpu.register_x, mem_value & cpu_stack_pointer);
    assert_eq!(cpu.stack_pointer, mem_value & cpu_stack_pointer);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xbb_las_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_value = 0x00;
    let mem_pos: u16 = 0x1000;
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    let cpu_stack_pointer = CPU::STACK_RESET - 1;
    cpu.mem_write(mem_pos, mem_value);

    let push_accumulator_to_stack = 0x48;
    cpu.load_and_run(vec![
        push_accumulator_to_stack,
        cpu_test_helper::increase_y_by_one(),
        0xbb,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value & cpu_stack_pointer);
    assert_eq!(cpu.register_x, mem_value & cpu_stack_pointer);
    assert_eq!(cpu.stack_pointer, mem_value & cpu_stack_pointer);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xbb_las_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_value = 0xF1;
    let mem_pos: u16 = 0x1000;
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    let cpu_stack_pointer = CPU::STACK_RESET - 1;
    cpu.mem_write(mem_pos, mem_value);

    let push_accumulator_to_stack = 0x48;
    cpu.load_and_run(vec![
        push_accumulator_to_stack,
        cpu_test_helper::increase_y_by_one(),
        0xbb,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value & cpu_stack_pointer);
    assert_eq!(cpu.register_x, mem_value & cpu_stack_pointer);
    assert_eq!(cpu.stack_pointer, mem_value & cpu_stack_pointer);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}