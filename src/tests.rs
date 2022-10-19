mod adc_tests;

mod and_tests;

mod asl_tests;

mod bit_tests;

mod clc_tests;

mod cld_tests;

mod cli_tests;

mod clv_tests;

mod cmp_tests;

mod cpx_tests;

mod cpy_tests;

mod dec_tests;

mod dex_tests;

mod dey_tests;

mod eor_tests;

mod inc_tests;

mod inx_tests;

mod iny_tests;

mod lda_tests;

mod ldx_tests;

mod ldy_tests;

mod lsr_tests;

mod ora_tests;

mod pha_tests;

mod php_tests;

mod pla_tests;

mod plp_tests;

mod rol_tests;

mod ror_tests;

mod sbc_tests;

mod sec_tests;

mod sed_tests;

mod sei_tests;

mod sta_tests;

mod stx_tests;

mod sty_tests;

mod tax_tests;

mod tay_tests;

mod test_helper;

mod tsx_tests;

mod  txa_tests;

mod txs_tests;

mod  tya_tests;

mod test {
    use crate::*;
    
    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

}


