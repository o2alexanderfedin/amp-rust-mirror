use super::*;
use crate::{__builtin___memcpy_chk, __builtin_object_size, malloc};
use crate::src::amp_h::AmpT;

///Write u32be.
extern "C" fn write_u32_be(buf: *mut i8, n: u32) -> () {
    unsafe { *buf.offset(0 as isize) = (n >> 24 & 255 as u32) as i8 };
    unsafe { *buf.offset(1 as isize) = (n >> 16 & 255 as u32) as i8 };
    unsafe { *buf.offset(2 as isize) = (n >> 8 & 255 as u32) as i8 };
    unsafe { *buf.offset(3 as isize) = (n & 255 as u32) as i8 };
}

///Decode the `msg` header in `buf`.
pub(crate) extern "C" fn amp_decode(msg: &mut AmpT, buf: *mut i8) -> () {
    (*msg).version = (unsafe { *buf.offset(0 as isize) } as i32 >> 4) as i16;
    (*msg).argc = (unsafe { *buf.offset(0 as isize) } as i32 & 15) as i16;
    (*msg).buf = unsafe { buf.offset(1 as isize) };
}

///Read u32be.
extern "C" fn read_u32_be(buf: *const i8) -> u32 {
    let mut n: u32 = 0 as u32;
    n |= ((unsafe { *buf.offset(0 as isize) } as i32) << 24) as u32;
    n |= ((unsafe { *buf.offset(1 as isize) } as i32) << 16) as u32;
    n |= ((unsafe { *buf.offset(2 as isize) } as i32) << 8) as u32;
    n |= unsafe { *buf.offset(3 as isize) } as u32;
    return n;
}

///Decode `msg` argument, returning a buffer
///that must be freed by the user and progressing
///the msg->buf cursor.
pub(crate) extern "C" fn amp_decode_arg(msg: &mut AmpT) -> *mut i8 {
    let len: u32 = read_u32_be((*msg).buf as *const i8);
    {
        let __n = 4;
        let __p = &mut (*msg).buf;
        *__p = unsafe { (*__p).offset(__n as isize) };
    };
    let buf: *mut i8 = unsafe { malloc(len as u64) } as *mut i8;
    if (buf).is_null() as i32 != 0 { return 0 as *mut () as *mut i8; }
    unsafe {
        __builtin___memcpy_chk(buf as *mut (), (*msg).buf as *const (),
            len as u64, unsafe { __builtin_object_size(buf as *const (), 0) })
    };
    {
        let __n = len;
        let __p = &mut (*msg).buf;
        *__p = unsafe { (*__p).add(__n as usize) };
    };
    return buf;
}
