use crate::components::bus::Bus;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0x9b_tas_absolute_y() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_value = 0x55;
    let mem_pos: u16 = 0x1000;
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    let cpu_stack_pointer = CPU::STACK_RESET - 1;
    let accum_value = 0x42;
    let x_value = 0x47;
    cpu.mem_write(mem_pos, mem_value);


    let set_accumulator_to_value = cpu_test_helper::set_register_a_to_value(accum_value);
    let set_x_to_value = cpu_test_helper::set_register_x_to_value(x_value);
    cpu.load_and_run(vec![
        set_accumulator_to_value[0],
        set_accumulator_to_value[1],
        set_x_to_value[0],
        set_x_to_value[1],
        cpu_test_helper::increase_y_by_one(),
        0x9b,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.stack_pointer, accum_value & x_value);
    assert_eq!(cpu.register_a, accum_value);
    assert_eq!(cpu.register_x, x_value);

    assert_eq!(cpu.mem_read(mem_pos), cpu.stack_pointer & mem_bytes[1].wrapping_add(1));
}