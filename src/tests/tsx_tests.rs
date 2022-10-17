use crate::{tests::test_helper, CPU};
use crate::AddressingMode::Indirect_X;

#[test]
fn test_0xba_tsx_move_stack_to_x() {
    let mut cpu = CPU::new();
    let  target_value=0xF;
    
    let  mut program=vec![];
    
    let  end= CPU::STACK_RESET-target_value;

    for i in 1..=end   {
        program.push(test_helper::push_accumulator_to_stack());
    }
    
    program.push(0xba);
    program.push(0x00);

    cpu.load_and_run(program);

    assert_eq!(cpu.register_x, target_value);
    test_helper::assert_inactive_negative_flag(&cpu);
    test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0xba_tsx_zero_flag() {
    let mut cpu = CPU::new();
    let  target_value=0;

    let  mut program=vec![];

    let  end= CPU::STACK_RESET-target_value;

    for i in 1..=end   {
        program.push(test_helper::push_accumulator_to_stack());
    }

    program.push(0xba);
    program.push(0x00);

    cpu.load_and_run(program);

    assert_eq!(cpu.register_x, target_value);
    test_helper::assert_inactive_negative_flag(&cpu);
    test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xba_tsx_negative_flag() {
    let mut cpu = CPU::new();
    let  target_value=0xFC;

    let  mut program=vec![];

    let  end= CPU::STACK_RESET-target_value;

    for i in 1..=end   {
        program.push(test_helper::push_accumulator_to_stack());
    }

    program.push(0xba);
    program.push(0x00);

    cpu.load_and_run(program);

    assert_eq!(cpu.register_x, target_value);
    test_helper::assert_active_negative_flag(&cpu);
    test_helper::assert_inactive_zero_flag(&cpu);
}

