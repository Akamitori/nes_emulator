use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0x85_sta_zero_page() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let set_a_to_5 = cpu_test_helper::set_accumulator_to_value(0x05);

    cpu.load_and_run(vec![set_a_to_5[0], set_a_to_5[1], 0x85, 0x40, 0x00]);

    assert_eq!(cpu.register_a, 0x05);
    assert_eq!(cpu.mem_read(0x40), 0x05)
}

#[test]
fn test_0x95_sta_zero_page_x() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let set_a_to_5 = cpu_test_helper::set_accumulator_to_value(0x05);
    let address_to_test: u8 = 0x10;
    cpu.mem_write(address_to_test as u16, 0x1);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        set_a_to_5[0],
        set_a_to_5[1],
        0x95,
        address_to_test - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0x05);
    assert_eq!(cpu.mem_read(address_to_test as u16), 0x05)
}

#[test]
fn test_0x8d_sta_absolute() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let set_a_to_5 = cpu_test_helper::set_accumulator_to_value(0x05);
    let address_to_test: u16 = 0x1234;
    let address_to_test_bytes = address_to_test.to_le_bytes();

    cpu.mem_write(address_to_test, 0x10);
    cpu.load_and_run(vec![
        set_a_to_5[0],
        set_a_to_5[1],
        0x8d,
        address_to_test_bytes[0],
        address_to_test_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0x05);
    assert_eq!(cpu.mem_read(address_to_test), 0x05)
}

#[test]
fn test_0x9d_sta_from_memory_absolute_x() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let set_a_to_5 = cpu_test_helper::set_accumulator_to_value(0x05);
    let address_to_test: u16 = 0x1001;
    let address_to_test_bytes = (address_to_test - 1).to_le_bytes();
    cpu.mem_write(address_to_test + 1, 0x55);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        set_a_to_5[0],
        set_a_to_5[1],
        0x9d,
        address_to_test_bytes[0],
        address_to_test_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0x5);
    assert_eq!(cpu.mem_read(address_to_test), 0x05);
}

#[test]
fn test_0x99_sta_from_memory_absolute_y() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let set_a_to_5 = cpu_test_helper::set_accumulator_to_value(0x05);
    let address_to_test: u16 = 0x1001;
    let address_to_test_bytes = (address_to_test - 1).to_le_bytes();
    cpu.mem_write(address_to_test + 1, 0x55);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_y_by_one(),
        set_a_to_5[0],
        set_a_to_5[1],
        0x99,
        address_to_test_bytes[0],
        address_to_test_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0x5);
    assert_eq!(cpu.mem_read(address_to_test), 0x05);
}

#[test]
fn test_0x81_sta_from_memory_indirect_x() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let addr_to_write: u8 = 0x40;
    let addr_to_store = 0x33;
    let set_a_to_5 = cpu_test_helper::set_accumulator_to_value(0x05);
    cpu.mem_write(addr_to_write as u16, addr_to_store);
    cpu.mem_write_u16(addr_to_store as u16, 0xF);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        set_a_to_5[0],
        set_a_to_5[1],
        0x81,
        addr_to_write - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0x5);
    assert_eq!(cpu.mem_read(addr_to_store as u16), 0x05);
}

#[test]
fn test_0x91_sta_from_memory_indirect_y() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let addr_to_write: u8 = 0x40;
    let addr_to_store = 0x33;
    let set_a_to_5 = cpu_test_helper::set_accumulator_to_value(0x05);

    cpu.mem_write(addr_to_write as u16, addr_to_store - 1);
    cpu.mem_write_u16(addr_to_store as u16, 0xF);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_y_by_one(),
        set_a_to_5[0],
        set_a_to_5[1],
        0x91,
        addr_to_write,
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0x5);
    assert_eq!(cpu.mem_read(addr_to_store as u16), 0x05);
}
