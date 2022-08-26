use naia_shared::serde::{BitReader, BitWrite, Serde, SerdeErr};
use strum::FromRepr;

#[derive(Clone, Debug, Copy, PartialEq, FromRepr)]
#[repr(u8)]
pub enum DestroymentMethod {
    Quiet,
    Explosion,
    Combo,
}

impl Serde for DestroymentMethod {
    fn ser(&self, writer: &mut dyn BitWrite) {
        writer.write_byte(*self as u8);
    }

    fn de(reader: &mut BitReader) -> Result<Self, SerdeErr> {
        DestroymentMethod::from_repr(reader.read_byte()).ok_or(SerdeErr {})
    }
}
