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

use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub(crate) fn fixture_path(name: &str) -> String {
    let root_dir = &env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut path = PathBuf::from(root_dir);
    path.push("src/tests/fixtures");
    path.push(name);

    assert!(path.exists());

    path.to_str().unwrap().to_string()
}

pub(crate) fn temp_copy_of_fixture(tempdir: &Path, name: &str) -> String {
    let fixture_src = fixture_path(name);
    let fixture_path = Path::join(tempdir, name);
    let fixture_copy = fixture_path.as_path();

    fs::copy(fixture_src, fixture_copy).unwrap();
    fixture_copy.display().to_string()
}

pub(crate) const PURPLE_SQUARE_XMP: &str = r#"<x:xmpmeta xmlns:x="adobe:ns:meta/" x:xmptk="Adobe XMP Core 4.0-c003 (debug), build -num-, -date-">
        <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
        <rdf:Description rdf:about=""
        xmlns:dc="http://purl.org/dc/elements/1.1/"
        xmlns:xap="http://ns.adobe.com/xap/1.0/"
        xmlns:xapMM="http://ns.adobe.com/xap/1.0/mm/"
        xmlns:tiff="http://ns.adobe.com/tiff/1.0/"
        xmlns:exif="http://ns.adobe.com/exif/1.0/"
        xmlns:photoshop="http://ns.adobe.com/photoshop/1.0/"
        xmlns:pdf="http://ns.adobe.com/pdf/1.3/"
        xmlns:pdfx="http://ns.adobe.com/pdfx/1.3/"
        xmlns:xapRights="http://ns.adobe.com/xap/1.0/rights/"
        dc:format="application/vnd.adobe.photoshop"
        xap:CreatorTool="Adobe Photoshop CS2 Windows"
        xap:CreateDate="2006-04-25T15:32:01+02:00"
        xap:ModifyDate="2006-04-27T15:38:36.655+02:00"
        xap:MetadataDate="2006-04-26T16:47:10+02:00"
        xapMM:DocumentID="uuid:FE607D9B5FD4DA118B7787757E22306B"
        xapMM:InstanceID="uuid:BF664E7B33D5DA119129F691B53239AD"
        tiff:Orientation="1"
        tiff:XResolution="720000/10000"
        tiff:YResolution="720000/10000"
        tiff:ResolutionUnit="2"
        tiff:NativeDigest="256,257,258,259,262,274,277,284,530,531,282,283,296,301,318,319,529,532,306,270,271,272,305,315,33432;6F0EC2A1D6ADFA4DF4BB00D7C83AFAC0"
        exif:PixelXDimension="200"
        exif:PixelYDimension="200"
        exif:ColorSpace="-1"
        exif:NativeDigest="36864,40960,40961,37121,37122,40962,40963,37510,40964,36867,36868,33434,33437,34850,34852,34855,34856,37377,37378,37379,37380,37381,37382,37383,37384,37385,37386,37396,41483,41484,41486,41487,41488,41492,41493,41495,41728,41729,41730,41985,41986,41987,41988,41989,41990,41991,41992,41993,41994,41995,41996,42016,0,2,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,20,22,23,24,25,26,27,28,30;D891A8B493E755131A3267739F6277DB"
        photoshop:ColorMode="3"
        photoshop:ICCProfile="Dell 1905FP Color Profile"
        photoshop:CaptionWriter="Stefan"
        photoshop:History=""
        pdf:Keywords="&quot;XMP  metadata  schema XML RDF&quot;"
        pdf:Copyright="2005 Adobe Systems Inc."
        pdfx:Copyright="2005 Adobe Systems Inc."
        xapRights:Marked="False">
        <dc:description>
        <rdf:Alt>
            <rdf:li xml:lang="x-default">a test file (öäüßÖÄÜ€中文)</rdf:li>
        </rdf:Alt>
        </dc:description>
        <dc:title>
        <rdf:Alt>
            <rdf:li xml:lang="x-default">Purple Square</rdf:li>
        </rdf:Alt>
        </dc:title>
        <dc:creator>
        <rdf:Seq>
            <rdf:li>Llywelyn</rdf:li>
        </rdf:Seq>
        </dc:creator>
        <dc:subject>
        <rdf:Bag>
            <rdf:li>purple</rdf:li>
            <rdf:li>square</rdf:li>
            <rdf:li>Stefan</rdf:li>
            <rdf:li>XMP</rdf:li>
            <rdf:li>XMPFiles</rdf:li>
            <rdf:li>test</rdf:li>
        </rdf:Bag>
        </dc:subject>
        </rdf:Description>
        </rdf:RDF>
        </x:xmpmeta>
        "#;

pub(crate) const STRUCT_EXAMPLE: &str = r#"
        <x:xmpmeta xmlns:x="adobe:ns:meta/" x:xmptk="Adobe XMP Core 7.0-c000 1.000000, 0000/00/00-00:00:00">
        <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
            <rdf:Description rdf:about=""
                xmlns:xmp="http://ns.adobe.com/xap/1.0/"
                xmlns:xmpRights="http://ns.adobe.com/xap/1.0/rights/"
                xmlns:Iptc4xmpCore="http://iptc.org/std/Iptc4xmpCore/1.0/xmlns/"
                xmpRights:Marked="True">
                <Iptc4xmpCore:CreatorContactInfo
                    Iptc4xmpCore:CiAdrPcode="98110"
                    Iptc4xmpCore:CiAdrCtry="US"/>
            </rdf:Description>
        </rdf:RDF>
    </x:xmpmeta>
    "#;

pub(crate) const ARRAY_EXAMPLE: &str = r#"
        <x:xmpmeta xmlns:x="adobe:ns:meta/" x:xmptk="Adobe XMP Core 7.0-c000 1.000000, 0000/00/00-00:00:00">
        <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
        <rdf:Description rdf:about=""
        xmlns:dc="http://purl.org/dc/elements/1.1/">
        <dc:subject>
        <rdf:Bag>
            <rdf:li>purple</rdf:li>
            <rdf:li>square</rdf:li>
            <rdf:li>Stefan</rdf:li>
            <rdf:li>XMP</rdf:li>
            <rdf:li>XMPFiles</rdf:li>
            <rdf:li>test</rdf:li>
        </rdf:Bag>
        </dc:subject>
        </rdf:Description>
        </rdf:RDF>
        </x:xmpmeta>
        "#;

pub(crate) const QUAL_EXAMPLE: &str = r#"
        <rdf:RDF xmlns:rdf='http://www.w3.org/1999/02/22-rdf-syntax-ns#'>
            <rdf:Description rdf:about='Test:XMPCoreCoverage/kRDFCoverage' xmlns:ns1='ns:test1/' xmlns:ns2='ns:test2/'>

                <ns1:SimpleProp1>Simple1 value</ns1:SimpleProp1>
                <ns1:SimpleProp2 xml:lang='x-default'>Simple2 value</ns1:SimpleProp2>

                <ns1:QualProp1 rdf:parseType='Resource'>
                    <rdf:value>Prop value</rdf:value>
                    <ns2:Qual>Qual value</ns2:Qual>
                </ns1:QualProp1>

                <ns1:QualProp2 rdf:parseType='Resource'>
                    <rdf:value xml:lang='x-default'>Prop value</rdf:value>
                    <ns2:Qual>Qual value</ns2:Qual>
                </ns1:QualProp2>

                <ns1:QualProp4 xml:lang='x-default' rdf:parseType='Resource'>
                    <ns2:Field1>Field1 value</ns2:Field1>
                    <ns2:Field2>Field2 value</ns2:Field2>
                </ns1:QualProp4>

                <ns1:QualProp5 xml:lang='x-default'>
                    <rdf:Bag>
                        <rdf:li>Item1.1 value</rdf:li>
                        <rdf:li>Item1.2 value</rdf:li>
                    </rdf:Bag>
                </ns1:QualProp5>
            </rdf:Description>
        </rdf:RDF>
        "#;

pub(crate) const LOCALIZED_TEXT_EXAMPLE: &str = r#"<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
        xmlns:dc="http://purl.org/dc/elements/1.1/">
        <rdf:Description rdf:about="">
            <dc:title>
                <rdf:Alt>
                    <rdf:li xml:lang="x-default">XMP - Extensible Metadata Platform</rdf:li>
                    <rdf:li xml:lang="en-us">XMP - Extensible Metadata Platform (US English)</rdf:li>
                    <rdf:li xml:lang="fr">XMP - Une Platforme Extensible pour les Métadonnées</rdf:li>
                </rdf:Alt>
            </dc:title>
        </rdf:Description>
    </rdf:RDF>"#;

// NOTE: Not using r# syntax here because we need the CR/LF chars
// in these values to be parsed as such.
pub(crate) const NEWLINE_RDF: &str =
    "<rdf:RDF xmlns:rdf='http://www.w3.org/1999/02/22-rdf-syntax-ns#'>
<rdf:Description rdf:about='Test:XMPCoreCoverage/kNewlineRDF' xmlns:ns1='ns:test1/'>

  <ns1:HasCR>ASCII \u{D} CR</ns1:HasCR>
  <ns1:HasLF>ASCII \u{A} LF</ns1:HasLF>
  <ns1:HasCRLF>ASCII \u{D}\u{A} CRLF</ns1:HasCRLF>

</rdf:Description>
</rdf:RDF>";

pub(crate) const INCONSISTENT_RDF: &str = r#"<rdf:RDF xmlns:rdf='http://www.w3.org/1999/02/22-rdf-syntax-ns#'>
      <rdf:Description rdf:about='Test:XMPCoreCoverage/kInconsistentRDF'
                       xmlns:pdf='http://ns.adobe.com/pdf/1.3/'
                       xmlns:xmp='http://ns.adobe.com/xap/1.0/'
                       xmlns:dc='http://purl.org/dc/elements/1.1/'>
    
        <pdf:Author>PDF Author</pdf:Author>
        <xmp:Author>XMP Author</xmp:Author>
    
        <xmp:Authors>
          <rdf:Seq>
            <rdf:li>XMP Authors [1]</rdf:li>
          </rdf:Seq>
        </xmp:Authors>
    
        <dc:creator>
          <rdf:Seq>
            <rdf:li>DC Creator [1]</rdf:li>
          </rdf:Seq>
        </dc:creator>
    
      </rdf:Description>
    </rdf:RDF>"#;

pub(crate) const RDF_COVERAGE: &str = r#"<rdf:RDF xmlns:rdf='http://www.w3.org/1999/02/22-rdf-syntax-ns#'>                
        <rdf:Description rdf:about='Test:XMPCoreCoverage/kRDFCoverage' xmlns:ns1='ns:test1/' xmlns:ns2='ns:test2/'>              
                    
            <ns1:SimpleProp1>Simple1 value</ns1:SimpleProp1>                
            <ns1:SimpleProp2 xml:lang='x-default'>Simple2 value</ns1:SimpleProp2>                
                    
            <ns1:ArrayProp1>                
                <rdf:Bag>                
                    <rdf:li>Item1.1 value</rdf:li>              
                    <rdf:li>Item1.2 value</rdf:li>              
                </rdf:Bag>              
            </ns1:ArrayProp1>                
                    
            <ns1:ArrayProp2>                
                <rdf:Alt>                
                    <rdf:li xml:lang='x-one'>Item2.1 value</rdf:li>              
                    <rdf:li xml:lang='x-two'>Item2.2 value</rdf:li>              
                </rdf:Alt>              
            </ns1:ArrayProp2>                
                    
            <ns1:ArrayProp3>                
                <rdf:Alt>                
                    <rdf:li xml:lang='x-one'>Item3.1 value</rdf:li>              
                    <rdf:li>Item3.2 value</rdf:li>              
                </rdf:Alt>              
            </ns1:ArrayProp3>                
                    
            <ns1:ArrayProp4>                
                <rdf:Alt>                
                    <rdf:li>Item4.1 value</rdf:li>              
                    <rdf:li xml:lang='x-two'>Item4.2 value</rdf:li>              
                </rdf:Alt>              
            </ns1:ArrayProp4>                
                    
            <ns1:ArrayProp5>                
                <rdf:Alt>                
                    <rdf:li xml:lang='x-xxx'>Item5.1 value</rdf:li>              
                    <rdf:li xml:lang='x-xxx'>Item5.2 value</rdf:li>              
                </rdf:Alt>              
            </ns1:ArrayProp5>                
                    
            <ns1:StructProp rdf:parseType='Resource'>                
                <ns2:Field1>Field1 value</ns2:Field1>                
                <ns2:Field2>Field2 value</ns2:Field2>                
            </ns1:StructProp>                
                    
            <ns1:QualProp1 rdf:parseType='Resource'>                
                <rdf:value>Prop value</rdf:value>                
                <ns2:Qual>Qual value</ns2:Qual>              
            </ns1:QualProp1>                
                    
            <ns1:QualProp2 rdf:parseType='Resource'>                
                <rdf:value xml:lang='x-default'>Prop value</rdf:value>              
                <ns2:Qual>Qual value</ns2:Qual>              
            </ns1:QualProp2>                
                    
            <!-- NOTE: QualProp3 is not quite kosher. Normally a qualifier on a struct is attached to the -->                
            <!-- struct node in the XMP tree, and the same for an array. See QualProp4 and QualProp5. But -->                
            <!-- for the pseudo-struct of a qualified simple property there is no final struct node that    -->              
            <!-- can own the qualifier. Instead the qualifier is attached to the value. The alternative     -->              
            <!-- of attaching the qualifier to the value and all other qualifiers is not compelling. This -->                
            <!-- issue only arises for xml:lang, it is the only qualifier that RDF has as an attribute.     -->              
                    
            <ns1:QualProp3 xml:lang='x-default' rdf:parseType='Resource'>                
                <rdf:value>Prop value</rdf:value>                
                <ns2:Qual>Qual value</ns2:Qual>              
            </ns1:QualProp3>                
                    
            <ns1:QualProp4 xml:lang='x-default' rdf:parseType='Resource'>                
                <ns2:Field1>Field1 value</ns2:Field1>                
                <ns2:Field2>Field2 value</ns2:Field2>                
            </ns1:QualProp4>                
                    
            <ns1:QualProp5 xml:lang='x-default'>                
                <rdf:Bag>                
                    <rdf:li>Item1.1 value</rdf:li>              
                    <rdf:li>Item1.2 value</rdf:li>              
                </rdf:Bag>              
            </ns1:QualProp5>                
                    
            <ns2:NestedStructProp rdf:parseType='Resource'>              
                <ns1:Outer rdf:parseType='Resource'>                
                    <ns1:Middle rdf:parseType='Resource'>                
                        <ns1:Inner rdf:parseType='Resource'>                
                            <ns1:Field1>Field1 value</ns1:Field1>                
                            <ns2:Field2>Field2 value</ns2:Field2>                
                        </ns1:Inner>                
                    </ns1:Middle>                
                </ns1:Outer>                
            </ns2:NestedStructProp>              
                    
        </rdf:Description>              
    </rdf:RDF>"#;
