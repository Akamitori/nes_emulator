use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::components::cartridge::Rom;
use crate::components::bus::Bus;



#[test]
fn test_0xf8_sed() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);

    cpu.load_and_run(vec![0xf8, 0x00]);

    cpu_test_helper::assert_active_decimal_flag(&cpu);
}
