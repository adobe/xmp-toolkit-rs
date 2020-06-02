use crate::ffi;

pub struct XmpToolkit {}

impl Drop for XmpToolkit {
    fn drop(&mut self) {
        unsafe {
            if ffi::CXmpTerminate() != 1 {
                panic!("Failed to close XMP Toolkit");
            }
        }
    }
}

impl XmpToolkit {
    /// Initializes the XMP Toolkit modules SXMPMeta and SXMPFiles.
    ///
    /// Terminates those libraries when out of scope.
    ///
    /// Only one XmpToolkit struct should be active at any one time.
    pub fn new() -> XmpToolkit {
        unsafe {
            if ffi::CXmpInitialize() != 1 {
                panic!("Failed to initialize XMP Toolkit");
            }
        }
        XmpToolkit {}
    }
}

#[cfg(test)]
mod tests {
    use super::XmpToolkit;

    #[test]
    fn init_and_terminate() {
        let _xmp = XmpToolkit::new();
    }
}
