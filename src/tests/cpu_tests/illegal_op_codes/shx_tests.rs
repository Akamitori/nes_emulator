use crate::components::bus::Bus;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0x9e_shx_absolute_y() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos: u16 = 0x1000;
    let x_value = 0x43;

    let set_x_to_value = cpu_test_helper::set_register_x_to_value(x_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        set_x_to_value[0],
        set_x_to_value[1],
        cpu_test_helper::increase_y_by_one(),
        0x9e,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_x, x_value);
    assert_eq!(cpu.mem_read(mem_pos), x_value & mem_bytes[1].wrapping_add(1));
}