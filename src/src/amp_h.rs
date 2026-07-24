use super::*;

///Message struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct AmpT {
    pub(crate) version: i16,
    pub(crate) argc: i16,
    pub(crate) buf: *mut i8,
}
