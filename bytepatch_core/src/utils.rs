#[macro_export]
macro_rules! try_gread_vec_with {
    ($src:ident, $offset:ident, $cap:expr, $ctx:expr) => {{
        let mut vec = Vec::with_capacity($cap as usize);
        for _ in 0..$cap {
            vec.push($src.gread_with($offset, $ctx)?);
        }
        vec
    }};
    ($src:ident, $offset:ident, $cap:expr; ctx = offset) => {{
        let mut vec = Vec::with_capacity($cap as usize);
        for _ in 0..$cap {
            vec.push($src.gread_with($offset, *$offset)?);
        }
        vec
    }};
}
