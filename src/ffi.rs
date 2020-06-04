use std::os::raw::{c_char, c_int};

pub enum CXmpFile {}
pub enum CXmpMeta {}

extern "C" {
    pub fn CXmpInitialize() -> c_int;
    pub fn CXmpTerminate() -> c_int;

    pub fn CXmpFileNew() -> *mut CXmpFile;
    pub fn CXmpFileDrop(file: *mut CXmpFile);
    pub fn CXmpFileOpen(file: *mut CXmpFile, path: *const c_char, flags: u32) -> c_int;
    pub fn CXmpFileGetXMP(file: *mut CXmpFile) -> *mut CXmpMeta;

    pub fn CXmpMetaNew() -> *mut CXmpMeta;
    pub fn CXmpMetaDrop(file: *mut CXmpMeta);

    pub fn CXmpMetaRegisterNamespace(
        namespace_uri: *const c_char,
        suggested_prefix: *const c_char,
    ) -> *mut c_char;
}
