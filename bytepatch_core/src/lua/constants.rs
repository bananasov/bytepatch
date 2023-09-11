use scroll::{Endian, Pread};

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
        size_t_size: u8,
        endian: Endian,
    ) -> Result<Constant, Box<dyn std::error::Error>> {
        let const_type: u8 = src.gread_with(offset, endian)?;
        let constant = match const_type {
            0 => Constant::LUA_TNIL,
            1 => {
                let value: u8 = src.gread_with(offset, endian)?;
                Constant::LUA_TBOOLEAN(value != 0)
            },
            3 => {
                let value: u32 = src.gread_with(offset, endian)?;
                Constant::LUA_TNUMBER(value)
            },
            4 => {
                let str = match size_t_size {
                    4 => LuaString::read_u32(src, offset, endian)?,
                    8 => LuaString::read_u64(src, offset, endian)?,
                    _ => unreachable!(),
                };

                Constant::LUA_TSTRING(str)
            },
            _ => unreachable!("Somehow got an invalid constant type")
        };

        Ok(constant)
    }
}
