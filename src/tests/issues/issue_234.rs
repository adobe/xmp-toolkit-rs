// Test case for https://github.com/adobe/xmp-toolkit-rs/issues/234:
// Unable to open an XMP XML file generated by darktable

use crate::{tests::fixtures::fixture_path, XmpMeta};

#[test]
fn issue_234() {
    let issue_path = fixture_path("issue_234.xmp");
    let meta = XmpMeta::from_file(issue_path).unwrap();
    println!("{:?}", meta); // prints metadata
}
