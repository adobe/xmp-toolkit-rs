use std::os::raw::{c_char, c_int};

pub enum CXmpDateTime {}
pub enum CXmpFile {}
pub enum CXmpMeta {}

extern "C" {
    pub fn CXmpFileNew() -> *mut CXmpFile;
    pub fn CXmpFileDrop(file: *mut CXmpFile);
    pub fn CXmpFileOpen(file: *mut CXmpFile, path: *const c_char, flags: u32) -> c_int;
    pub fn CXmpFileGetXMP(file: *mut CXmpFile) -> *mut CXmpMeta;

    pub fn CXmpMetaNew() -> *mut CXmpMeta;
    pub fn CXmpMetaDrop(meta: *mut CXmpMeta);

    pub fn CXmpMetaRegisterNamespace(
        namespace_uri: *const c_char,
        suggested_prefix: *const c_char,
    ) -> *mut c_char;

    pub fn CXmpMetaSetProperty(
        meta: *mut CXmpMeta,
        schema_ns: *const c_char,
        prop_name: *const c_char,
        prop_value: *const c_char,
    );

    pub fn CXmpDateTimeNew() -> *mut CXmpDateTime;
    pub fn CXmpDateTimeDrop(dt: *mut CXmpDateTime);
}
