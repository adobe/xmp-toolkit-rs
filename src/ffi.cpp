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

#include <cstring>
#include <mutex>
#include <string>

#define TXMP_STRING_TYPE std::string
#define XMP_INCLUDE_XMPFILES 1

#include "XMP.incl_cpp"
#include "XMP.hpp"

std::once_flag xmp_init_flag;

inline void init_xmp_fn() {
    // TO DO: Check return status from Initialize functions.
    try {
        SXMPMeta::Initialize();
        SXMPFiles::Initialize(kXMPFiles_IgnoreLocalText);
    }
    catch (XMP_Error& e) {
        fprintf(stderr, "Failed to initialize XMP Toolkit: %s\n", e.GetErrMsg());
        exit(1);
    }

    // TO DO: Terminate? How to hook into process exit?
    // Or do we care that it's a messy exit?
}

static void init_xmp() {
    std::call_once(xmp_init_flag, init_xmp_fn);
}

extern "C" {
    typedef struct CXmpFile {
        SXMPFiles f;
    } CXmpFile;

    CXmpFile* CXmpFileNew() {
        init_xmp();
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

    typedef struct CXmpDateTime {
        XMP_DateTime dt;
    } CXmpDateTime;

    CXmpDateTime* CXmpDateTimeNew() {
        return new CXmpDateTime;
    }

    void CXmpDateTimeDrop(CXmpDateTime* dt) {
        try {
            delete dt;
        }
        catch (XMP_Error& e) {
            fprintf(stderr, "CXMPDateTimeDrop: ERROR %s\n", e.GetErrMsg());
        }
    }

    CXmpDateTime* CXmpDateTimeCurrent() {
        CXmpDateTime* dt = new CXmpDateTime;
        try {
            SXMPUtils::CurrentDateTime(&dt->dt);
        }
        catch (XMP_Error& e) {
            fprintf(stderr, "CXMPDateTimeCurrent: ERROR %s\n", e.GetErrMsg());
        }
        return dt;
    }

    typedef struct CXmpMeta {
        SXMPMeta m;
    } CXmpMeta;

    CXmpMeta* CXmpMetaNew() {
        return new CXmpMeta;
    }

    CXmpMeta* CXmpFileGetXmp(CXmpFile* f) {
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

    static const char* copyForResult(const std::string& result) {
        size_t size = result.size();
        void* cstr = malloc(size + 1);
        memcpy(cstr, result.c_str(), size + 1);
        return (const char*) cstr;
    }

    const char* CXmpMetaRegisterNamespace(const char* namespaceURI,
                                          const char* suggestedPrefix) {
        init_xmp();

        std::string registeredPrefix;

        SXMPMeta::RegisterNamespace(namespaceURI, suggestedPrefix, &registeredPrefix);

        return copyForResult(registeredPrefix);
    }

    const char* CXmpMetaGetProperty(CXmpMeta* m,
                                    const char* schemaNS,
                                    const char* propName) {
        std::string propValue;

        if (m->m.GetProperty(schemaNS, propName, &propValue, NULL /* options */)) {
            return copyForResult(propValue);
        } else {
            return NULL;
        }
    }

    void CXmpMetaSetProperty(CXmpMeta* m,
                             const char* schemaNS,
                             const char* propName,
                             const char* propValue) {
        // TO DO: Bridge options parameter.
        // For my purposes at the moment,
        // default value (0) always suffices.
        m->m.SetProperty(schemaNS, propName, propValue);
    }

    void CXmpMetaSetPropertyDate(CXmpMeta* m,
                                 const char* schemaNS,
                                 const char* propName,
                                 const CXmpDateTime* propValue) {
        // TO DO: Bridge options parameter.
        // For my purposes at the moment,
        // default value (0) always suffices.
        m->m.SetProperty_Date(schemaNS, propName, propValue->dt);
    }

    int CXmpMetaDoesPropertyExist(CXmpMeta* m,
                                  const char* schemaNS,
                                  const char* propName) {
        return (m->m.DoesPropertyExist(schemaNS, propName)) ? 1 : 0;
    }

    int CXmpFileCanPutXmp(const CXmpFile* f,
                          const CXmpMeta* m) {
        return const_cast<SXMPFiles&>(f->f).CanPutXMP(m->m) ? 1 : 0;
    }

    void CXmpFilePutXmp(CXmpFile* f,
                        const CXmpMeta* m) {
        f->f.PutXMP(m->m);
    }

    void CXmpFileClose(CXmpFile* f) {
        // TO DO: Bridge closeFlags parameter.
        // For my purposes at the moment,
        // default value (0) always suffices.
        f->f.CloseFile();
    }
}