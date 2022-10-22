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
            #ifndef NOOP_FFI
                f.SetErrorCallback(xmpFileErrorCallback, &err, 0xffffffff);
            #endif
        }
    } CXmpFile;

    typedef struct CXmpMeta {
        #ifdef NOOP_FFI
            int x;
        #else
            SXMPMeta m;
        #endif
    } CXmpMeta;

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
        #ifndef NOOP_FFI
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
        #ifndef NOOP_FFI
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
        #endif

        // No metadata or exception occurred.
        // Signal this by returning NULL.
        return NULL;
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
        #ifndef NOOP_FFI
            try {
                return const_cast<SXMPFiles&>(f->f).CanPutXMP(m->m) ? 1 : 0;
            }
            catch (...) {
                // Intentional no-op.
            }
        #endif

        return 0;
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
        #ifndef NOOP_FFI
            delete m;
        #endif
    }

    CXmpMeta* CXmpMetaParseFromBuffer(CXmpError* outError,
                                      const char* buffer,
                                      AdobeXMPCommon::uint32 buffer_size) {
        #ifndef NOOP_FFI
            init_xmp();
            CXmpMeta* result = new CXmpMeta;

            try {
                result->m.ParseFromBuffer(buffer, buffer_size);
                return result;
            }
            catch (XMP_Error& e) {
                delete result;
                copyErrorForResult(e, outError);
            }
            catch (...) {
                delete result;
                signalUnknownError(outError);
            }
        #endif

        return NULL;
    }

    const char* CXmpMetaRegisterNamespace(CXmpError* outError,
                                          const char* namespaceURI,
                                          const char* suggestedPrefix) {
        #ifndef NOOP_FFI
            init_xmp();

            try {
                std::string registeredPrefix;
                SXMPMeta::RegisterNamespace(namespaceURI, suggestedPrefix, &registeredPrefix);

                return copyStringForResult(registeredPrefix);
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

    const char* CXmpMetaGetProperty(CXmpMeta* m,
                                    CXmpError* outError,
                                    const char* schemaNS,
                                    const char* propName,
                                    AdobeXMPCommon::uint32* outOptions) {
        *outOptions = 0;

        #ifndef NOOP_FFI
            try {
                std::string propValue;
                if (m->m.GetProperty(schemaNS, propName, &propValue, outOptions)) {
                    return copyStringForResult(propValue);
                }
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

    bool CXmpMetaGetProperty_Bool(CXmpMeta* m,
                                  CXmpError* outError,
                                  const char* schemaNS,
                                  const char* propName,
                                  bool* outValue,
                                  AdobeXMPCommon::uint32* outOptions) {
        *outOptions = 0;

        #ifndef NOOP_FFI
            try {
                std::string propValue;
                if (m->m.GetProperty_Bool(schemaNS, propName, outValue, outOptions)) {
                    return true;
                }
            }
            catch (XMP_Error& e) {
                copyErrorForResult(e, outError);
            }
            catch (...) {
                signalUnknownError(outError);
            }
        #endif

        return false;
    }
    
	bool CXmpMetaGetProperty_Int(CXmpMeta* m,
                                 CXmpError* outError,
                                 const char* schemaNS,
                                 const char* propName,
                                 AdobeXMPCommon::int32* outValue,
                                 AdobeXMPCommon::uint32* outOptions) {
        *outOptions = 0;

        #ifndef NOOP_FFI
            try {
                std::string propValue;
                if (m->m.GetProperty_Int(schemaNS, propName, outValue, outOptions)) {
                    return true;
                }
            }
            catch (XMP_Error& e) {
                copyErrorForResult(e, outError);
            }
            catch (...) {
                signalUnknownError(outError);
            }
        #endif

        return false;
    }

	bool CXmpMetaGetProperty_Int64(CXmpMeta* m,
                                   CXmpError* outError,
                                   const char* schemaNS,
                                   const char* propName,
                                   AdobeXMPCommon::int64* outValue,
                                   AdobeXMPCommon::uint32* outOptions) {
        *outOptions = 0;

        #ifndef NOOP_FFI
            try {
                std::string propValue;
                if (m->m.GetProperty_Int64(schemaNS, propName, outValue, outOptions)) {
                    return true;
                }
            }
            catch (XMP_Error& e) {
                copyErrorForResult(e, outError);
            }
            catch (...) {
                signalUnknownError(outError);
            }
        #endif

        return false;
    }

	bool CXmpMetaGetProperty_Float(CXmpMeta* m,
                                   CXmpError* outError,
                                   const char* schemaNS,
                                   const char* propName,
                                   double* outValue,
                                   AdobeXMPCommon::uint32* outOptions) {
        *outOptions = 0;

        #ifndef NOOP_FFI
            try {
                std::string propValue;
                if (m->m.GetProperty_Float(schemaNS, propName, outValue, outOptions)) {
                    return true;
                }
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
    
    bool CXmpMetaGetProperty_Date(CXmpMeta* m,
                                  CXmpError* outError,
                                  const char* schemaNS,
                                  const char* propName,
                                  XMP_DateTime* outValue,
                                  AdobeXMPCommon::uint32* outOptions) {
        *outOptions = 0;

        #ifndef NOOP_FFI
            try {
                std::string propValue;
                if (m->m.GetProperty_Date(schemaNS, propName, outValue, outOptions)) {
                    return true;
                }
            }
            catch (XMP_Error& e) {
                copyErrorForResult(e, outError);
            }
            catch (...) {
                signalUnknownError(outError);
            }
        #endif

        return false;
    }

    void CXmpMetaSetProperty(CXmpMeta* m,
                             CXmpError* outError,
                             const char* schemaNS,
                             const char* propName,
                             const char* propValue,
                             AdobeXMPCommon::uint32 options) {
        #ifndef NOOP_FFI
            try {
                m->m.SetProperty(schemaNS, propName, propValue, options);
            }
            catch (XMP_Error& e) {
                copyErrorForResult(e, outError);
            }
            catch (...) {
                signalUnknownError(outError);
            }
        #endif
    }

    void CXmpMetaSetProperty_Bool(CXmpMeta* m,
                                  CXmpError* outError,
                                  const char* schemaNS,
                                  const char* propName,
                                  bool propValue,
                                  AdobeXMPCommon::uint32 options) {
        #ifndef NOOP_FFI
            try {
                m->m.SetProperty_Bool(schemaNS, propName, propValue, options);
            }
            catch (XMP_Error& e) {
                copyErrorForResult(e, outError);
            }
            catch (...) {
                signalUnknownError(outError);
            }
        #endif
    }

    void CXmpMetaSetProperty_Int(CXmpMeta* m,
                                 CXmpError* outError,
                                 const char* schemaNS,
                                 const char* propName,
                                 AdobeXMPCommon::int32 propValue,
                                 AdobeXMPCommon::uint32 options) {
        #ifndef NOOP_FFI
            try {
                m->m.SetProperty_Int(schemaNS, propName, propValue, options);
            }
            catch (XMP_Error& e) {
                copyErrorForResult(e, outError);
            }
            catch (...) {
                signalUnknownError(outError);
            }
        #endif
    }

    void CXmpMetaSetProperty_Int64(CXmpMeta* m,
                                   CXmpError* outError,
                                   const char* schemaNS,
                                   const char* propName,
                                   AdobeXMPCommon::int64 propValue,
                                   AdobeXMPCommon::uint32 options) {
        #ifndef NOOP_FFI
            try {
                m->m.SetProperty_Int64(schemaNS, propName, propValue, options);
            }
            catch (XMP_Error& e) {
                copyErrorForResult(e, outError);
            }
            catch (...) {
                signalUnknownError(outError);
            }
        #endif
    }

    void CXmpMetaSetProperty_Float(CXmpMeta* m,
                                   CXmpError* outError,
                                   const char* schemaNS,
                                   const char* propName,
                                   double propValue,
                                   AdobeXMPCommon::uint32 options) {
        #ifndef NOOP_FFI
            try {
                m->m.SetProperty_Float(schemaNS, propName, propValue, options);
            }
            catch (XMP_Error& e) {
                copyErrorForResult(e, outError);
            }
            catch (...) {
                signalUnknownError(outError);
            }
        #endif
    }

    void CXmpMetaSetProperty_Date(CXmpMeta* m,
                                  CXmpError* outError,
                                  const char* schemaNS,
                                  const char* propName,
                                  const XMP_DateTime* propValue,
                                  AdobeXMPCommon::uint32 options) {
        #ifndef NOOP_FFI
            try {
                m->m.SetProperty_Date(schemaNS, propName, *propValue, options);
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
        #ifndef NOOP_FFI
            try {
                return (m->m.DoesPropertyExist(schemaNS, propName)) ? 1 : 0;
            }
            catch (...) {
                // Intentional no-op.
            }
        #endif

        return 0;
    }

    const char* CXmpMetaGetArrayItem(CXmpMeta* m,
                                     CXmpError* outError,
                                     const char* schemaNS,
                                     const char* propName,
                                     AdobeXMPCommon::uint32 index,
                                     AdobeXMPCommon::uint32* outOptions) {
        #ifdef NOOP_FFI
            *outOptions = 0;
            return NULL;
        #else
            std::string propValue;

            try {
                if (m->m.GetArrayItem(schemaNS, propName, index, &propValue, outOptions)) {
                    return copyStringForResult(propValue);
                } else {
                    *outOptions = 0;
                    return NULL;
                }
            }
            catch (XMP_Error& e) {
                copyErrorForResult(e, outError);
                return NULL;
            }
            catch (...) {
                signalUnknownError(outError);
                return NULL;
            }
        #endif
    }

    // --- CXmpDateTime ---

    void CXmpDateTimeCurrent(XMP_DateTime* dt, CXmpError* outError) {
        #ifndef NOOP_FFI
            try {
                if (dt) {
                    SXMPUtils::CurrentDateTime(dt);
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
}
