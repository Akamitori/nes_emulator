use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0xe0_cpx_immediate_access() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);

    cpu.load_and_run(vec![0xe0, 0x01, 0x00]);

    cpu_test_helper::assert_inactive_zero_carry_flags(&cpu);
}

#[test]
fn test_0xe0_cpx_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);

    cpu.load_and_run(vec![cpu_test_helper::increase_x_by_one(), 0xe0, 0x01, 0x00]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xe0_cpx_carry_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);

    cpu.load_and_run(vec![cpu_test_helper::increase_x_by_one(), 0xe0, 0x01, 0x00]);

    cpu_test_helper::assert_active_carry_flag(&cpu);
}

#[test]
fn test_0xe0_cpx_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);

    cpu.load_and_run(vec![cpu_test_helper::increase_x_by_one(), 0xe0, 0x05, 0x00]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xe4_cpx_zero_page() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0x1);

    cpu.load_and_run(vec![0xe4, 0x10, 0x00]);

    cpu_test_helper::assert_inactive_zero_carry_flags(&cpu);
}

#[test]
fn test_0xe4_cpx_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0x1);

    cpu.load_and_run(vec![cpu_test_helper::increase_x_by_one(), 0xe4, 0x10, 0x00]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xe4_cpx_carry_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0x1);

    cpu.load_and_run(vec![cpu_test_helper::increase_x_by_one(), 0xe4, 0x10, 0x00]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xe4_cpx_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0x5);

    cpu.load_and_run(vec![cpu_test_helper::increase_x_by_one(), 0xe4, 0x10, 0x00]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xec_cpx_absolute() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem: u16 = 0x1000;
    let mem_bytes = mem.to_le_bytes();
    cpu.mem_write(mem, 0x1);

    let prep = cpu_test_helper::set_accumulator_to_value(4);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        0xec,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_inactive_zero_carry_flags(&cpu);
}

#[test]
fn test_0xec_cpx_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem: u16 = 0x1000;
    let mem_bytes = mem.to_le_bytes();
    cpu.mem_write(mem, 0x1);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xec,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xec_cpx_carry_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem: u16 = 0x1000;
    let mem_bytes = mem.to_le_bytes();
    cpu.mem_write(mem, 0x1);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xec,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_active_carry_flag(&cpu);
}

#[test]
fn test_0xec_cpx_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem: u16 = 0x1000;
    let mem_bytes = mem.to_le_bytes();
    cpu.mem_write(mem, 0x5);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xec,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}
