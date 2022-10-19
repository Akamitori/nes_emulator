use crate::{tests::test_helper, CPU};
use crate::AddressingMode::Indirect_X;

#[test]
fn test_0x9a_tsx_move_stack_to_x() {
    let mut cpu = CPU::new();
    let target_value = 0xF;
    let mut program = vec![];
    let end = CPU::STACK_RESET - target_value;

    for i in 1..=end {
        program.push(test_helper::push_accumulator_to_stack());
    }

    let  set_x_to_value=test_helper::set_register_x_to_value(CPU::STACK_RESET);
    program.push(set_x_to_value[0]);
    program.push(set_x_to_value[1]);
    program.push(0x9a);
    
    program.push(0x00);

    cpu.load_and_run(program);

    assert_eq!(cpu.stack_pointer, CPU::STACK_RESET);
}