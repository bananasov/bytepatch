use scroll::{ctx, Endian, Pread};

use crate::try_gread_vec_with;

#[derive(Debug)]
pub struct Header {
    pub magic: u32,           // four bytes
    pub version: u8,          // one byte, Version number, 0x51 (81 decimal) for Lua 5.1
    pub format_version: u8,   // one byte
    pub endianess_flag: u8,   // one byte, default is 1,  0=big endian, 1=little endian
    pub int_size: u8,         // one byte, default value is 4, Size of int (in bytes)
    pub size_t_size: u8,      // one byte default value is 4, Size of size_t (in bytes)
    pub instruction_size: u8, // one byte, default value is 4, Size of Instruction (in bytes)
    pub lua_number_size: u8,  // one byte, default value is 8, Size of lua_Number (in bytes)
    pub integral_flag: u8,    // one byte default value 0, 0=floating-point, 1=integral number type
}

#[derive(Debug)]
pub struct LuaString {
    pub data: Vec<u8>,
}

impl<'a> LuaString {
    pub fn read(
        src: &'a [u8],
        offset: &mut usize,
        endian: Endian,
        integer: u8,
    ) -> Result<(LuaString, usize), Box<dyn std::error::Error>> {
        let data: Vec<u8> = try_gread_vec_with!(src, offset, integer, endian);

        Ok((LuaString { data }, *offset))
    }
}

impl<'a> ctx::TryFromCtx<'a, Endian> for Header {
    type Error = scroll::Error;
    fn try_from_ctx(src: &'a [u8], endian: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let magic: u32 = src.gread_with(offset, endian)?;
        let version: u8 = src.gread_with(offset, endian)?;
        let format_version: u8 = src.gread_with(offset, endian)?;
        let endianess_flag: u8 = src.gread_with(offset, endian)?;
        let int_size: u8 = src.gread_with(offset, endian)?;
        let size_t_size: u8 = src.gread_with(offset, endian)?;
        let instruction_size: u8 = src.gread_with(offset, endian)?;
        let lua_number_size: u8 = src.gread_with(offset, endian)?;
        let integral_flag: u8 = src.gread_with(offset, endian)?;

        Ok((
            Header {
                magic,
                version,
                format_version,
                endianess_flag,
                int_size,
                size_t_size,
                instruction_size,
                lua_number_size,
                integral_flag,
            },
            *offset,
        ))
    }
}
