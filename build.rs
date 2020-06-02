use std::env;

fn main() {
    copy_external_to_third_party("expat/lib");

    let mut zlib_adler_c_path = env::current_dir().unwrap();
    zlib_adler_c_path.push("external/xmp_toolkit/third-party/zlib/adler.c");
    if !zlib_adler_c_path.is_file() {
        zlib_adler_c_path.pop();
        std::fs::remove_dir_all(zlib_adler_c_path).unwrap();
        copy_external_to_third_party("zlib");
    }

    let mut config = cc::Build::new();

    let target_os = env::var("CARGO_CFG_TARGET_OS");

    if target_os == Ok("macos".to_string()) {
        config
            .define("MAC_ENV", "1")
            .define("XMP_MacBuild", "1")
            .flag("-Wno-deprecated-declarations")
            .include("external/xmp_toolkit/XMPCore/resource/mac")
            .include("external/xmp_toolkit/XMPFiles/resource/mac")
            .file("external/xmp_toolkit/source/Host_IO-POSIX.cpp")
            .file("external/xmp_toolkit/XMPFiles/source/PluginHandler/OS_Utils_Mac.cpp");

        println!("cargo:rustc-link-lib=framework=Carbon");
        println!("cargo:rustc-link-lib=framework=Security");
    } else {
        // See https://github.com/amethyst/rlua/blob/master/build.rs
        // for suggestions on how to handle other operating systems.

        panic!("Not prepared to build for this OS yet.");
    }

    config
        .cpp(true)
        .define("HAVE_EXPAT_CONFIG_H", "1")
        .define("TXMP_STRING_TYPE", "std::string")
        .define("XML_STATIC", "1")
        .define("XMP_StaticBuild", "1")
        .define("_LARGEFILE64_SOURCE", None)
        .flag_if_supported("-std=c++11")
        .flag_if_supported("-Wno-deprecated")
        .flag_if_supported("-Wno-deprecated-register")
        .flag_if_supported("-Wno-missing-field-initializers")
        .flag_if_supported("-Wno-null-conversion")
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
        .file("external/xmp_toolkit/third-party/expat/lib/xmlparse.c")
        .file("external/xmp_toolkit/third-party/expat/lib/xmlrole.c")
        .file("external/xmp_toolkit/third-party/expat/lib/xmltok.c")
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
        .file("external/xmp_toolkit/third-party/zuid/interfaces/MD5.cpp")
        .compile("libxmp.a");
}

fn copy_external_to_third_party(name: &str) {
    use fs_extra::dir::{copy, CopyOptions};

    let mut dest_path = env::current_dir().unwrap();
    dest_path.push("external/xmp_toolkit/third-party");
    dest_path.push(name);

    if !dest_path.is_dir() {
        let mut src_path = env::current_dir().unwrap();
        src_path.push("external");
        src_path.push(name);

        dest_path.pop();

        let copy_options = CopyOptions::new();
        println!("COPYING {} to {}", src_path.display(), dest_path.display());
        copy(src_path, dest_path, &copy_options).unwrap();
    }
}
