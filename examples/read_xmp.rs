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

// ------------------------------------------------------------

// This application will accept a file path to a resource, open
// the file as read-only, then read the XMP data from the file.
// Once the XMP packet is available, it will access several
// properties and print those values to stdout.

// The application reads properties from three different schemas:
// the XMP Basic schema, the Dublin Core schema, and the Exif
// schema.

// Based on the example titled "Creating the MyReadXMP application"
// from XMP Toolkit SDK Programmer's Guide (pages 68-71 of the
// February 2022 edition).

use std::env;

use anyhow::{anyhow, Context, Result};
use xmp_toolkit::{xmp_ns, OpenFileOptions, XmpFile, XmpMeta};

fn read_xmp_from_file() -> Result<()> {
    // Parse command-line arguments. There should be only one
    // argument: a path to a file to be read.
    let args: Vec<String> = env::args().collect();

    let path = match args.len() {
        // args[0] = path to executable
        2 => Ok(&args[1]),
        n => Err(anyhow!(
            "expected 1 argument (file name), got {} arguments",
            n - 1
        )),
    }?;

    // Open the file for read-only access and request to use a format-specific
    // handler.
    let mut f = XmpFile::new()?;

    f.open_file(
        path,
        OpenFileOptions::default().only_xmp().use_smart_handler(),
    )
    .or_else(|_err| {
        // There might not be an appropriate handler available.
        // Retry using packet scanning, providing a different set of
        // open-file options.
        eprintln!(
            "No smart handler available for file {}. Trying packet scanning.",
            path
        );
        f.open_file(path, OpenFileOptions::default().use_packet_scanning())
    })
    .with_context(|| format!("could not find XMP in file {}", path))?;

    // Retrieve the XMP from the file.
    let xmp = f
        .xmp()
        .ok_or_else(|| anyhow!("unable to process XMP in file {}", path))?;

    // Add the code to display the simple property "CreatorTool" by providing
    // the namespace URI and the name of the property.
    if let Some(creator_tool) = xmp.property(xmp_ns::XMP, "CreatorTool") {
        println!("CreatorTool = {}", creator_tool.value);
    }

    // Display the first element of the `creator` array.
    if let Some(first_creator) = xmp.property_array(xmp_ns::DC, "creator").next() {
        println!("dc:creator = {}", first_creator.value);
    } else {
        println!("No creator found");
    }

    // Display all elements in the `subject` property (which is an array).
    // Note that the C++ XMP Toolkit's indices are 1-based. This example's output
    // instead follows Rust's convention of being 0-based.
    for (index, v) in xmp.property_array(xmp_ns::DC, "subject").enumerate() {
        println!("dc::subject[{}] = {}", index, v.value);
    }

    // Get a localized text item; display the `title` property in English.
    if let Some((value, _actual_lang)) =
        xmp.localized_text(xmp_ns::DC, "title", Some("en"), "en-US")
    {
        println!("dc:title in English = {}", value.value);
    }

    // Get a localized text item; display the `title` property in French.
    if let Some((value, _actual_lang)) =
        xmp.localized_text(xmp_ns::DC, "title", Some("fr"), "fr-FR")
    {
        println!("dc:title in French = {}", value.value);
    }

    // Get a date property; read the `MetadataDate` property if it exists. If so,
    // convert the `XmpDateTime` into a string and display it.
    if let Some(value) = xmp.property_date(xmp_ns::XMP, "MetadataDate") {
        println!("meta:MetadataDate = {}", value.value);
    }

    // Discover if the Exif Flash structure is available. If so, display the
    // flash status at the time the photograph was taken.
    if xmp.contains_struct_field(xmp_ns::EXIF, "Flash", xmp_ns::EXIF, "Fired") {
        let path = XmpMeta::compose_struct_field_path(xmp_ns::EXIF, "Flash", xmp_ns::EXIF, "Fired")
            .unwrap();

        if let Some(value) = xmp.property_bool(xmp_ns::EXIF, &path) {
            println!("Flash Used = {}", value.value);
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = read_xmp_from_file().context("could not read XMP from file") {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}
