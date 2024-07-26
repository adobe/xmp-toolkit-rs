// Copyright 2024 Adobe. All rights reserved.
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

// Test case for https://github.com/adobe/xmp-toolkit-rs/issues/230:
// Updating the XMP data of same image concurrently aborts the entire process.

use std::thread::sleep;

use futures::stream::StreamExt;
use rand::{thread_rng, Rng};
use tempfile::tempdir;
use tokio::task::spawn_blocking;
use tokio_macros::test;

use crate::{
    tests::fixtures::temp_copy_of_fixture, xmp_ns::TIFF, OpenFileOptions, XmpFile, XmpMeta,
    XmpValue,
};

#[test(flavor = "multi_thread")]
async fn original_bug() {
    let tempdir: tempfile::TempDir = tempdir().unwrap();
    let image2 = temp_copy_of_fixture(tempdir.path(), "image2.jpg");

    let mut handles = Vec::new();

    for _ in 0..2 {
        let image2 = image2.clone();

        let handle = spawn_blocking(move || {
            let flip = thread_rng().gen_range(1..=8);

            let (mut xmp_file, mut meta) = open_file(&image2);

            sleep(std::time::Duration::from_secs(3));
            update(&mut meta, flip);

            sleep(std::time::Duration::from_secs(3));
            write_to_file(&mut xmp_file, &meta);
        });

        handles.push(handle);
    }

    futures::stream::iter(handles)
        .buffer_unordered(4)
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
}

#[test(flavor = "multi_thread")]
async fn new_api_try_close() {
    let tempdir: tempfile::TempDir = tempdir().unwrap();
    let image2 = temp_copy_of_fixture(tempdir.path(), "image2.jpg");

    let mut handles = Vec::new();

    for _ in 0..2 {
        let image2 = image2.clone();

        let handle = spawn_blocking(move || {
            let flip = thread_rng().gen_range(1..=8);

            let (mut xmp_file, mut meta) = open_file(&image2);

            sleep(std::time::Duration::from_secs(3));
            update(&mut meta, flip);

            sleep(std::time::Duration::from_secs(3));
            write_to_file_try_close(&mut xmp_file, &meta);
        });

        handles.push(handle);
    }

    futures::stream::iter(handles)
        .buffer_unordered(4)
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
}

fn open_file(path: impl AsRef<std::path::Path>) -> (XmpFile, XmpMeta) {
    let path = path.as_ref();

    let mut xmp_file = XmpFile::new().unwrap();

    xmp_file
        .open_file(
            &path,
            OpenFileOptions::default()
                .only_xmp()
                .for_update()
                .use_smart_handler(),
        )
        .or_else(|_| {
            xmp_file.open_file(
                &path,
                OpenFileOptions::default()
                    .only_xmp()
                    .for_update()
                    .use_packet_scanning(),
            )
        })
        .unwrap();

    let xmp = xmp_file.xmp().unwrap_or(XmpMeta::new().unwrap());

    (xmp_file, xmp)
}

fn update(meta: &mut XmpMeta, flip: u8) {
    meta.set_property(TIFF, "Orientation", &XmpValue::new(flip.to_string()))
        .unwrap();
}

fn write_to_file(xmp_file: &mut XmpFile, meta: &XmpMeta) {
    xmp_file.put_xmp(meta).unwrap();
    xmp_file.close();
}

fn write_to_file_try_close(xmp_file: &mut XmpFile, meta: &XmpMeta) {
    xmp_file.put_xmp(meta).unwrap();
    let _ = xmp_file.try_close();
    // This is a race condition: We can't predict which thread
    // will encounter the error on close, so we ignore it.
    // The primary concern here is that we no longer abort the process when that
    // error is encountered.
}
