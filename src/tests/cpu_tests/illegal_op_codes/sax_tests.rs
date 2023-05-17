use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0x8f_sax_zero_absolute() {
    let mut cpu = CPU::new(Bus::new(test_rom(0x0600, None)));
    let address_to_test: u16 = 0x1234;
    let address_to_test_bytes = address_to_test.to_le_bytes();
    
    let x_value = 0x03;
    let set_x = cpu_test_helper::set_register_x_to_value(x_value);
    let accum_value = 0x01;
    let set_a = cpu_test_helper::set_accumulator_to_value(accum_value);
    let command = 0x8f;
    
    cpu.status;
    cpu.load_and_run(vec![
        set_a[0],
        set_a[1],
        set_x[0],
        set_x[1],
        command,
        address_to_test_bytes[0],
        address_to_test_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.mem_read(address_to_test), accum_value & x_value);
    cpu_test_helper::assert_flags_unaffected(&cpu);
}

#[test]
fn test_0x87_sax_zero_page() {
    let mut cpu = CPU::new(Bus::new(test_rom(0x0600, None)));
    let address_to_test: u8 = 0x40;

    let x_value = 0x03;
    let set_x = cpu_test_helper::set_register_x_to_value(x_value);
    let accum_value = 0x01;
    let set_a = cpu_test_helper::set_accumulator_to_value(accum_value);
    let command = 0x87;


    cpu.load_and_run(vec![
        set_a[0],
        set_a[1],
        set_x[0],
        set_x[1],
        command,
        address_to_test,
        0x00,
    ]);

    assert_eq!(cpu.mem_read(address_to_test as u16), accum_value & x_value);
    cpu_test_helper::assert_flags_unaffected(&cpu);
}

#[test]
fn test_0x97_sax_zero_page_y() {
    let mut cpu = CPU::new(Bus::new(test_rom(0x0600, None)));
    let address_to_test: u8 = 0xF0;

    let x_value = 0x03;
    let set_x = cpu_test_helper::set_register_x_to_value(x_value);
    let accum_value = 0x01;
    let set_a = cpu_test_helper::set_accumulator_to_value(accum_value);
    let y_value=5;
    let set_y=cpu_test_helper::set_register_y_to_value(y_value);
    let command = 0x97;


    cpu.load_and_run(vec![
        set_y[0],
        set_y[1],
        set_a[0],
        set_a[1],
        set_x[0],
        set_x[1],
        command,
        address_to_test-y_value,
        0x00,
    ]);

    assert_eq!(cpu.mem_read(address_to_test as u16), accum_value & x_value);
    cpu_test_helper::assert_flags_unaffected(&cpu);
}

#[test]
fn test_0x83_sax_zero_indirect_x() {
    let mut cpu = CPU::new(Bus::new(test_rom(0x0600, None)));
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x00F1;

    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);

    let x_value = 0x03;
    let set_x = cpu_test_helper::set_register_x_to_value(x_value);
    let accum_value = 0x01;
    let set_a = cpu_test_helper::set_accumulator_to_value(accum_value);
    
    let command = 0x83;


    cpu.load_and_run(vec![
        set_x[0],
        set_x[1],
        set_a[0],
        set_a[1],
        command,
        mem_to_load-x_value,
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_pos_indirect as u16), accum_value & x_value);
    cpu_test_helper::assert_flags_unaffected(&cpu);
}
