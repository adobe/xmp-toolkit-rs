// Copyright 2022 Adobe. All rights reserved.
// This file is licensed to you under the Apache License,
// Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
// or the MIT license (http://opensource.org/licenses/MIT),
// at your option.

// Unless required by applicable law or agreed to in writing,
// this software is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR REPRESENTATIONS OF ANY KIND, either express or
// implied. See the LICENSE-MIT and LICENSE-APACHE files for the
// specific language governing permissions and limitations under
// each license.

// This application will accept a file path to a resource, open
// the file as read-only, then read the XMP data from the file.
// Once the XMP packet is available, it will access several
// properties and print those values to stdout.

// The application reads properties from three different schemas:
// the XMP Basic schema, the Dublin Core schema, and the EXIF
// schema.

// Based on the example titled "Creating the MyReadXMP application"
// from XMP TOolkit SDK Programmer's Guide (page 68 of the February
// 2022 edition).

use anyhow::{anyhow, Context, Result};
use xmp_toolkit::{OpenFileOptions, XmpFile};

use std::env;

fn read_xmp_from_file() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    eprintln!("args = {:#?}", args);

    let path = match args.len() {
        // args[0] = path to executable
        2 => Ok(&args[1]),
        n => Err(anyhow!(
            "expected 1 argument (file name), got {} arguments",
            n - 1
        )),
    }?;

    let mut f = XmpFile::new();

    f.open_file(&path, OpenFileOptions::OPEN_ONLY_XMP | OpenFileOptions::OPEN_USE_SMART_HANDLER)
        .or_else(|_err| {
            eprintln!(
                "No smart handler available for file {}. Trying packet scanning.",
                path
            );
            f.open_file(path, OpenFileOptions::OPEN_USE_PACKET_SCANNING)
        })
        .with_context(|| format!("could not find XMP in file {}", path))?;

    let _xmp = f.xmp().ok_or(anyhow!("unable to process XMP in file {}", path))?;

    // TODO: Continue from step 10 of C++ example.

    Ok(())
}

fn main() {
    if let Err(err) = read_xmp_from_file().context("could not read XMP from file") {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}
