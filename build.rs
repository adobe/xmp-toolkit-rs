// Copyright 2020 Adobe. All rights reserved.
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

use std::{env, ffi::OsStr, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    println!("> git submodule init\n");
    git_command(["submodule", "init"]);

    println!("> git submodule update\n");
    git_command(["submodule", "update"]);

    // docs.rs builds in an environment that doesn't allow us to modify
    // the underlying source. We don't actually need to fully compile,
    // so we do a specialized build that makes all the FFIs into no-ops.
    let docs_rs = env::var("DOCS_RS");
    if docs_rs == Ok("1".to_string()) {
        eprintln!("INFO: building no-op FFI for docs.rs");
        compile_for_docs();
        return;
    } else {
        eprintln!("INFO: building standard FFI for crate");
    }

    // Special note: Because of the post-processing we're doing here,
    // you must specify the `--no-verify` option when invoking `cargo publish`.
    // This is unfortunately necessary because the original XMP Toolkit requires
    // the modified versions of these files to be present in these locations.

    copy_external_to_third_party("libexpat/expat/lib", "expat/lib");

    let mut zlib_adler_c_path = env::current_dir().unwrap();
    zlib_adler_c_path.push("external/xmp_toolkit/third-party/zlib/adler.c");
    if !zlib_adler_c_path.is_file() {
        zlib_adler_c_path.pop();
        let _ignore = std::fs::remove_dir_all(zlib_adler_c_path);
        copy_external_to_third_party("zlib", "zlib");
    }

    // C vs C++ compilation approach adapted from
    // https://github.com/rust-lang/rust/blob/7510b0ca45d1204f8f0e9dc1bb2dc7d95b279c9a/library/unwind/build.rs.

    let mut expat_config = cc::Build::new();
    let mut xmp_config = cc::Build::new();

    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS not defined");
    match target_os.as_ref() {
        "windows" => {
            expat_config
                .include("external/xmp_toolkit/XMPCore/resource/win")
                .include("external/xmp_toolkit/XMPFiles/resource/win");

            if cfg!(feature = "crt_static") {
                xmp_config.static_crt(true);
            }
            xmp_config
                .define("WIN_ENV", "1")
                .define("XMP_WinBuild", "1")
                .define("WIN64", "")
                .define("_WIN64", "1")
                .define("NOMINMAX", "")
                .define("UNICODE", "")
                .define("_UNICODE", "")
                .define("NDEBUG", "")
                .define("_LARGEFILE64_SOURCE", "0")
                .flag("/EHsc")
                .flag("/GF")
                .flag("/GS")
                .flag("/MP")
                .flag("/wd4100")
                .flag("/wd4189")
                .flag("/wd4245")
                .flag("/wd4310")
                .flag("/wd4458")
                .flag("/wd4505")
                .flag("/wd4530")
                .flag("/wd4701")
                .flag("/wd4702")
                .flag("/wd4996")
                .include("external/xmp_toolkit/XMPCore/resource/win")
                .include("external/xmp_toolkit/XMPFiles/resource/win")
                .file("external/xmp_toolkit/source/Host_IO-Win.cpp")
                .file("external/xmp_toolkit/XMPFiles/source/PluginHandler/OS_Utils_WIN.cpp");
        }

        "macos" => {
            expat_config
                .define("XML_DEV_URANDOM", None)
                .include("external/xmp_toolkit/XMPCore/resource/mac")
                .include("external/xmp_toolkit/XMPFiles/resource/mac");

            xmp_config
                .define("MAC_ENV", "1")
                .define("XMP_MacBuild", "1")
                .define("_LARGEFILE64_SOURCE", None)
                .define("XML_DEV_URANDOM", None)
                .flag("-Wno-deprecated-declarations")
                .flag("-Wno-deprecated-register")
                .flag("-Wno-int-in-bool-context")
                .flag("-Wno-macro-redefined")
                .flag("-Wno-null-conversion")
                .flag("-Wno-unused-but-set-variable")
                .include("external/xmp_toolkit/XMPCore/resource/mac")
                .include("external/xmp_toolkit/XMPFiles/resource/mac")
                .file("external/xmp_toolkit/source/Host_IO-POSIX.cpp")
                .file("external/xmp_toolkit/XMPFiles/source/PluginHandler/OS_Utils_Mac.cpp");

            println!("cargo:rustc-link-lib=framework=Carbon");
            println!("cargo:rustc-link-lib=framework=Security");
        }

        "linux" => {
            expat_config
                .define("XML_DEV_URANDOM", None)
                .include("external/xmp_toolkit/XMPCore/resource/linux")
                .include("external/xmp_toolkit/XMPFiles/resource/linux");

            xmp_config
                .define(
                    "kBigEndianHost",
                    if cfg!(target_endian = "little") {
                        "0"
                    } else {
                        "1"
                    },
                )
                .define("UNIX_ENV", "1")
                .define("XMP_UNIXBuild", "1")
                .define("_LARGEFILE64_SOURCE", None)
                .define("XML_DEV_URANDOM", None)
                .flag("-Wno-class-memaccess")
                .flag("-Wno-extra")
                .flag("-Wno-ignored-qualifiers")
                .flag("-Wno-int-in-bool-context")
                .flag("-Wno-int-to-pointer-cast")
                .flag("-Wno-multichar")
                .flag("-Wno-parentheses")
                .flag("-Wno-unused-but-set-variable")
                .flag("-Wno-type-limits")
                .flag("-fpermissive")
                .include("external/xmp_toolkit/XMPCore/resource/linux")
                .include("external/xmp_toolkit/XMPFiles/resource/linux")
                .file("external/xmp_toolkit/source/Host_IO-POSIX.cpp")
                .file("external/xmp_toolkit/XMPFiles/source/PluginHandler/OS_Utils_Linux.cpp");
        }

        _ => {
            // See https://github.com/amethyst/rlua/blob/master/build.rs
            // for suggestions on how to handle other operating systems.

            panic!("Not prepared to build for this OS ({:?}) yet.", target_os);
        }
    };

    expat_config
        .cpp(false)
        .define("HAVE_EXPAT_CONFIG_H", "1")
        .define("NDEBUG", "")
        .flag_if_supported("-Wno-enum-conversion")
        .flag_if_supported("-Wno-missing-field-initializers")
        .flag_if_supported("-Wno-unused-parameter")
        .file("external/xmp_toolkit/third-party/expat/lib/xmlparse.c")
        .file("external/xmp_toolkit/third-party/expat/lib/xmlrole.c")
        .file("external/xmp_toolkit/third-party/expat/lib/xmltok.c")
        .cargo_metadata(false)
        .compile("libexpat.a");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not defined");
    println!("cargo:rustc-link-search=native={}", &out_dir);

    let mut expat_dir = PathBuf::from(&out_dir);
    expat_dir.push("external/xmp_toolkit/third-party/expat/lib");

    let mut count = 0;
    for entry in std::fs::read_dir(&expat_dir).unwrap() {
        let obj = entry.unwrap().path().canonicalize().unwrap();
        if let Some(ext) = obj.extension() {
            if ext == "o" {
                xmp_config.object(&obj);
                count += 1;
            }
        }
    }
    assert_eq!(
        count, 3,
        "Didn't find expected object files from {:?}",
        &out_dir
    );

    println!(
        "cargo:include={}/external/xmp_toolkit/public/include",
        std::env::var("CARGO_MANIFEST_DIR").expect("Failed to get CARGO_MANIFEST_DIR")
    );

    xmp_config
        .cpp(true)
        .define("TXMP_STRING_TYPE", "std::string")
        .define("XML_STATIC", "1")
        .define("XMP_StaticBuild", "1")
        .define("HAVE_EXPAT_CONFIG_H", "1")
        .flag_if_supported("-std=c++11")
        .flag_if_supported("-Wno-deprecated")
        .flag_if_supported("-Wno-deprecated-declarations")
        .flag_if_supported("-Wno-missing-field-initializers")
        .flag_if_supported("-Wno-reorder")
        .flag_if_supported("-Wno-unused-function")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-variable")
        .flag_if_supported("-Wnon-virtual-dtor")
        .flag_if_supported("-Woverloaded-virtual")
        .include("external/xmp_toolkit")
        .include("external/xmp_toolkit/build")
        .include("external/xmp_toolkit/public/include")
        .include("external/xmp_toolkit/XMPFilesPlugins/api/source")
        .file("external/xmp_toolkit/source/IOUtils.cpp")
        .file("external/xmp_toolkit/source/PerfUtils.cpp")
        .file("external/xmp_toolkit/source/UnicodeConversions.cpp")
        .file("external/xmp_toolkit/source/XIO.cpp")
        .file("external/xmp_toolkit/source/XML_Node.cpp")
        .file("external/xmp_toolkit/source/XMPFiles_IO.cpp")
        .file("external/xmp_toolkit/source/XMP_LibUtils.cpp")
        .file("external/xmp_toolkit/source/XMP_ProgressTracker.cpp")
        .file("external/xmp_toolkit/XMPCore/source/ExpatAdapter.cpp")
        .file("external/xmp_toolkit/XMPCore/source/ParseRDF.cpp")
        .file("external/xmp_toolkit/XMPCore/source/WXMPMeta.cpp")
        .file("external/xmp_toolkit/XMPCore/source/WXMPUtils.cpp")
        .file("external/xmp_toolkit/XMPCore/source/XMPCore_Impl.cpp")
        .file("external/xmp_toolkit/XMPCore/source/XMPIterator.cpp")
        .file("external/xmp_toolkit/XMPCore/source/WXMPIterator.cpp")
        .file("external/xmp_toolkit/XMPCore/source/XMPMeta.cpp")
        .file("external/xmp_toolkit/XMPCore/source/XMPMeta-GetSet.cpp")
        .file("external/xmp_toolkit/XMPCore/source/XMPMeta-Parse.cpp")
        .file("external/xmp_toolkit/XMPCore/source/XMPMeta-Serialize.cpp")
        .file("external/xmp_toolkit/XMPCore/source/XMPUtils.cpp")
        .file("external/xmp_toolkit/XMPCore/source/XMPUtils-FileInfo.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/AIFF_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/ASF_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/Basic_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/FLV_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/GIF_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/InDesign_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/JPEG_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/MP3_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/MPEG2_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/MPEG4_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/P2_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/PNG_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/PostScript_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/PSD_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/RIFF_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/Scanner_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/SonyHDV_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/SVG_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/SWF_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/TIFF_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/Trivial_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/UCF_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/WAVE_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/XDCAM_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/XDCAMEX_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/XDCAMFAM_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FileHandlers/XDCAMSAM_Handler.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/AIFF/AIFFBehavior.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/AIFF/AIFFMetadata.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/AIFF/AIFFReconcile.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/ASF_Support.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/ID3_Support.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/IFF/Chunk.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/IFF/ChunkController.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/IFF/ChunkPath.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/IFF/IChunkBehavior.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/IPTC_Support.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/ISOBaseMedia_Support.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/MOOV_Support.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/QuickTime_Support.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/PackageFormat_Support.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/P2_Support.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/PNG_Support.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/PostScript_Support.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/PSIR_FileWriter.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/PSIR_MemoryReader.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/ReconcileIPTC.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/ReconcileLegacy.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/ReconcileTIFF.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/Reconcile_Impl.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/RIFF.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/RIFF_Support.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/SVG_Adapter.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/SWF_Support.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/TIFF_FileWriter.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/TIFF_MemoryReader.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/TIFF_Support.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/TimeConversionUtils.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/XDCAM_Support.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/XMPScanner.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/WAVE/BEXTMetadata.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/WAVE/CartMetadata.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/WAVE/DISPMetadata.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/WAVE/INFOMetadata.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/WAVE/iXMLMetadata.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/WAVE/WAVEBehavior.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/FormatSupport/WAVE/WAVEReconcile.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/HandlerRegistry.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/NativeMetadataSupport/IMetadata.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/NativeMetadataSupport/MetadataSet.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/NativeMetadataSupport/IReconcile.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/PluginHandler/FileHandlerInstance.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/PluginHandler/HostAPIImpl.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/PluginHandler/Module.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/PluginHandler/PluginManager.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/PluginHandler/XMPAtoms.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/WXMPFiles.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/XMPFiles.cpp")
        .file("external/xmp_toolkit/XMPFiles/source/XMPFiles_Impl.cpp")
        .file("external/xmp_toolkit/third-party/zlib/adler32.c")
        .file("external/xmp_toolkit/third-party/zlib/compress.c")
        .file("external/xmp_toolkit/third-party/zlib/crc32.c")
        .file("external/xmp_toolkit/third-party/zlib/deflate.c")
        .file("external/xmp_toolkit/third-party/zlib/gzclose.c")
        .file("external/xmp_toolkit/third-party/zlib/gzlib.c")
        .file("external/xmp_toolkit/third-party/zlib/gzread.c")
        .file("external/xmp_toolkit/third-party/zlib/gzwrite.c")
        .file("external/xmp_toolkit/third-party/zlib/infback.c")
        .file("external/xmp_toolkit/third-party/zlib/inffast.c")
        .file("external/xmp_toolkit/third-party/zlib/inflate.c")
        .file("external/xmp_toolkit/third-party/zlib/inftrees.c")
        .file("external/xmp_toolkit/third-party/zlib/trees.c")
        .file("external/xmp_toolkit/third-party/zlib/uncompr.c")
        .file("external/xmp_toolkit/third-party/zlib/zutil.c")
        .file("src/ffi.cpp")
        .file("external/xmp_toolkit/third-party/zuid/interfaces/MD5.cpp")
        .compile("libxmp.a");
}

fn copy_external_to_third_party(from_path: &str, to_path: &str) {
    use fs_extra::dir::{copy, CopyOptions};

    let mut dest_path = env::current_dir().unwrap();
    dest_path.push("external/xmp_toolkit/third-party");
    dest_path.push(to_path);

    if !dest_path.is_dir() {
        let mut src_path = env::current_dir().unwrap();
        src_path.push("external");
        src_path.push(from_path);

        assert!(src_path.is_dir());

        dest_path.pop();

        let copy_options = CopyOptions::new();
        println!("COPYING {} to {}", src_path.display(), dest_path.display());
        copy(src_path, dest_path, &copy_options).unwrap();
    }
}

fn git_command<I, S>(args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = std::process::Command::new("git")
        .args(args)
        .output()
        .unwrap();

    println!(
        "--- stdout ---\n{}\n\n--- stderr ---\n{}\n\n",
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap()
    );

    // When we run inside the docs.rs environment (and, presumably,
    // any client that is building xmp-toolkit-rs as a dependency),
    // the submodule doesn't exist, so we should ignore any
    // error from git.
    // assert_eq!(output.status.code().unwrap(), 0);
}

fn compile_for_docs() {
    let mut config = cc::Build::new();

    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS not defined");

    match target_os.as_ref() {
        "macos" => {
            config
                .define("MAC_ENV", "1")
                .define("XMP_MacBuild", "1")
                .flag("-Wno-deprecated-declarations")
                .flag("-Wno-deprecated-register")
                .flag("-Wno-macro-redefined")
                .flag("-Wno-null-conversion")
                .flag("-Wno-unused-but-set-variable")
                .include("external/xmp_toolkit/XMPCore/resource/mac")
                .include("external/xmp_toolkit/XMPFiles/resource/mac")
                .file("external/xmp_toolkit/source/Host_IO-POSIX.cpp")
                .file("external/xmp_toolkit/XMPFiles/source/PluginHandler/OS_Utils_Mac.cpp");

            println!("cargo:rustc-link-lib=framework=Carbon");
            println!("cargo:rustc-link-lib=framework=Security");
        }

        "linux" => {
            config
                .define("UNIX_ENV", "1")
                .define("XMP_UNIXBuild", "1")
                .flag("-Wno-class-memaccess")
                .flag("-Wno-extra")
                .flag("-Wno-ignored-qualifiers")
                .flag("-Wno-int-in-bool-context")
                .flag("-Wno-int-to-pointer-cast")
                .flag("-Wno-multichar")
                .flag("-Wno-parentheses")
                .flag("-Wno-unused-but-set-variable")
                .flag("-Wno-type-limits")
                .include("external/xmp_toolkit/XMPCore/resource/linux")
                .include("external/xmp_toolkit/XMPFiles/resource/linux")
                .file("external/xmp_toolkit/source/Host_IO-POSIX.cpp")
                .file("external/xmp_toolkit/XMPFiles/source/PluginHandler/OS_Utils_Linux.cpp");
        }

        _ => {
            panic!("Not prepared to do docs build for this OS yet.");
        }
    };

    config
        .cpp(true)
        .define("NOOP_FFI", Some("1"))
        .flag_if_supported("-std=c++11")
        .flag_if_supported("-Wno-deprecated")
        .flag_if_supported("-Wno-deprecated-declarations")
        .flag_if_supported("-Wno-missing-field-initializers")
        .flag_if_supported("-Wno-reorder")
        .flag_if_supported("-Wno-unused-function")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-variable")
        .flag_if_supported("-Wnon-virtual-dtor")
        .flag_if_supported("-Woverloaded-virtual")
        .include("external/xmp_toolkit")
        .include("external/xmp_toolkit/build")
        .include("external/xmp_toolkit/public/include")
        .include("external/xmp_toolkit/XMPFilesPlugins/api/source")
        .file("src/ffi.cpp")
        .compile("libxmp.a");
}
