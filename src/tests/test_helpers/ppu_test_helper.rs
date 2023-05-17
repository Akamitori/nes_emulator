use std::collections::btree_map::Values;
use sdl2::sys::va_list;
use crate::components::cartridge::Mirroring;
use crate::components::ppu::NesPPU;
use crate::tests::test_helpers::cpu_test_helper;

pub fn set_value_to_PPUDATA(value :u8) -> Vec<u8> {
    let load_data = cpu_test_helper::set_accumulator_to_value(value);
    let PPUDATA : u16= 0x2007;
    let store_accumulator_to_PPUDATA=cpu_test_helper::store_accumulator_to_memory_address(PPUDATA);
    let set_data_in_ppu=vec![
        //LDA value
        load_data[0],
        load_data[1],

        //STA PPUDATA
        store_accumulator_to_PPUDATA[0],
        store_accumulator_to_PPUDATA[1],
        store_accumulator_to_PPUDATA[2],
    ];
    
    return set_data_in_ppu;
}

pub fn set_value_to_PPUADDRESS(value : u16) -> Vec<u8>{
    let value=value.to_be_bytes();
    let load_adrress_1 = cpu_test_helper::set_accumulator_to_value(value[0]);
    let load_address_2 = cpu_test_helper::set_accumulator_to_value(value[1]);

    let PPUADDRESS: u16 = 0x2006;
    let store_accumulator_to_PPUADDRESS=cpu_test_helper::store_accumulator_to_memory_address(PPUADDRESS);

    let set_value_in_PPUADDRESS = vec![
        //LDA value_byte_1
        load_adrress_1[0],
        load_adrress_1[1],

        //STA PPUADDRESS
        store_accumulator_to_PPUADDRESS[0],
        store_accumulator_to_PPUADDRESS[1],
        store_accumulator_to_PPUADDRESS[2],

        //LDA value_byte_2
        load_address_2[0],
        load_address_2[1],

        //STA PPUADDRESS
        store_accumulator_to_PPUADDRESS[0],
        store_accumulator_to_PPUADDRESS[1],
        store_accumulator_to_PPUADDRESS[2],
    ];
    
    return set_value_in_PPUADDRESS;
    
}

pub fn write_data_to_PPUADDRESS(value:u8, address :u16) -> Vec<u8>{
    let mut result=vec![];
    result.extend(set_value_to_PPUADDRESS(address));
    result.extend(set_value_to_PPUDATA(value));
    return result;
}

pub fn write_data_to_PPUCTRL(value : u8) -> Vec<u8>{
    let load_data = cpu_test_helper::set_accumulator_to_value(value);
    let PPUCTRL: u16= 0x2000;
    let store_accumulator_to_PPUCTRL =cpu_test_helper::store_accumulator_to_memory_address(PPUCTRL);
    let set_data_in_ppu=vec![
        //LDA value
        load_data[0],
        load_data[1],

        //STA PPUCTRL
        store_accumulator_to_PPUCTRL[0],
        store_accumulator_to_PPUCTRL[1],
        store_accumulator_to_PPUCTRL[2],
    ];

    return set_data_in_ppu;
}