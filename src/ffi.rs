use std::os::raw::c_int;

extern "C" {
    pub fn CXmpInitialize() -> c_int;
    pub fn CXmpTerminate() -> c_int;
}
