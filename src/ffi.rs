use std::os::raw::{c_char, c_int};

pub enum CXmpDateTime {}
pub enum CXmpFile {}
pub enum CXmpMeta {}

extern "C" {
    // --- CXmpFile

    pub fn CXmpFileNew() -> *mut CXmpFile;
    pub fn CXmpFileDrop(file: *mut CXmpFile);
    pub fn CXmpFileOpen(file: *mut CXmpFile, path: *const c_char, flags: u32) -> c_int;
    pub fn CXmpFileGetXmp(file: *mut CXmpFile) -> *mut CXmpMeta;
    pub fn CXmpFileCanPutXmp(file: *const CXmpFile, meta: *const CXmpMeta) -> c_int;
    pub fn CXmpFilePutXmp(file: *mut CXmpFile, meta: *const CXmpMeta);
    pub fn CXmpFileClose(file: *mut CXmpFile);

    // --- CXmpMeta

    pub fn CXmpMetaNew() -> *mut CXmpMeta;
    pub fn CXmpMetaDrop(meta: *mut CXmpMeta);

    pub fn CXmpMetaRegisterNamespace(
        namespace_uri: *const c_char,
        suggested_prefix: *const c_char,
    ) -> *mut c_char;

    pub fn CXmpMetaGetProperty(
        meta: *mut CXmpMeta,
        schema_ns: *const c_char,
        prop_name: *const c_char,
    ) -> *mut c_char;

    pub fn CXmpMetaSetProperty(
        meta: *mut CXmpMeta,
        schema_ns: *const c_char,
        prop_name: *const c_char,
        prop_value: *const c_char,
    );

    pub fn CXmpMetaSetPropertyDate(
        meta: *mut CXmpMeta,
        schema_ns: *const c_char,
        prop_name: *const c_char,
        prop_value: *const CXmpDateTime,
    );

    pub fn CXmpMetaDoesPropertyExist(
        meta: *const CXmpMeta,
        schema_ns: *const c_char,
        prop_name: *const c_char,
    ) -> c_int;

    // --- CXmpDateTime

    pub fn CXmpDateTimeNew() -> *mut CXmpDateTime;
    pub fn CXmpDateTimeDrop(dt: *mut CXmpDateTime);
    pub fn CXmpDateTimeCurrent() -> *mut CXmpDateTime;
}
