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
static volatile bool xmp_init_succeeded = false;

inline void init_xmp_fn() {
    #ifndef NOOP_FFI
        try {
            SXMPMeta::Initialize();
            SXMPFiles::Initialize(kXMPFiles_IgnoreLocalText);
            xmp_init_succeeded = true;
        }
        catch (XMP_Error& e) {
            fprintf(stderr, "Failed to initialize XMP Toolkit: %s\n", e.GetErrMsg());
        }
    #endif
}

static bool init_xmp() {
    std::call_once(xmp_init_flag, init_xmp_fn);
    return xmp_init_succeeded;
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
            debugMessage = NULL;
            reset();
        }

        void reset() {
            hadError = 0;
            id = 0;
            free((void*) debugMessage);
            debugMessage = NULL;
        }
    } CXmpError;
}

static void copyErrorForResult(XMP_Error& e, CXmpError* outError) {
    if (outError) {
        outError->hadError = 1;
        outError->id = e.GetID();
        free((void*) outError->debugMessage);
        outError->debugMessage = copyStringForResult(e.GetErrMsg());
    }
}

static void signalUnknownError(CXmpError* outError) {
    if (outError) {
        outError->hadError = 1;
        outError->id = kXMPErr_Unknown;
        free((void*) outError->debugMessage);
        outError->debugMessage = NULL;
    }
}

static void signalXmpInitFailure(CXmpError* outError) {
    if (outError) {
        outError->hadError = 1;
        outError->id = kXMPErr_Unknown;
        free((void*) outError->debugMessage);
        outError->debugMessage = copyStringForResult("XMP Toolkit unavailable");
    }
}

static bool xmpFileErrorCallback(void* context,
                                 XMP_StringPtr filePath,
                                 XMP_ErrorSeverity severity,
                                 AdobeXMPCommon::int32 cause,
                                 XMP_StringPtr message) {
    CXmpError* err = (CXmpError*) context;
    if (err) {
        err->hadError = 1;
        err->id = cause;
        free((void*) err->debugMessage);
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

    typedef struct CXmpIterator {
        #ifdef NOOP_FFI
            int x;
        #else
            CXmpIterator(SXMPIterator i): i(i) {}
            SXMPIterator i;
        #endif
    } CXmpIterator;

    const char* CXmpStringCopy(const char* str) {
        // This function should be used *only* to test FFI behavior.
        // It copies a Rust-originated string so that it can subsequently
        // be used for CXmpStringDrop.
        return copyStringForResult(str);
    }

    void CXmpStringDrop(void* str) {
        // This function must be called from Rust FFI code for every
        // const char* sent over to Rust (i.e. generated via copyStringForResult).
        free(str);
    }

    // --- CXmpFile ---

    CXmpFile* CXmpFileNew(CXmpError* outError) {
        #ifndef NOOP_FFI
            if (!init_xmp()) {
                signalXmpInitFailure(outError);
                return NULL;
            }
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
            if (!init_xmp()) {
                signalXmpInitFailure(outError);
                return NULL;
            }
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

    CXmpMeta* CXmpMetaClone(CXmpMeta* m,
                            CXmpError* outError) {
        #ifndef NOOP_FFI
            try {
                CXmpMeta* result = new CXmpMeta;
                result->m = m->m.Clone();
                return result;
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

    CXmpMeta* CXmpMetaParseFromBuffer(CXmpError* outError,
                                      const char* buffer,
                                      AdobeXMPCommon::uint32 buffer_size,
                                      AdobeXMPCommon::uint32 options) {
        #ifndef NOOP_FFI
            if (!init_xmp()) {
                signalXmpInitFailure(outError);
                return NULL;
            }
            CXmpMeta* result = new CXmpMeta;

            try {
                result->m.ParseFromBuffer(buffer, buffer_size, options);
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

    const char* CXmpMetaSerializeToBuffer(CXmpMeta* m,
                                          CXmpError* outError,
                                          AdobeXMPCommon::uint32 options,
                                          AdobeXMPCommon::uint32 padding,
                                          const char* newline,
                                          const char* indent,
                                          AdobeXMPCommon::uint32 baseIndent) {
        #ifndef NOOP_FFI
            if (!init_xmp()) {
                signalXmpInitFailure(outError);
                return NULL;
            }

            try {
                std::string buffer;
                m->m.SerializeToBuffer(&buffer, options, padding, newline, indent, baseIndent);
                return copyStringForResult(buffer);
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

    const char* CXmpMetaRegisterNamespace(CXmpError* outError,
                                          const char* namespaceURI,
                                          const char* suggestedPrefix) {
        #ifndef NOOP_FFI
            if (!init_xmp()) {
                signalXmpInitFailure(outError);
                return NULL;
            }

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

    const char* CXmpMetaGetNamespacePrefix(CXmpError* outError,
                                           const char* namespaceURI) {
        #ifndef NOOP_FFI
            if (!init_xmp()) {
                signalXmpInitFailure(outError);
                return NULL;
            }

            try {
                std::string outPrefix;
                if (SXMPMeta::GetNamespacePrefix(namespaceURI, &outPrefix)) {
                    return copyStringForResult(outPrefix);
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

    const char* CXmpMetaGetNamespaceURI(CXmpError* outError,
                                        const char* namespacePrefix) {
        #ifndef NOOP_FFI
            if (!init_xmp()) {
                signalXmpInitFailure(outError);
                return NULL;
            }

            try {
                std::string outURI;
                if (SXMPMeta::GetNamespaceURI(namespacePrefix, &outURI)) {
                    return copyStringForResult(outURI);
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

    void CXmpDumpNamespaces(void* rustString, XMP_TextOutputProc callback) {
        #ifndef NOOP_FFI
            if (init_xmp()) {
                try {
                    SXMPMeta::DumpNamespaces(callback, rustString);
                }
                catch (...) {
                    // intentional no-op
                }
            }
        #endif
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

    void CXmpMetaDeleteProperty(CXmpMeta* m,
                                CXmpError* outError,
                                const char* schemaNS,
                                const char* propName) {
        #ifndef NOOP_FFI
            try {
                m->m.DeleteProperty(schemaNS, propName);
            }
            catch (XMP_Error& e) {
                copyErrorForResult(e, outError);
            }
            catch (...) {
                signalUnknownError(outError);
            }
        #endif
    }

    void CXmpMetaSetArrayItem(CXmpMeta* m,
                              CXmpError* outError,
                              const char* schemaNS,
                              const char* arrayName,
                              AdobeXMPCommon::uint32 itemIndex,
                              const char* itemValue,
                              AdobeXMPCommon::uint32 itemOptions) {
        #ifndef NOOP_FFI
            try {
                m->m.SetArrayItem(schemaNS, arrayName, itemIndex, itemValue, itemOptions);
            }
            catch (XMP_Error& e) {
                copyErrorForResult(e, outError);
            }
            catch (...) {
                signalUnknownError(outError);
            }
        #endif
    }

    void CXmpMetaAppendArrayItem(CXmpMeta* m,
                                 CXmpError* outError,
                                 const char* schemaNS,
                                 const char* arrayName,
                                 AdobeXMPCommon::uint32 arrayOptions,
                                 const char* itemValue,
                                 AdobeXMPCommon::uint32 itemOptions) {
        #ifndef NOOP_FFI
            try {
                m->m.AppendArrayItem(schemaNS, arrayName, arrayOptions, itemValue, itemOptions);
            }
            catch (XMP_Error& e) {
                copyErrorForResult(e, outError);
            }
            catch (...) {
                signalUnknownError(outError);
            }
        #endif
    }

    void CXmpMetaDeleteArrayItem(CXmpMeta* m,
                                 CXmpError* outError,
                                 const char* schemaNS,
                                 const char* arrayName,
                                 AdobeXMPCommon::uint32 itemIndex) {
        #ifndef NOOP_FFI
            try {
                m->m.DeleteArrayItem(schemaNS, arrayName, itemIndex);
            }
            catch (XMP_Error& e) {
                copyErrorForResult(e, outError);
            }
            catch (...) {
                signalUnknownError(outError);
            }
        #endif
    }

	bool CXmpMetaCountArrayItems(CXmpMeta* m,
                                 CXmpError* outError,
                                 const char* arrayNS,
                                 const char* arrayName,
                                 AdobeXMPCommon::uint32* outValue) {
        *outValue = 0;

        #ifndef NOOP_FFI
            try {
                *outValue = m->m.CountArrayItems(arrayNS, arrayName);
                return true;
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

    const char* CXmpMetaGetStructField(CXmpMeta* m,
                                       CXmpError* outError,
                                       const char* schemaNS,
                                       const char* structName,
                                       const char* fieldNS,
                                       const char* fieldName,
                                       AdobeXMPCommon::uint32* outOptions) {
        *outOptions = 0;

        #ifndef NOOP_FFI
            try {
                std::string propValue;
                if (m->m.GetStructField(schemaNS, structName, fieldNS, fieldName,
                                        &propValue, outOptions)) {
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

    void CXmpMetaSetStructField(CXmpMeta* m,
                                CXmpError* outError,
                                const char* schemaNS,
                                const char* structName,
                                const char* fieldNS,
                                const char* fieldName,
                                const char* itemValue,
                                AdobeXMPCommon::uint32 itemOptions) {
        #ifndef NOOP_FFI
            try {
                m->m.SetStructField(schemaNS, structName, fieldNS, fieldName,
                                    itemValue, itemOptions);
            }
            catch (XMP_Error& e) {
                copyErrorForResult(e, outError);
            }
            catch (...) {
                signalUnknownError(outError);
            }
        #endif
    }

    void CXmpMetaDeleteStructField(CXmpMeta* m,
                                   CXmpError* outError,
                                   const char* schemaNS,
                                   const char* structName,
                                   const char* fieldNS,
                                   const char* fieldName) {
        #ifndef NOOP_FFI
            try {
                m->m.DeleteStructField(schemaNS, structName, fieldNS, fieldName);
            }
            catch (XMP_Error& e) {
                copyErrorForResult(e, outError);
            }
            catch (...) {
                signalUnknownError(outError);
            }
        #endif
    }

    const char* CXmpMetaGetQualifier(CXmpMeta* m,
                                     CXmpError* outError,
                                     const char* schemaNS,
                                     const char* propName,
                                     const char* qualNS,
                                     const char* qualName,
                                     AdobeXMPCommon::uint32* outOptions) {
        *outOptions = 0;

        #ifndef NOOP_FFI
            try {
                std::string qualValue;
                if (m->m.GetQualifier(schemaNS, propName, qualNS, qualName,
                                      &qualValue, outOptions)) {
                    return copyStringForResult(qualValue);
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

    void CXmpMetaSetQualifier(CXmpMeta* m,
                              CXmpError* outError,
                              const char* schemaNS,
                              const char* propName,
                              const char* qualNS,
                              const char* qualName,
                              const char* qualValue,
                              AdobeXMPCommon::uint32 qualOptions) {
        #ifndef NOOP_FFI
            try {
                m->m.SetQualifier(schemaNS, propName, qualNS, qualName,
                                  qualValue, qualOptions);
            }
            catch (XMP_Error& e) {
                copyErrorForResult(e, outError);
            }
            catch (...) {
                signalUnknownError(outError);
            }
        #endif
    }

    void CXmpMetaDeleteQualifier(CXmpMeta* m,
                                 CXmpError* outError,
                                 const char* propNS,
                                 const char* propName,
                                 const char* qualNS,
                                 const char* qualName) {
        #ifndef NOOP_FFI
            try {
                m->m.DeleteQualifier(propNS, propName, qualNS, qualName);
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

    int CXmpMetaDoesStructFieldExist(CXmpMeta* m,
                                  const char* schemaNS,
                                  const char* structName,
                                  const char* fieldNS,
                                  const char* fieldName) {
        #ifndef NOOP_FFI
            try {
                return (m->m.DoesStructFieldExist(schemaNS, structName, fieldNS, fieldName)) ? 1 : 0;
            }
            catch (...) {
                // Intentional no-op.
            }
        #endif

        return 0;
    }

    int CXmpMetaDoesQualifierExist(CXmpMeta* m,
                                   const char* schemaNS,
                                   const char* propName,
                                   const char* qualNS,
                                   const char* qualName) {
        #ifndef NOOP_FFI
            try {
                return (m->m.DoesQualifierExist(schemaNS, propName, qualNS, qualName)) ? 1 : 0;
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

    const char* CXmpMetaGetLocalizedText(CXmpMeta* m,
                                         CXmpError* outError,
                                         const char* schemaNS,
                                         const char* altTextName,
                                         const char* genericLang,
                                         const char* specificLang,
                                         const char** actualLang,
                                         AdobeXMPCommon::uint32* outOptions) {
        *outOptions = 0;

        #ifndef NOOP_FFI
            try {
                std::string propValue;
                std::string outActualLang;
                if (m->m.GetLocalizedText(schemaNS,
                                          altTextName,
                                          genericLang,
                                          specificLang,
                                          &outActualLang,
                                          &propValue,
                                          outOptions)) {
                    *actualLang = copyStringForResult(outActualLang);
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

    void CXmpMetaSetLocalizedText(CXmpMeta* m,
                                  CXmpError* outError,
                                  const char* schemaNS,
                                  const char* altTextName,
                                  const char* genericLang,
                                  const char* specificLang,
                                  const char* itemValue,                                  const char** actualLang,
                                  AdobeXMPCommon::uint32 options) {
        #ifndef NOOP_FFI
            try {
                m->m.SetLocalizedText(schemaNS,
                                      altTextName,
                                      genericLang,
                                      specificLang,
                                      itemValue,
                                      options);
            }
            catch (XMP_Error& e) {
                copyErrorForResult(e, outError);
            }
            catch (...) {
                signalUnknownError(outError);
            }
        #endif
    }

    void CXmpMetaSort(CXmpMeta* m, CXmpError* outError) {
        #ifndef NOOP_FFI
            try {
                m->m.Sort();
            }
            catch (XMP_Error& e) {
                copyErrorForResult(e, outError);
            }
            catch (...) {
                signalUnknownError(outError);
            }
        #endif
    }

    const char* CXmpMetaGetObjectName(CXmpMeta* m, CXmpError* outError) {
        #ifndef NOOP_FFI
            try {
                std::string name;
                m->m.GetObjectName(&name);
                return copyStringForResult(name);
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

    void CXmpMetaSetObjectName(CXmpMeta* m,
                               CXmpError* outError,
                               const char* name) {
        #ifndef NOOP_FFI
            try {
                m->m.SetObjectName(name);
            }
            catch (XMP_Error& e) {
                copyErrorForResult(e, outError);
            }
            catch (...) {
                signalUnknownError(outError);
            }
        #endif
    }

    const char* CXmpMetaComposeArrayItemPath(CXmpError* outError,
                                             const char* schemaNS,
                                             const char* arrayName,
                                             AdobeXMPCommon::int32 index) {
        #ifndef NOOP_FFI
            if (!init_xmp()) {
                signalXmpInitFailure(outError);
                return NULL;
            }

            try {
                std::string resultPath;
                SXMPUtils::ComposeArrayItemPath(schemaNS,
                                                arrayName,
                                                index,
                                                &resultPath);

                return copyStringForResult(resultPath);
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

    const char* CXmpMetaComposeLangSelector(CXmpError* outError,
                                            const char* schemaNS,
                                            const char* arrayName,
                                            const char* langName) {
        #ifndef NOOP_FFI
            if (!init_xmp()) {
                signalXmpInitFailure(outError);
                return NULL;
            }

            try {
                std::string resultPath;
                SXMPUtils::ComposeLangSelector(schemaNS,
                                               arrayName,
                                               langName,
                                               &resultPath);

                return copyStringForResult(resultPath);
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

    const char* CXmpMetaComposeFieldSelector(CXmpError* outError,
                                             const char* schemaNS,
                                             const char* structName,
                                             const char* fieldNS,
                                             const char* fieldName,
                                             const char* fieldValue) {
        #ifndef NOOP_FFI
            if (!init_xmp()) {
                signalXmpInitFailure(outError);
                return NULL;
            }

            try {
                std::string resultPath;
                SXMPUtils::ComposeFieldSelector(schemaNS,
                                                structName,
                                                fieldNS,
                                                fieldName,
                                                fieldValue,
                                                &resultPath);

                return copyStringForResult(resultPath);
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

    const char* CXmpMetaComposeQualifierPath(CXmpError* outError,
                                             const char* schemaNS,
                                             const char* structName,
                                             const char* qualNS,
                                             const char* qualName) {
        #ifndef NOOP_FFI
            if (!init_xmp()) {
                signalXmpInitFailure(outError);
                return NULL;
            }

            try {
                std::string resultPath;
                SXMPUtils::ComposeQualifierPath(schemaNS,
                                                structName,
                                                qualNS,
                                                qualName,
                                                &resultPath);

                return copyStringForResult(resultPath);
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

    const char* CXmpMetaComposeStructFieldPath(CXmpError* outError,
                                               const char* schemaNS,
                                               const char* structName,
                                               const char* fieldNS,
                                               const char* fieldName) {
        #ifndef NOOP_FFI
            if (!init_xmp()) {
                signalXmpInitFailure(outError);
                return NULL;
            }

            try {
                std::string resultPath;
                SXMPUtils::ComposeStructFieldPath(schemaNS,
                                                  structName,
                                                  fieldNS,
                                                  fieldName,
                                                  &resultPath);

                return copyStringForResult(resultPath);
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

    void CXmpMetaDumpObj(CXmpMeta* m,
                         void* rustString,
                         XMP_TextOutputProc callback) {
        #ifndef NOOP_FFI
            try {
                m->m.DumpObject(callback, rustString);
            }
            catch (...) {
                // intentional no-op
            }
        #endif
    }

    // --- CXmpIterator ---

    CXmpIterator* CXmpIteratorNew(CXmpMeta* m,
                                  CXmpError* outError,
                                  const char* schemaNS,
                                  const char* propName,
                                  AdobeXMPCommon::uint32 options) {
        #ifndef NOOP_FFI
            try {
                if (m) {
                    return new CXmpIterator(SXMPIterator(m->m, schemaNS, propName, options));
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

    void CXmpIteratorDrop(CXmpIterator* i) {
        #ifndef NOOP_FFI
            delete i;
        #endif
    }

    bool CXmpIteratorNext(CXmpIterator* i,
                          CXmpError* outError,
                          const char** outSchemaNS,
                          const char** outPropPath,
                          const char** outPropValue,
                          AdobeXMPCommon::uint32* outOptions) {
        #ifndef NOOP_FFI
            try {
                if (i) {
                    std::string schemaNS;
                    std::string propPath;
                    std::string propValue;
                    if (i->i.Next(&schemaNS, &propPath, &propValue, outOptions)) {
                        *outSchemaNS = copyStringForResult(schemaNS);
                        *outPropPath = copyStringForResult(propPath);
                        *outPropValue = copyStringForResult(propValue);
                        return true;
                    }
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

    void CXmpIteratorSkip(CXmpIterator* i,
                          CXmpError* outError,
                          AdobeXMPCommon::uint32 options) {
        #ifndef NOOP_FFI
            try {
                if (i) {
                    i->i.Skip(options);
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

    void CXmpDateTimeSetTimeZone(XMP_DateTime* dt, CXmpError* outError) {
        #ifndef NOOP_FFI
            if (!init_xmp()) {
                signalXmpInitFailure(outError);
                return;
            }

            try {
                if (dt) {
                    SXMPUtils::SetTimeZone(dt);
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

    void CXmpDateTimeConvertToLocalTime(XMP_DateTime* dt, CXmpError* outError) {
        #ifndef NOOP_FFI
            if (!init_xmp()) {
                signalXmpInitFailure(outError);
                return;
            }

            try {
                if (dt) {
                    SXMPUtils::ConvertToLocalTime(dt);
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

    void CXmpDateTimeConvertToUTCTime(XMP_DateTime* dt, CXmpError* outError) {
        #ifndef NOOP_FFI
            if (!init_xmp()) {
                signalXmpInitFailure(outError);
                return;
            }

            try {
                if (dt) {
                    SXMPUtils::ConvertToUTCTime(dt);
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

    const char* CXmpDateTimeToString(const XMP_DateTime* dt, CXmpError* outError) {
        #ifndef NOOP_FFI
            if (!init_xmp()) {
                signalXmpInitFailure(outError);
                return NULL;
            }

            try {
                if (dt) {
                    std::string dtAsString;
                    SXMPUtils::ConvertFromDate(*dt, &dtAsString);
                    return copyStringForResult(dtAsString);
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
}
