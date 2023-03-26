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

//! Standard XML namespace constants.

/// The XML namespace for the XMP "basic" schema.
pub const XMP: &str = "http://ns.adobe.com/xap/1.0/";

/// The XML namespace for the XMP copyright schema.
pub const XMP_RIGHTS: &str = "http://ns.adobe.com/xap/1.0/rights/";

/// The XML namespace for the XMP digital asset management schema.
pub const XMP_MM: &str = "http://ns.adobe.com/xap/1.0/mm/";

/// The XML namespace for the job management schema.
pub const XMP_BJ: &str = "http://ns.adobe.com/xap/1.0/bj/";

/// The XML namespace for the PDF schema.
pub const PDF: &str = "http://ns.adobe.com/pdf/1.3/";

/// The XML namespace for the Photoshop custom schema.
pub const PHOTOSHOP: &str = "http://ns.adobe.com/photoshop/1.0/";

/// The XML namespace for Adobe's Exif schema.
pub const EXIF: &str = "http://ns.adobe.com/exif/1.0/";

/// The XML namespace for Adobe's TIFF schema.
pub const TIFF: &str = "http://ns.adobe.com/tiff/1.0/";

// --- XML namespace constants for qualifiers and structured property fields ---

/// The XML namespace for qualifiers of the `xmp:Identifier` property.
pub const IDENTIFIER_QUAL: &str = "http://ns.adobe.com/xmp/Identifier/qual/1.0/";

/// The XML namespace for fields of the `Dimensions` type.
pub const DIMENSIONS: &str = "http://ns.adobe.com/xap/1.0/sType/Dimensions#";

/// The XML namespace for fields of a graphical image. Used for the `Thumbnail`
/// type.
pub const IMAGE: &str = "http://ns.adobe.com/xap/1.0/g/img/";

/// The XML namespace for fields of the `ResourceEvent` type.
pub const RESOURCE_EVENT: &str = "http://ns.adobe.com/xap/1.0/sType/ResourceEvent#";

/// The XML namespace for fields of the `ResourceRef` type.
pub const RESOURCE_REF: &str = "http://ns.adobe.com/xap/1.0/sType/ResourceRef#";

/// The XML namespace for fields of the `Version` type.
pub const ST_VERSION: &str = "http://ns.adobe.com/xap/1.0/sType/Version#";

/// The XML namespace for fields of the `JobRef` type.
pub const ST_JOB: &str = "http://ns.adobe.com/xap/1.0/sType/Job#";

// --- XML namespace constants from outside Adobe ---

/// The XML namespace for the Dublin Core schema.
pub const DC: &str = "http://purl.org/dc/elements/1.1/";

/// The XML namespace for the IPTC Core schema.
pub const IPTC_CORE: &str = "http://iptc.org/std/Iptc4xmpCore/1.0/xmlns/";

/// The XML namespace for the IPTC Extension schema.
pub const IPTC_EXT: &str = "http://iptc.org/std/Iptc4xmpExt/2008-02-29/";

/// The XML namespace for RDF.
pub const RDF: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";

/// The XML namespace for XML.
pub const XML: &str = "http://www.w3.org/XML/1998/namespace";
