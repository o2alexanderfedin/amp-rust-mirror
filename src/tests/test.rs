use super::*;
use crate::src::amp::{amp_decode, amp_decode_arg};
use crate::src::amp_h::AmpT;
use crate::{__assert_rtn, amp_encode, printf, strcmp};

#[allow(unused_doc_comments)]
pub(crate) extern "C" fn __main_inner() -> i32 {
    let mut args: [*mut i8; 3] = [
        c"some".as_ptr() as *mut i8,
        c"stuff".as_ptr() as *mut i8,
        c"here".as_ptr() as *mut i8,
    ];
    ///Read u32be.
    let buf: *mut i8 = unsafe { amp_encode(&raw mut args[0 as usize] as *mut *mut i8, 3) };
    let mut msg: AmpT = AmpT {
        version: 0 as i16,
        argc: 0,
        buf: core::ptr::null_mut(),
    };
    amp_decode(&mut msg, buf);
    if !(1 == msg.version as i32) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(
                c"main".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                17,
                c"1 == msg.version".as_ptr() as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if !(3 == msg.argc as i32) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(
                c"main".as_ptr() as *const i8,
                c"test.c".as_ptr() as *mut i8 as *const i8,
                18,
                c"3 == msg.argc".as_ptr() as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    {
        let mut i: i32 = 0;
        '__b0: loop {
            if !(i < msg.argc as i32) {
                break '__b0;
            }
            '__c0: loop {
                let arg: *const i8 = amp_decode_arg(&mut msg) as *const i8;
                '__s1: {
                    match i {
                        0 => {
                            if !(0
                                == unsafe {
                                    strcmp(
                                        c"some".as_ptr() as *mut i8 as *const i8,
                                        arg as *const i8,
                                    )
                                }) as i32 as i64
                                != 0
                            {
                                unsafe {
                                    __assert_rtn(
                                        c"main".as_ptr() as *const i8,
                                        c"test.c".as_ptr() as *mut i8 as *const i8,
                                        25,
                                        c"0 == strcmp(\"some\", arg)".as_ptr() as *mut i8
                                            as *const i8,
                                    )
                                }
                            } else {
                                {
                                    let _ = 0;
                                }
                            };
                        }
                        1 => {
                            if !(0
                                == unsafe {
                                    strcmp(
                                        c"stuff".as_ptr() as *mut i8 as *const i8,
                                        arg as *const i8,
                                    )
                                }) as i32 as i64
                                != 0
                            {
                                unsafe {
                                    __assert_rtn(
                                        c"main".as_ptr() as *const i8,
                                        c"test.c".as_ptr() as *mut i8 as *const i8,
                                        28,
                                        c"0 == strcmp(\"stuff\", arg)".as_ptr() as *mut i8
                                            as *const i8,
                                    )
                                }
                            } else {
                                {
                                    let _ = 0;
                                }
                            };
                        }
                        2 => {
                            if !(0
                                == unsafe {
                                    strcmp(
                                        c"here".as_ptr() as *mut i8 as *const i8,
                                        arg as *const i8,
                                    )
                                }) as i32 as i64
                                != 0
                            {
                                unsafe {
                                    __assert_rtn(
                                        c"main".as_ptr() as *const i8,
                                        c"test.c".as_ptr() as *mut i8 as *const i8,
                                        31,
                                        c"0 == strcmp(\"here\", arg)".as_ptr() as *mut i8
                                            as *const i8,
                                    )
                                }
                            } else {
                                {
                                    let _ = 0;
                                }
                            };
                        }
                        _ => {}
                    }
                }
                break '__c0;
            }
            i += 1;
        }
    }
    unsafe { printf(c"ok\n".as_ptr() as *mut i8 as *const i8) };
    return 0;
}
