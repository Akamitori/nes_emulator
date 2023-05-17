use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0xd8_cld() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);

    let set_decimal_flag = 0xf8;

    cpu.load_and_run(vec![set_decimal_flag, 0xd8, 0x00]);

    cpu_test_helper::assert_inactive_decimal_flag(&cpu);
}
