#![deny(warnings)]

mod ffi;

mod xmp_const;
pub use xmp_const::*;

mod xmp_date_time;
pub use xmp_date_time::XmpDateTime;

mod xmp_file;
pub use xmp_file::OpenFileOptions;
pub use xmp_file::XmpFile;
pub use xmp_file::XmpFileError;

mod xmp_meta;
pub use xmp_meta::XmpMeta;
