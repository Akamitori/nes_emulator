use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0x9f_ahx_absolute_y() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos: u16 = 0x1000;
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    let accum_value = 80;
    let x_value = 0x81;
    let mem_value = 112;
    cpu.mem_write(mem_pos, mem_value);

    let prep_1 = cpu_test_helper::set_accumulator_to_value(accum_value);
    let prep_2 = cpu_test_helper::set_register_x_to_value(x_value);

    cpu.load_and_run(vec![
        prep_1[0],
        prep_1[1],
        prep_2[0],
        prep_2[1],
        cpu_test_helper::increase_y_by_one(),
        0x9f,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_pos), accum_value & mem_bytes[1].wrapping_add(1) & x_value);
}

#[test]
fn test_0x93_ahx_indirect_y() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0010;
    let accum_value = 80;
    let x_value = 0x81;
    let mem_value = 112;
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect + 1, mem_value);

    let prep_1 = cpu_test_helper::set_accumulator_to_value(accum_value);
    let prep_2 = cpu_test_helper::set_register_x_to_value(x_value);

    cpu.load_and_run(vec![
        prep_1[0],
        prep_1[1],
        prep_2[0],
        prep_2[1],
        cpu_test_helper::increase_y_by_one(),
        0x93,
        mem_to_load,
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_pos_indirect + 1), accum_value & mem_to_load.wrapping_add(1) & x_value);
}