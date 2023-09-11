use scroll::Endian;

use super::LuaString;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Constant {
    LUA_TNIL,
    LUA_TBOOLEAN(bool),
    LUA_TNUMBER(u32),
    LUA_TSTRING(LuaString),
}

impl<'a> Constant {
    pub fn decode(
        src: &'a [u8],
        offset: &mut usize,
        endian: Endian,
    ) -> Result<Constant, Box<dyn std::error::Error>> {
        todo!()
    }
}
