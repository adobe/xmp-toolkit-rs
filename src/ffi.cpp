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
}
