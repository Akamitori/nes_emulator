

#[cfg(test)]
mod and_tests;

#[cfg(test)]
mod asl_tests;

#[cfg(test)]
mod bit_tests;

#[cfg(test)]
mod cmp_tests;

#[cfg(test)]
mod cpx_tests;

#[cfg(test)]
mod cpy_tests;

#[cfg(test)]
mod dec_tests;

#[cfg(test)]
mod dex_tests;

#[cfg(test)]
mod dey_tests;

#[cfg(test)]
mod eor_tests;

#[cfg(test)]
mod inc_tests;

#[cfg(test)]
mod inx_tests;

#[cfg(test)]
mod iny_tests;

#[cfg(test)]
mod lda_tests;

#[cfg(test)]
mod ldx_tests;

#[cfg(test)]
mod ldy_tests;

#[cfg(test)]
mod sta_tests;

#[cfg(test)]
mod ta_tests;

mod test_helper;

#[cfg(test)]
mod test {
    use crate::*;
    
    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

}


