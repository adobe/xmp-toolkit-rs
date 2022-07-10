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
#ifndef NOOP_FFI
    #include "XMP.hpp"
#endif

std::once_flag xmp_init_flag;

inline void init_xmp_fn() {
    #ifndef NOOP_FFI
        // TO DO: Check return status from Initialize functions.
        try {
            SXMPMeta::Initialize();
            SXMPFiles::Initialize(kXMPFiles_IgnoreLocalText);
        }
        catch (XMP_Error& e) {
            fprintf(stderr, "Failed to initialize XMP Toolkit: %s\n", e.GetErrMsg());
            exit(1);
        }
    #endif

    // TO DO: Terminate? How to hook into process exit?
    // Or do we care that it's a messy exit?
}

static void init_xmp() {
    std::call_once(xmp_init_flag, init_xmp_fn);
}

static const char* copyStringForResult(const std::string& result) {
    size_t size = result.size();
    void* cstr = malloc(size + 1);
    memcpy(cstr, result.c_str(), size + 1);
    return (const char*) cstr;
}

static const char* copyStringForResult(const char* result) {
    size_t size = strlen(result);
    void* cstr = malloc(size + 1);
    memcpy(cstr, result, size + 1);
    return (const char*) cstr;
}

extern "C" {
    typedef struct CXmpError {
        AdobeXMPCommon::int32 hadError;
        AdobeXMPCommon::int32 id;
        const char* debugMessage;

        CXmpError() {
            reset();
        }

        void reset() {
            hadError = 0;
            id = 0;
            debugMessage = NULL;
        }
    } CXmpError;
}

static void copyErrorForResult(XMP_Error& e, CXmpError* outError) {
    if (outError) {
        outError->hadError = 1;
        outError->id = e.GetID();
        outError->debugMessage = copyStringForResult(e.GetErrMsg());
    }
}

static void signalUnknownError(CXmpError* outError) {
    if (outError) {
        outError->hadError = 1;
        outError->id = kXMPErr_Unknown;
    }
}

static bool xmpFileErrorCallback(void* context,
                                 XMP_StringPtr filePath,
                                 XMP_ErrorSeverity severity,
                                 XMP_Int32 cause,
                                 XMP_StringPtr message) {
    CXmpError* err = (CXmpError*) context;
    if (err) {
        err->hadError = 1;
        err->id = cause;
        err->debugMessage = copyStringForResult(message);
    }

    // False means don't attempt to proceed in face of an error.
    return false;
}


extern "C" {
    typedef struct CXmpFile {
        #ifdef NOOP_FFI
            int x;
        #else
            SXMPFiles f;
            CXmpError err;
        #endif

        CXmpFile() {
            f.SetErrorCallback(xmpFileErrorCallback, &err, 0xffffffff);
        }
    } CXmpFile;

    typedef struct CXmpMeta {
        #ifdef NOOP_FFI
            int x;
        #else
            SXMPMeta m;
        #endif
    } CXmpMeta;

    typedef struct CXmpDateTime {
        #ifdef NOOP_FFI
            int x;
        #else
            XMP_DateTime dt;
        #endif
    } CXmpDateTime;

    // --- CXmpFile ---

    CXmpFile* CXmpFileNew(CXmpError* outError) {
        #ifndef NOOP_FFI
            init_xmp();
            try {
                return new CXmpFile;
            }
            catch (XMP_Error& e) {
                copyErrorForResult(e, outError);
            }
            catch (...) {
                signalUnknownError(outError);
            }
        #endif
        return NULL;
    }

    void CXmpFileDrop(CXmpFile* f) {
        #ifndef NOOP_FFI
            delete f;
        #endif
    }

    void CXmpFileOpen(CXmpFile* f,
                      CXmpError* outError,
                      const char* filePath,
                      AdobeXMPCommon::uint32 openFlags) {
        #ifdef NOOP_FFI
            return 1;
        #else
            // TO DO: Bridge file format parameter.
            // For my purposes at the moment,
            // kXMP_UnknownFile always suffices.
            try {
                f->err.reset();
                if (!f->f.OpenFile(filePath, kXMP_UnknownFile, openFlags)) {
                    *outError = f->err;
                    f->err.reset();
                }
            }
            catch (XMP_Error& e) {
                copyErrorForResult(e, outError);
            }
            catch (...) {
                signalUnknownError(outError);
            }
        #endif
    }

    void CXmpFileClose(CXmpFile* f) {
        #ifndef NOOP_FFI
            // TO DO: Bridge closeFlags parameter.
            // For my purposes at the moment,
            // default value (0) always suffices.
            f->f.CloseFile();
        #endif
    }

    CXmpMeta* CXmpFileGetXmp(CXmpFile* f) {
        #ifdef NOOP_FFI
            return NULL;
        #else
            try {
                CXmpMeta* r = new CXmpMeta;
                try {
                    if (f->f.GetXMP(&(r->m))) {
                        return r;
                    }
                }
                catch (...) {
                    delete r;
                }
            }
            catch (...) {
                // Intentional no-op.
            }

            // No metadata or exception occurred.
            // Signal this by returning NULL.
            return NULL;
        #endif
    }

    void CXmpFilePutXmp(CXmpFile* f,
                        CXmpError* outError,
                        const CXmpMeta* m) {
        #ifndef NOOP_FFI
            try {
                f->f.PutXMP(m->m);
            }
            catch (XMP_Error& e) {
                copyErrorForResult(e, outError);
            }
            catch (...) {
                signalUnknownError(outError);
            }
        #endif
    }

    int CXmpFileCanPutXmp(const CXmpFile* f,
                          const CXmpMeta* m) {
        #ifdef NOOP_FFI
            return 0;
        #else
            try {
                return const_cast<SXMPFiles&>(f->f).CanPutXMP(m->m) ? 1 : 0;
            }
            catch (...) {
                return 0;
            }
        #endif
    }

    // --- CXmpMeta ---

    CXmpMeta* CXmpMetaNew(CXmpError* outError) {
        #ifndef NOOP_FFI
            init_xmp();
            try {
                return new CXmpMeta;
            }
            catch (XMP_Error& e) {
                copyErrorForResult(e, outError);
            }
            catch (...) {
                signalUnknownError(outError);
            }
        #endif
        return NULL;
    }

    void CXmpMetaDrop(CXmpMeta* m) {
        delete m;
    }

    const char* CXmpMetaRegisterNamespace(const char* namespaceURI,
                                          const char* suggestedPrefix) {
        #ifdef NOOP_FFI
            return NULL;
        #else
            init_xmp();

            std::string registeredPrefix;

            SXMPMeta::RegisterNamespace(namespaceURI, suggestedPrefix, &registeredPrefix);

            return copyStringForResult(registeredPrefix);
        #endif
    }

    const char* CXmpMetaGetProperty(CXmpMeta* m,
                                    const char* schemaNS,
                                    const char* propName) {
        #ifdef NOOP_FFI
            return NULL;
        #else
            std::string propValue;

            if (m->m.GetProperty(schemaNS, propName, &propValue, NULL /* options */)) {
                return copyStringForResult(propValue);
            } else {
                return NULL;
            }
        #endif
    }

    void CXmpMetaSetProperty(CXmpMeta* m,
                             CXmpError* outError,
                             const char* schemaNS,
                             const char* propName,
                             const char* propValue) {
        #ifndef NOOP_FFI
            // TO DO: Bridge options parameter.
            // For my purposes at the moment,
            // default value (0) always suffices.
            try {
                m->m.SetProperty(schemaNS, propName, propValue);
            }
            catch (XMP_Error& e) {
                copyErrorForResult(e, outError);
            }
            catch (...) {
                signalUnknownError(outError);
            }
        #endif
    }

    int CXmpMetaDoesPropertyExist(CXmpMeta* m,
                                  const char* schemaNS,
                                  const char* propName) {
        #ifdef NOOP_FFI
            return 0;
        #else
            return (m->m.DoesPropertyExist(schemaNS, propName)) ? 1 : 0;
        #endif
    }

    void CXmpMetaSetPropertyDate(CXmpMeta* m,
                                 const char* schemaNS,
                                 const char* propName,
                                 const CXmpDateTime* propValue) {
        #ifndef NOOP_FFI
            // TO DO: Bridge options parameter.
            // For my purposes at the moment,
            // default value (0) always suffices.
            m->m.SetProperty_Date(schemaNS, propName, propValue->dt);
        #endif
    }

    // --- CXmpDateTime ---

    CXmpDateTime* CXmpDateTimeNew() {
        return new CXmpDateTime;
    }

    void CXmpDateTimeDrop(CXmpDateTime* dt) {
        #ifndef NOOP_FFI
            try {
                delete dt;
            }
            catch (XMP_Error& e) {
                fprintf(stderr, "CXMPDateTimeDrop: ERROR %s\n", e.GetErrMsg());
            }
        #endif
    }

    CXmpDateTime* CXmpDateTimeCurrent() {
        CXmpDateTime* dt = new CXmpDateTime;
        #ifndef NOOP_FFI
            try {
                SXMPUtils::CurrentDateTime(&dt->dt);
            }
            catch (XMP_Error& e) {
                fprintf(stderr, "CXMPDateTimeCurrent: ERROR %s\n", e.GetErrMsg());
            }
        #endif
        return dt;
    }
}
