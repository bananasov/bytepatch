
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum ConstantType {
    LUA_TNIL,
    LUA_TBOOLEAN(bool),
    LUA_TNUMBER(u32),
    LUA_TSTRING(String),
}

#[derive(Debug)]
pub struct Constant(ConstantType);