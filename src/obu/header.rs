use crate::utils::{bits::{BitsReader, FromBitsReader}, enums::ObuType};


pub struct ObuHeader {
    pub obu_forbidden_bit: bool,
    pub obu_type: ObuType,
    pub obu_extension_flag: bool,
    pub obu_has_size_field: bool,
    pub temporal_id: u8,
    pub spatial_id: u8,
    pub obu_size: usize,
}

impl FromBitsReader for ObuHeader {
    fn from_bits_reader(reader: &mut BitsReader) -> Self {
        let obu_forbidden_bit = reader.read_bit();
        let obu_type = reader.read_u8(4).into();
        let obu_extension_flag = reader.read_bit();
        let obu_has_size_field = reader.read_bit();
        reader.read_skip(1);
        let (temporal_id, spatial_id) = if obu_extension_flag {
            (reader.read_u8(3), reader.read_u8(2))
        } else {
            (Default::default(), Default::default())
        };
        let obu_size = reader.read_leb128();
        Self { obu_forbidden_bit, obu_type, obu_extension_flag, obu_has_size_field, temporal_id, spatial_id, obu_size }
    }
}

