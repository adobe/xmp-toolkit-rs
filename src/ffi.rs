use std::os::raw::c_int;

pub enum CXmpFile {}

extern "C" {
    pub fn CXmpInitialize() -> c_int;
    pub fn CXmpTerminate() -> c_int;

    pub fn CXmpFileNew() -> *mut CXmpFile;
    pub fn CXmpFileDrop(file: *mut CXmpFile);
}
