use crate::components::bus::Bus;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0x9c_shy_absolute_x() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos: u16 = 0x1000;
    let y_value = 0x43;

    let set_y_to_value = cpu_test_helper::set_register_y_to_value(y_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        set_y_to_value[0],
        set_y_to_value[1],
        cpu_test_helper::increase_x_by_one(),
        0x9c,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_y,y_value);
    assert_eq!(cpu.mem_read(mem_pos), y_value & mem_bytes[1].wrapping_add(1));
}