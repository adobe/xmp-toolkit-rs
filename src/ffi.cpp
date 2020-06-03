#include <string>

#define TXMP_STRING_TYPE std::string 
#define XMP_INCLUDE_XMPFILES 1 

#include "XMP.incl_cpp"
#include "XMP.hpp"

extern "C" {
    int CXmpInitialize() {
        try {
            if (!SXMPMeta::Initialize()) return 0;
            #ifdef UNIX_ENV
                // ERROR (InitXMP): Generic UNIX clients must pass kXMPFiles_IgnoreLocalText
                if (!SXMPFiles::Initialize(kXMPFiles_IgnoreLocalText))   
                    return false;
            #else
                if (!SXMPFiles::Initialize()) return 0;
            #endif
        }
        catch(XMP_Error& e) {
            return 0;
        }
        return 1;
    }

    int CXmpTerminate() {
        try {
            SXMPFiles::Terminate();
            SXMPMeta::Terminate();
            return 1;
        }
        catch(XMP_Error& e) {
            return 0;
        }
    }

    typedef struct CXmpFile {
        SXMPFiles f;
    } CXmpFile;

    CXmpFile* CXmpFileNew() {
        return new CXmpFile;
    }

    void CXmpFileDrop(CXmpFile* f) {
        delete f;
    }

    int CXmpFileOpen(CXmpFile* f,
                     const char* filePath,
                     AdobeXMPCommon::uint32 openFlags) {
        // TO DO: Bridge file format parameter.
        // For my purposes at the moment,
        // kXMP_UnknownFile always suffices.
        return f->f.OpenFile(filePath, kXMP_UnknownFile, openFlags) ? 1 : 0;
    }

    typedef struct CXmpMeta {
        SXMPMeta m;
    } CXmpMeta;

    CXmpMeta* CXmpMetaNew() {
        return new CXmpMeta;
    }

    CXmpMeta* CXmpFileGetXMP(CXmpFile* f) {
        CXmpMeta* r = new CXmpMeta;
        if (f->f.GetXMP(&(r->m))) {
            return r;
        } else {
            // No metadata. Signal this by returning NULL.
            delete r;
            return NULL;
        }
    }

    void CXmpMetaDrop(CXmpMeta* m) {
        delete m;
    }
}
