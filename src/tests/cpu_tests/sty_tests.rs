use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::components::cartridge::Rom;
use crate::components::bus::Bus;



#[test]
fn test_0x84_sty_zero_page() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);
    let set_y_to_5 = cpu_test_helper::set_register_y_to_value(0x05);

    cpu.load_and_run(vec![set_y_to_5[0], set_y_to_5[1], 0x84, 0x40, 0x00]);

    assert_eq!(cpu.register_y, 0x05);
    assert_eq!(cpu.mem_read(0x40), 0x05)
}

#[test]
fn test_0x94_sty_zero_page_x() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);
    let set_y_to_5 = cpu_test_helper::set_register_y_to_value(0x05);
    let address_to_test: u8 = 0x10;
    cpu.mem_write(address_to_test as u16, 0x1);
    
    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        set_y_to_5[0],
        set_y_to_5[1],
        0x94,
        address_to_test - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_y, 0x05);
    assert_eq!(cpu.mem_read(address_to_test as u16), 0x05)
}

#[test]
fn test_0x8c_sty_absolute() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);
    let set_y_to_5 = cpu_test_helper::set_register_y_to_value(0x05);
    let address_to_test: u16 = 0x1234;
    let address_to_test_bytes = address_to_test.to_le_bytes();
    cpu.mem_write(address_to_test, 0x10);
    
    cpu.load_and_run(vec![
        set_y_to_5[0],
        set_y_to_5[1],
        0x8C,
        address_to_test_bytes[0],
        address_to_test_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_y, 0x05);
    assert_eq!(cpu.mem_read(address_to_test), 0x05)
}

