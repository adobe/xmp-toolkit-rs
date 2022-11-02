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

// This file is an adaptation of the file XMPCoreCoverage.cpp
// from the C++ XMP Toolkit.

// Demonstrates syntax and usage by exercising most of the API
// functions of XMPCore Toolkit SDK component, using a sample
// XMP Packet that contains all of the different property and
// attribute types.

#![allow(dead_code)] // TEMPORARY while in development

use std::{str::FromStr, string::ToString};

use crate::{
    xmp_ns, xmp_value::xmp_prop, ItemPlacement, ToStringOptions, XmpDate, XmpDateTime, XmpMeta,
    XmpTime, XmpTimeZone, XmpValue,
};

const NS1: &str = "ns:test1/";
const NS2: &str = "ns:test2/";

const RDF_COVERAGE: &str = r#"<rdf:RDF xmlns:rdf='http://www.w3.org/1999/02/22-rdf-syntax-ns#'>
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
        <!-- for the pseudo-struct of a qualified simple property there is no final struct node that  -->
        <!-- can own the qualifier. Instead the qualifier is attached to the value. The alternative   -->
        <!-- of attaching the qualifier to the value and all other qualifiers is not compelling. This -->
        <!-- issue only arises for xml:lang, it is the only qualifier that RDF has as an attribute.   -->
    
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

const SIMPLE_RDF: &str = r#"<rdf:RDF xmlns:rdf='http://www.w3.org/1999/02/22-rdf-syntax-ns#'>
      <rdf:Description rdf:about='Test:XMPCoreCoverage/kSimpleRDF' xmlns:ns1='ns:test1/' xmlns:ns2='ns:test2/'>
    
        <ns1:SimpleProp>Simple value</ns1:SimpleProp>
    
        <ns1:ArrayProp>
          <rdf:Bag>
            <rdf:li>Item1 value</rdf:li>
            <rdf:li>Item2 value</rdf:li>
          </rdf:Bag>
        </ns1:ArrayProp>
    
        <ns1:StructProp rdf:parseType='Resource'>
          <ns2:Field1>Field1 value</ns2:Field1>
          <ns2:Field2>Field2 value</ns2:Field2>
        </ns1:StructProp>
    
        <ns1:QualProp rdf:parseType='Resource'>
          <rdf:value>Prop value</rdf:value>
          <ns2:Qual>Qual value</ns2:Qual>
        </ns1:QualProp>
    
        <ns1:AltTextProp>
          <rdf:Alt>
            <rdf:li xml:lang='x-one'>x-one value</rdf:li>
            <rdf:li xml:lang='x-two'>x-two value</rdf:li>
          </rdf:Alt>
        </ns1:AltTextProp>
    
        <ns1:ArrayOfStructProp>
          <rdf:Bag>
            <rdf:li rdf:parseType='Resource'>
              <ns2:Field1>Item-1</ns2:Field1>
              <ns2:Field2>Field 1.2 value</ns2:Field2>
            </rdf:li>
            <rdf:li rdf:parseType='Resource'>
              <ns2:Field1>Item-2</ns2:Field1>
              <ns2:Field2>Field 2.2 value</ns2:Field2>
            </rdf:li>
          </rdf:Bag>
        </ns1:ArrayOfStructProp>
    
      </rdf:Description>
    </rdf:RDF>"#;

const NAMESPACE_RDF: &str = r#"<rdf:RDF xmlns:rdf='http://www.w3.org/1999/02/22-rdf-syntax-ns#'>
      <rdf:Description rdf:about='Test:XMPCoreCoverage/kNamespaceRDF' xmlns:ns1='ns:test1/'>
    
        <ns1:NestedStructProp rdf:parseType='Resource'>
          <ns2:Outer rdf:parseType='Resource' xmlns:ns2='ns:test2/' xmlns:ns3='ns:test3/'>
            <ns3:Middle rdf:parseType='Resource' xmlns:ns4='ns:test4/'>
              <ns4:Inner rdf:parseType='Resource' xmlns:ns5='ns:test5/' xmlns:ns6='ns:test6/'>
                <ns5:Field1>Field1 value</ns5:Field1>
                <ns6:Field2>Field2 value</ns6:Field2>
              </ns4:Inner>
            </ns3:Middle>
          </ns2:Outer>
        </ns1:NestedStructProp>
    
      </rdf:Description>
    </rdf:RDF>"#;

const XMP_META_RDF: &str = r#"<x:Outermost xmlns:x='adobe:ns:meta/'>
    
    <rdf:RDF xmlns:rdf='http://www.w3.org/1999/02/22-rdf-syntax-ns#'>
      <rdf:Description rdf:about='Test:XMPCoreCoverage/kBogusLeadingRDF' xmlns:ns1='ns:test1/'>
        <ns1:BogusLeadingProp>bogus packet</ns1:BogusLeadingProp>
      </rdf:Description>
    </rdf:RDF>
    
    <x:xmpmeta>
    <rdf:RDF xmlns:rdf='http://www.w3.org/1999/02/22-rdf-syntax-ns#'>
      <rdf:Description rdf:about='Test:XMPCoreCoverage/XMP_META_RDF' xmlns:ns1='ns:test1/'>
        <ns1:XMPMetaProp>xmpmeta packet</ns1:XMPMetaProp>
      </rdf:Description>
    </rdf:RDF>
    </x:xmpmeta>
    
    <rdf:RDF xmlns:rdf='http://www.w3.org/1999/02/22-rdf-syntax-ns#'>
      <rdf:Description rdf:about='Test:XMPCoreCoverage/kBogusTrailingRDF' xmlns:ns1='ns:test1/'>
        <ns1:BogusTrailingProp>bogus packet</ns1:BogusTrailingProp>
      </rdf:Description>
    </rdf:RDF>
    
    </x:Outermost>"#;

// NOTE: Not using r# syntax here because we need the CR/LF chars
// in these values to be parsed as such.
const NEWLINE_RDF: &str = "<rdf:RDF xmlns:rdf='http://www.w3.org/1999/02/22-rdf-syntax-ns#'>
      <rdf:Description rdf:about='Test:XMPCoreCoverage/kNewlineRDF' xmlns:ns1='ns:test1/'>
    
        <ns1:HasCR>ASCII \u{D} CR</ns1:HasCR>
        <ns1:HasLF>ASCII \u{A} LF</ns1:HasLF>
        <ns1:HasCRLF>ASCII \u{D}\u{A} CRLF</ns1:HasCRLF>
    
      </rdf:Description>
    </rdf:RDF>";

const INCONSISTENT_RDF: &str = r#"<rdf:RDF xmlns:rdf='http://www.w3.org/1999/02/22-rdf-syntax-ns#'>
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

const DATE_TIME_RDF: &str = r#"<rdf:RDF xmlns:rdf='http://www.w3.org/1999/02/22-rdf-syntax-ns#'>
      <rdf:Description rdf:about='Test:XMPCoreCoverage/kDateTimeRDF' xmlns:ns1='ns:test1/'>
    
        <ns1:Date1>2003</ns1:Date1>
        <ns1:Date2>2003-12</ns1:Date2>
        <ns1:Date3>2003-12-31</ns1:Date3>
    
        <ns1:Date4>2003-12-31T12:34Z</ns1:Date4>
        <ns1:Date5>2003-12-31T12:34:56Z</ns1:Date5>
    
        <ns1:Date6>2003-12-31T12:34:56.001Z</ns1:Date6>
        <ns1:Date7>2003-12-31T12:34:56.000000001Z</ns1:Date7>
    
        <ns1:Date8>2003-12-31T10:04:56-02:30</ns1:Date8>
        <ns1:Date9>2003-12-31T15:49:56+03:15</ns1:Date9>
    
      </rdf:Description>
    </rdf:RDF>"#;

// #define FoundOrNot(b)	((b) ? "found" : "not found")
// #define YesOrNo(b)		((b) ? "yes" : "no")

// // -------------------------------------------------------------------------------------------------

// static void FillDateTime ( XMP_DateTime * dateTime, XMP_Int32 year, XMP_Int32
// month, XMP_Int32 day, 						   XMP_Int32 hour, XMP_Int32 minute, XMP_Int32 second,
// 						   XMP_Bool hasDate, XMP_Bool hasTime, XMP_Bool hasTimeZone,
// 						   XMP_Int8 tzSign, XMP_Int32 tzHour, XMP_Int32 tzMinute, XMP_Int32
// nanoSecond ) {

// 	dateTime->year = year;
// 	dateTime->month = month;
// 	dateTime->day = day;
// 	dateTime->hour = hour;
// 	dateTime->minute = minute;
// 	dateTime->second = second;
// 	dateTime->hasDate = hasDate;
// 	dateTime->hasTime = hasTime;
// 	dateTime->hasTimeZone = hasTimeZone;
// 	dateTime->tzSign = tzSign;
// 	dateTime->tzHour = tzHour;
// 	dateTime->tzMinute = tzMinute;
// 	dateTime->nanoSecond = nanoSecond;

// }	// FillDateTime

fn write_major_label(title: &str) {
    println!();
    println!("// =============================================================================");
    println!("//  {}.", title);
    println!("// =============================================================================");
    println!();
}

// static void WriteMinorLabel ( FILE * log, const char * title )
// {

// 	fprintf ( log, "\n// " );
// 	for ( size_t i = 0; i < strlen(title); ++i ) fprintf ( log, "-" );
// 	fprintf ( log, "--\n// %s :\n\n", title );
// 	fflush ( log );

// }	// WriteMinorLabel

// // -------------------------------------------------------------------------------------------------

// static XMP_Status DumpToString ( void * refCon, XMP_StringPtr outStr,
// XMP_StringLen outLen ) {
// 	XMP_Status	status	= 0;
// 	std::string * dumpString = (std::string*)refCon;

// 	try {
// 		dumpString->append ( outStr, outLen );
// 	} catch ( ... ) {
// 		status = -1;
// 	}
// 	return status;

// }	// DumpToString

// // -------------------------------------------------------------------------------------------------

// static XMP_Status DumpToFile ( void * refCon, XMP_StringPtr outStr,
// XMP_StringLen outLen ) {
// 	XMP_Status	status	= 0;
// 	size_t		count;
// 	FILE *		outFile	= static_cast < FILE * > ( refCon );

// 	count = fwrite ( outStr, 1, outLen, outFile );
// 	if ( count != outLen ) status = errno;
// 	fflush ( outFile );
// 	return status;

// }	// DumpToFile

// // -------------------------------------------------------------------------------------------------

// static void DumpXMPObj ( FILE * log, SXMPMeta & meta, const char * title )
// {

// 	WriteMinorLabel ( log, title );
// 	meta.DumpObject ( DumpToFile, log );

// }	// DumpXMPObj

// // -------------------------------------------------------------------------------------------------

// static void VerifyNewlines ( FILE * log, std::string xmp, const char *
// newline ) {
// 	for ( size_t i = 0; i < xmp.size(); ++i ) {
// 		if ( (xmp[i] == '\x0A') || (xmp[i] == '\x0D') ) {
// 			if ( strncmp ( &xmp[i], newline, strlen(newline) ) != 0 ) {
// 				fprintf ( log, "** Wrong newline at offset %zd\n", i );
// 			}
// 			if ( strlen(newline) == 2 ) ++i;
// 		}
// 	}
// }

#[test]
fn xmp_core_coverage() {
    write_major_label("Test global XMP toolkit options");
    println!("SKIPPING: Global options not ported to Rust");

    //-------------------------------------------------------------------------

    write_major_label("Dump predefined namespaces");
    println!("{}", XmpMeta::debug_dump_namespaces());

    //-------------------------------------------------------------------------

    {
        write_major_label("Test simple constructors and parsing, setting the instance ID");

        let mut meta1 = XmpMeta::new().unwrap();
        println!("Empty XMP object = {:#?}", meta1);

        assert_eq!(format!("{:#?}", meta1), "XMPMeta object \"\"  (0x0)\n");

        let name = meta1.name();
        println!("Empty object name = \"{}\"", name);

        meta1.set_name("New object name").unwrap();
        println!("Set object name -> {:#?}", meta1);

        assert_eq!(
            format!("{:#?}", meta1),
            "XMPMeta object \"New object name\"  (0x0)\n"
        );

        let mut meta2 = XmpMeta::from_str(RDF_COVERAGE).unwrap();
        println!("Construct and parse from buffer = {:#?}", meta2);
        println!("RDFCoverage object name = {}", meta2.name());

        assert_eq!(meta2.name(), "Test:XMPCoreCoverage/kRDFCoverage");

        meta2
            .set_property(xmp_ns::XMP_MM, "InstanceID", &"meta2:original".into())
            .unwrap();

        println!("Add instance ID = {:#?}", meta2);

        let mut meta4 = meta2.clone();
        meta4
            .set_property(xmp_ns::XMP_MM, "InstanceID", &"meta4:Clone".into())
            .unwrap();

        assert_eq!(
            meta2.property(xmp_ns::XMP_MM, "InstanceID").unwrap().value,
            "meta2:original"
        );

        assert_eq!(meta2.to_string(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"Test:XMPCoreCoverage/kRDFCoverage\" xmlns:ns1=\"ns:test1/\" xmlns:ns2=\"ns:test2/\" xmlns:xmpMM=\"http://ns.adobe.com/xap/1.0/mm/\"> <ns1:SimpleProp1>Simple1 value</ns1:SimpleProp1> <ns1:SimpleProp2 xml:lang=\"x-default\">Simple2 value</ns1:SimpleProp2> <ns1:ArrayProp1> <rdf:Bag> <rdf:li>Item1.1 value</rdf:li> <rdf:li>Item1.2 value</rdf:li> </rdf:Bag> </ns1:ArrayProp1> <ns1:ArrayProp2> <rdf:Alt> <rdf:li xml:lang=\"x-one\">Item2.1 value</rdf:li> <rdf:li xml:lang=\"x-two\">Item2.2 value</rdf:li> </rdf:Alt> </ns1:ArrayProp2> <ns1:ArrayProp3> <rdf:Alt> <rdf:li xml:lang=\"x-one\">Item3.1 value</rdf:li> <rdf:li>Item3.2 value</rdf:li> </rdf:Alt> </ns1:ArrayProp3> <ns1:ArrayProp4> <rdf:Alt> <rdf:li>Item4.1 value</rdf:li> <rdf:li xml:lang=\"x-two\">Item4.2 value</rdf:li> </rdf:Alt> </ns1:ArrayProp4> <ns1:ArrayProp5> <rdf:Alt> <rdf:li xml:lang=\"x-xxx\">Item5.1 value</rdf:li> <rdf:li xml:lang=\"x-xxx\">Item5.2 value</rdf:li> </rdf:Alt> </ns1:ArrayProp5> <ns1:StructProp rdf:parseType=\"Resource\"> <ns2:Field1>Field1 value</ns2:Field1> <ns2:Field2>Field2 value</ns2:Field2> </ns1:StructProp> <ns1:QualProp1 rdf:parseType=\"Resource\"> <rdf:value>Prop value</rdf:value> <ns2:Qual>Qual value</ns2:Qual> </ns1:QualProp1> <ns1:QualProp2 xml:lang=\"x-default\" rdf:parseType=\"Resource\"> <rdf:value>Prop value</rdf:value> <ns2:Qual>Qual value</ns2:Qual> </ns1:QualProp2> <ns1:QualProp3 xml:lang=\"x-default\" rdf:parseType=\"Resource\"> <rdf:value>Prop value</rdf:value> <ns2:Qual>Qual value</ns2:Qual> </ns1:QualProp3> <ns1:QualProp4 xml:lang=\"x-default\" rdf:parseType=\"Resource\"> <ns2:Field1>Field1 value</ns2:Field1> <ns2:Field2>Field2 value</ns2:Field2> </ns1:QualProp4> <ns1:QualProp5 xml:lang=\"x-default\"> <rdf:Bag> <rdf:li>Item1.1 value</rdf:li> <rdf:li>Item1.2 value</rdf:li> </rdf:Bag> </ns1:QualProp5> <ns2:NestedStructProp rdf:parseType=\"Resource\"> <ns1:Outer rdf:parseType=\"Resource\"> <ns1:Middle rdf:parseType=\"Resource\"> <ns1:Inner rdf:parseType=\"Resource\"> <ns1:Field1>Field1 value</ns1:Field1> <ns2:Field2>Field2 value</ns2:Field2> </ns1:Inner> </ns1:Middle> </ns1:Outer> </ns2:NestedStructProp> <xmpMM:InstanceID>meta2:original</xmpMM:InstanceID> </rdf:Description> </rdf:RDF> </x:xmpmeta>");

        assert_eq!(
            meta4.property(xmp_ns::XMP_MM, "InstanceID").unwrap().value,
            "meta4:Clone"
        );

        assert_eq!(meta4.to_string(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"Test:XMPCoreCoverage/kRDFCoverage\" xmlns:ns1=\"ns:test1/\" xmlns:ns2=\"ns:test2/\" xmlns:xmpMM=\"http://ns.adobe.com/xap/1.0/mm/\"> <ns1:SimpleProp1>Simple1 value</ns1:SimpleProp1> <ns1:SimpleProp2 xml:lang=\"x-default\">Simple2 value</ns1:SimpleProp2> <ns1:ArrayProp1> <rdf:Bag> <rdf:li>Item1.1 value</rdf:li> <rdf:li>Item1.2 value</rdf:li> </rdf:Bag> </ns1:ArrayProp1> <ns1:ArrayProp2> <rdf:Alt> <rdf:li xml:lang=\"x-one\">Item2.1 value</rdf:li> <rdf:li xml:lang=\"x-two\">Item2.2 value</rdf:li> </rdf:Alt> </ns1:ArrayProp2> <ns1:ArrayProp3> <rdf:Alt> <rdf:li xml:lang=\"x-one\">Item3.1 value</rdf:li> <rdf:li>Item3.2 value</rdf:li> </rdf:Alt> </ns1:ArrayProp3> <ns1:ArrayProp4> <rdf:Alt> <rdf:li>Item4.1 value</rdf:li> <rdf:li xml:lang=\"x-two\">Item4.2 value</rdf:li> </rdf:Alt> </ns1:ArrayProp4> <ns1:ArrayProp5> <rdf:Alt> <rdf:li xml:lang=\"x-xxx\">Item5.1 value</rdf:li> <rdf:li xml:lang=\"x-xxx\">Item5.2 value</rdf:li> </rdf:Alt> </ns1:ArrayProp5> <ns1:StructProp rdf:parseType=\"Resource\"> <ns2:Field1>Field1 value</ns2:Field1> <ns2:Field2>Field2 value</ns2:Field2> </ns1:StructProp> <ns1:QualProp1 rdf:parseType=\"Resource\"> <rdf:value>Prop value</rdf:value> <ns2:Qual>Qual value</ns2:Qual> </ns1:QualProp1> <ns1:QualProp2 xml:lang=\"x-default\" rdf:parseType=\"Resource\"> <rdf:value>Prop value</rdf:value> <ns2:Qual>Qual value</ns2:Qual> </ns1:QualProp2> <ns1:QualProp3 xml:lang=\"x-default\" rdf:parseType=\"Resource\"> <rdf:value>Prop value</rdf:value> <ns2:Qual>Qual value</ns2:Qual> </ns1:QualProp3> <ns1:QualProp4 xml:lang=\"x-default\" rdf:parseType=\"Resource\"> <ns2:Field1>Field1 value</ns2:Field1> <ns2:Field2>Field2 value</ns2:Field2> </ns1:QualProp4> <ns1:QualProp5 xml:lang=\"x-default\"> <rdf:Bag> <rdf:li>Item1.1 value</rdf:li> <rdf:li>Item1.2 value</rdf:li> </rdf:Bag> </ns1:QualProp5> <ns2:NestedStructProp rdf:parseType=\"Resource\"> <ns1:Outer rdf:parseType=\"Resource\"> <ns1:Middle rdf:parseType=\"Resource\"> <ns1:Inner rdf:parseType=\"Resource\"> <ns1:Field1>Field1 value</ns1:Field1> <ns2:Field2>Field2 value</ns2:Field2> </ns1:Inner> </ns1:Middle> </ns1:Outer> </ns2:NestedStructProp> <xmpMM:InstanceID>meta4:Clone</xmpMM:InstanceID> </rdf:Description> </rdf:RDF> </x:xmpmeta>");

        println!("Clone and add instance ID = {:#?}", meta4);

        write_major_label("Test XMPMeta object options");
        println!("object options APIs not ported since there are no relevant options");

        println!("\nSKIPPING object options tests -- not ported");
    }

    //-------------------------------------------------------------------------

    write_major_label("Test static namespace functions");

    let ns2_prefix = XmpMeta::register_namespace(NS2, "ns2").unwrap();
    println!("RegisterNamespace ns2 = {}", ns2_prefix);

    let nsx_prefix = XmpMeta::register_namespace(NS2, "nsx:").unwrap();
    println!("RegisterNamespace nsx = {}", nsx_prefix);
    assert_eq!(nsx_prefix, "ns2:");

    let ns1_prefix = XmpMeta::namespace_prefix(NS1);
    println!("namespace_prefix(ns1) = {:#?}", ns1_prefix);
    assert_eq!(ns1_prefix, Some("ns1:".to_owned()));

    let ns1_uri = XmpMeta::namespace_uri("ns1");
    println!("namespace_uri(ns1) = {:#?}", ns1_uri);
    assert_eq!(ns1_uri, Some(NS1.to_owned()));

    let bogus_prefix = XmpMeta::namespace_prefix("bogus");
    println!("namespace_prefix(bogus) = {:#?}", bogus_prefix);
    assert_eq!(bogus_prefix, None);

    let bogus_uri = XmpMeta::namespace_uri("bogus");
    println!("namespace_prefix(bogus) = {:#?}", bogus_uri);
    assert_eq!(bogus_uri, None);

    println!("{}", XmpMeta::debug_dump_namespaces());

    // NOTE: Delete namespace API not ported.

    //-------------------------------------------------------------------------

    {
        write_major_label("Test set_property and related methods");

        let mut meta = XmpMeta::default();

        meta.set_property(NS1, "Prop", &"Prop value".into())
            .unwrap();

        meta.set_property(NS1, "ns1:XMLProp", &"<PropValue/>".into())
            .unwrap();

        assert_eq!(meta.to_string(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"\" xmlns:ns1=\"ns:test1/\"> <ns1:Prop>Prop value</ns1:Prop> <ns1:XMLProp>&lt;PropValue/&gt;</ns1:XMLProp> </rdf:Description> </rdf:RDF> </x:xmpmeta>");

        meta.set_property(
            NS1,
            "ns1:URIProp",
            &XmpValue::new("URI:value/".to_owned()).set_is_uri(true),
        )
        .unwrap();

        meta.append_array_item(
            NS1,
            &XmpValue::from("Bag").set_is_array(true),
            &"BagItem value".into(),
        )
        .unwrap();

        assert_eq!(
            meta.property(NS1, "Bag").unwrap(),
            XmpValue {
                value: "".to_owned(),
                options: xmp_prop::VALUE_IS_ARRAY
            }
        );

        assert_eq!(meta.to_string(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"\" xmlns:ns1=\"ns:test1/\"> <ns1:Prop>Prop value</ns1:Prop> <ns1:XMLProp>&lt;PropValue/&gt;</ns1:XMLProp> <ns1:URIProp rdf:resource=\"URI:value/\"/> <ns1:Bag> <rdf:Bag> <rdf:li>BagItem value</rdf:li> </rdf:Bag> </ns1:Bag> </rdf:Description> </rdf:RDF> </x:xmpmeta>");

        let bag: Vec<XmpValue<String>> = meta.property_array(NS1, "Bag").collect();
        assert_eq!(
            bag,
            [XmpValue {
                value: "BagItem value".to_owned(),
                options: 0
            }]
        );

        meta.append_array_item(
            NS1,
            &XmpValue::from("ns1:Seq").set_is_ordered(true),
            &"SeqItem value".into(),
        )
        .unwrap();

        let seq: Vec<XmpValue<String>> = meta.property_array(NS1, "ns1:Seq").collect();
        assert_eq!(
            seq,
            [XmpValue {
                value: "SeqItem value".to_owned(),
                options: 0
            }]
        );

        meta.append_array_item(
            NS1,
            &XmpValue::from("ns1:Alt").set_is_alternate(true),
            &"AltItem value".into(),
        )
        .unwrap();

        let alt: Vec<XmpValue<String>> = meta.property_array(NS1, "ns1:Alt").collect();
        assert_eq!(
            alt,
            [XmpValue {
                value: "AltItem value".to_owned(),
                options: 0
            }]
        );

        meta.set_struct_field(NS1, "Struct", NS2, "Field1", &"Field1 value".into())
            .unwrap();

        meta.set_struct_field(NS1, "ns1:Struct", NS2, "Field2", &"Field2 value".into())
            .unwrap();

        meta.set_struct_field(NS1, "ns1:Struct", NS2, "Field3", &"Field3 value".into())
            .unwrap();

        assert_eq!(meta.to_string(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"\" xmlns:ns1=\"ns:test1/\" xmlns:ns2=\"ns:test2/\"> <ns1:Prop>Prop value</ns1:Prop> <ns1:XMLProp>&lt;PropValue/&gt;</ns1:XMLProp> <ns1:URIProp rdf:resource=\"URI:value/\"/> <ns1:Bag> <rdf:Bag> <rdf:li>BagItem value</rdf:li> </rdf:Bag> </ns1:Bag> <ns1:Seq> <rdf:Seq> <rdf:li>SeqItem value</rdf:li> </rdf:Seq> </ns1:Seq> <ns1:Alt> <rdf:Alt> <rdf:li>AltItem value</rdf:li> </rdf:Alt> </ns1:Alt> <ns1:Struct rdf:parseType=\"Resource\"> <ns2:Field1>Field1 value</ns2:Field1> <ns2:Field2>Field2 value</ns2:Field2> <ns2:Field3>Field3 value</ns2:Field3> </ns1:Struct> </rdf:Description> </rdf:RDF> </x:xmpmeta>");

        meta.set_array_item(
            NS1,
            "Bag",
            ItemPlacement::ReplaceItemAtIndex(1),
            &"BagItem 3".into(),
        )
        .unwrap();

        meta.set_array_item(
            NS1,
            "ns1:Bag",
            ItemPlacement::InsertBeforeIndex(1),
            &"BagItem 1".into(),
        )
        .unwrap();

        meta.set_array_item(
            NS1,
            "ns1:Bag",
            ItemPlacement::InsertAfterIndex(1),
            &"BagItem 2".into(),
        )
        .unwrap();

        meta.append_array_item(NS1, &"Bag".into(), &"BagItem 4".into())
            .unwrap();

        let bag_contents: Vec<XmpValue<String>> = meta.property_array(NS1, "Bag").collect();
        println!("bag_contents = {:#?}", bag_contents);

        assert_eq!(
            bag_contents,
            [
                XmpValue {
                    value: "BagItem 1".to_owned(),
                    options: 0
                },
                XmpValue {
                    value: "BagItem 2".to_owned(),
                    options: 0
                },
                XmpValue {
                    value: "BagItem 3".to_owned(),
                    options: 0
                },
                XmpValue {
                    value: "BagItem 4".to_owned(),
                    options: 0
                }
            ]
        );

        assert_eq!(meta.to_string(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"\" xmlns:ns1=\"ns:test1/\" xmlns:ns2=\"ns:test2/\"> <ns1:Prop>Prop value</ns1:Prop> <ns1:XMLProp>&lt;PropValue/&gt;</ns1:XMLProp> <ns1:URIProp rdf:resource=\"URI:value/\"/> <ns1:Bag> <rdf:Bag> <rdf:li>BagItem 1</rdf:li> <rdf:li>BagItem 2</rdf:li> <rdf:li>BagItem 3</rdf:li> <rdf:li>BagItem 4</rdf:li> </rdf:Bag> </ns1:Bag> <ns1:Seq> <rdf:Seq> <rdf:li>SeqItem value</rdf:li> </rdf:Seq> </ns1:Seq> <ns1:Alt> <rdf:Alt> <rdf:li>AltItem value</rdf:li> </rdf:Alt> </ns1:Alt> <ns1:Struct rdf:parseType=\"Resource\"> <ns2:Field1>Field1 value</ns2:Field1> <ns2:Field2>Field2 value</ns2:Field2> <ns2:Field3>Field3 value</ns2:Field3> </ns1:Struct> </rdf:Description> </rdf:RDF> </x:xmpmeta>");

        println!("A few basic Set... calls = {:#?}", meta);

        assert_eq!(meta.to_string_with_options(ToStringOptions::default().omit_packet_wrapper()).unwrap(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\">\n   <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\">\n      <rdf:Description rdf:about=\"\"\n            xmlns:ns1=\"ns:test1/\"\n            xmlns:ns2=\"ns:test2/\">\n         <ns1:Prop>Prop value</ns1:Prop>\n         <ns1:XMLProp>&lt;PropValue/&gt;</ns1:XMLProp>\n         <ns1:URIProp rdf:resource=\"URI:value/\"/>\n         <ns1:Bag>\n            <rdf:Bag>\n               <rdf:li>BagItem 1</rdf:li>\n               <rdf:li>BagItem 2</rdf:li>\n               <rdf:li>BagItem 3</rdf:li>\n               <rdf:li>BagItem 4</rdf:li>\n            </rdf:Bag>\n         </ns1:Bag>\n         <ns1:Seq>\n            <rdf:Seq>\n               <rdf:li>SeqItem value</rdf:li>\n            </rdf:Seq>\n         </ns1:Seq>\n         <ns1:Alt>\n            <rdf:Alt>\n               <rdf:li>AltItem value</rdf:li>\n            </rdf:Alt>\n         </ns1:Alt>\n         <ns1:Struct rdf:parseType=\"Resource\">\n            <ns2:Field1>Field1 value</ns2:Field1>\n            <ns2:Field2>Field2 value</ns2:Field2>\n            <ns2:Field3>Field3 value</ns2:Field3>\n         </ns1:Struct>\n      </rdf:Description>\n   </rdf:RDF>\n</x:xmpmeta>\n");

        assert_eq!(meta.array_len(NS1, "Bag"), 4);

        meta.set_property(NS1, "QualProp1", &"Prop value".into())
            .unwrap();

        meta.set_qualifier(NS1, "QualProp1", NS2, "Qual1", &"Qual1 value".into())
            .unwrap();

        // meta.setProperty ( NS1, "QualProp1/Qual2", "Qual2 value",
        // kXMP_PropIsQualifier ); ^^ is invalid in C++; can't construct this in
        // Rust. (There is no setter for kXMP_PropIsQualifier.)

        meta.set_property(NS1, "QualProp1/?ns2:Qual3", &"Qual3 value".into())
            .unwrap();
        meta.set_property(NS1, "QualProp1/?xml:lang", &"x-qual".into())
            .unwrap();

        meta.set_property(NS1, "QualProp2", &"Prop value".into())
            .unwrap();
        meta.set_qualifier(NS1, "QualProp2", xmp_ns::XML, "lang", &"en-us".into())
            .unwrap();

        // meta.setProperty ( NS1, "QualProp2/xml:lang", "x-field", kXMP_PropIsQualifier
        // ); ^^ is invalid in C++; can't construct this in Rust. (There is no
        // setter for kXMP_PropIsQualifier.)

        meta.set_property(NS1, "QualProp2/@xml:lang", &"x-attr".into())
            .unwrap();
        meta.set_property(NS1, "QualProp3", &"Prop value".into())
            .unwrap();

        meta.set_qualifier(
            NS1,
            "ns1:QualProp3",
            xmp_ns::XML,
            "xml:lang",
            &"en-us".into(),
        )
        .unwrap();

        meta.set_qualifier(NS1, "ns1:QualProp3", NS2, "ns2:Qual", &"Qual value".into())
            .unwrap();

        meta.set_property(NS1, "QualProp4", &"Prop value".into())
            .unwrap();
        meta.set_qualifier(NS1, "QualProp4", NS2, "Qual", &"Qual value".into())
            .unwrap();
        meta.set_qualifier(NS1, "QualProp4", xmp_ns::XML, "lang", &"en-us".into())
            .unwrap();

        println!("Add some qualifiers = {:#?}", meta);

        assert_eq!(meta.to_string(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"\" xmlns:ns1=\"ns:test1/\" xmlns:ns2=\"ns:test2/\"> <ns1:Prop>Prop value</ns1:Prop> <ns1:XMLProp>&lt;PropValue/&gt;</ns1:XMLProp> <ns1:URIProp rdf:resource=\"URI:value/\"/> <ns1:Bag> <rdf:Bag> <rdf:li>BagItem 1</rdf:li> <rdf:li>BagItem 2</rdf:li> <rdf:li>BagItem 3</rdf:li> <rdf:li>BagItem 4</rdf:li> </rdf:Bag> </ns1:Bag> <ns1:Seq> <rdf:Seq> <rdf:li>SeqItem value</rdf:li> </rdf:Seq> </ns1:Seq> <ns1:Alt> <rdf:Alt> <rdf:li>AltItem value</rdf:li> </rdf:Alt> </ns1:Alt> <ns1:Struct rdf:parseType=\"Resource\"> <ns2:Field1>Field1 value</ns2:Field1> <ns2:Field2>Field2 value</ns2:Field2> <ns2:Field3>Field3 value</ns2:Field3> </ns1:Struct> <ns1:QualProp1 xml:lang=\"x-qual\" rdf:parseType=\"Resource\"> <rdf:value>Prop value</rdf:value> <ns2:Qual1>Qual1 value</ns2:Qual1> <ns2:Qual3>Qual3 value</ns2:Qual3> </ns1:QualProp1> <ns1:QualProp2 xml:lang=\"x-attr\">Prop value</ns1:QualProp2> <ns1:QualProp3 xml:lang=\"en-US\" rdf:parseType=\"Resource\"> <rdf:value>Prop value</rdf:value> <ns2:Qual>Qual value</ns2:Qual> </ns1:QualProp3> <ns1:QualProp4 xml:lang=\"en-US\" rdf:parseType=\"Resource\"> <rdf:value>Prop value</rdf:value> <ns2:Qual>Qual value</ns2:Qual> </ns1:QualProp4> </rdf:Description> </rdf:RDF> </x:xmpmeta>");

        assert_eq!(
            meta.to_string_with_options(ToStringOptions::default().omit_packet_wrapper())
                .unwrap(),
            "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\">\n   <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\">\n      <rdf:Description rdf:about=\"\"\n            xmlns:ns1=\"ns:test1/\"\n            xmlns:ns2=\"ns:test2/\">\n         <ns1:Prop>Prop value</ns1:Prop>\n         <ns1:XMLProp>&lt;PropValue/&gt;</ns1:XMLProp>\n         <ns1:URIProp rdf:resource=\"URI:value/\"/>\n         <ns1:Bag>\n            <rdf:Bag>\n               <rdf:li>BagItem 1</rdf:li>\n               <rdf:li>BagItem 2</rdf:li>\n               <rdf:li>BagItem 3</rdf:li>\n               <rdf:li>BagItem 4</rdf:li>\n            </rdf:Bag>\n         </ns1:Bag>\n         <ns1:Seq>\n            <rdf:Seq>\n               <rdf:li>SeqItem value</rdf:li>\n            </rdf:Seq>\n         </ns1:Seq>\n         <ns1:Alt>\n            <rdf:Alt>\n               <rdf:li>AltItem value</rdf:li>\n            </rdf:Alt>\n         </ns1:Alt>\n         <ns1:Struct rdf:parseType=\"Resource\">\n            <ns2:Field1>Field1 value</ns2:Field1>\n            <ns2:Field2>Field2 value</ns2:Field2>\n            <ns2:Field3>Field3 value</ns2:Field3>\n         </ns1:Struct>\n         <ns1:QualProp1 xml:lang=\"x-qual\" rdf:parseType=\"Resource\">\n            <rdf:value>Prop value</rdf:value>\n            <ns2:Qual1>Qual1 value</ns2:Qual1>\n            <ns2:Qual3>Qual3 value</ns2:Qual3>\n         </ns1:QualProp1>\n         <ns1:QualProp2 xml:lang=\"x-attr\">Prop value</ns1:QualProp2>\n         <ns1:QualProp3 xml:lang=\"en-US\" rdf:parseType=\"Resource\">\n            <rdf:value>Prop value</rdf:value>\n            <ns2:Qual>Qual value</ns2:Qual>\n         </ns1:QualProp3>\n         <ns1:QualProp4 xml:lang=\"en-US\" rdf:parseType=\"Resource\">\n            <rdf:value>Prop value</rdf:value>\n            <ns2:Qual>Qual value</ns2:Qual>\n         </ns1:QualProp4>\n      </rdf:Description>\n   </rdf:RDF>\n</x:xmpmeta>\n"
        );

        meta.set_property(NS1, "QualProp1", &"new value".into())
            .unwrap();
        meta.set_property(NS1, "QualProp2", &"new value".into())
            .unwrap();
        meta.set_property(NS1, "QualProp3", &"new value".into())
            .unwrap();
        meta.set_property(NS1, "QualProp4", &"new value".into())
            .unwrap();

        println!("Change values and keep qualifiers = {:#?}", meta);

        assert_eq!(meta.to_string(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"\" xmlns:ns1=\"ns:test1/\" xmlns:ns2=\"ns:test2/\"> <ns1:Prop>Prop value</ns1:Prop> <ns1:XMLProp>&lt;PropValue/&gt;</ns1:XMLProp> <ns1:URIProp rdf:resource=\"URI:value/\"/> <ns1:Bag> <rdf:Bag> <rdf:li>BagItem 1</rdf:li> <rdf:li>BagItem 2</rdf:li> <rdf:li>BagItem 3</rdf:li> <rdf:li>BagItem 4</rdf:li> </rdf:Bag> </ns1:Bag> <ns1:Seq> <rdf:Seq> <rdf:li>SeqItem value</rdf:li> </rdf:Seq> </ns1:Seq> <ns1:Alt> <rdf:Alt> <rdf:li>AltItem value</rdf:li> </rdf:Alt> </ns1:Alt> <ns1:Struct rdf:parseType=\"Resource\"> <ns2:Field1>Field1 value</ns2:Field1> <ns2:Field2>Field2 value</ns2:Field2> <ns2:Field3>Field3 value</ns2:Field3> </ns1:Struct> <ns1:QualProp1 xml:lang=\"x-qual\" rdf:parseType=\"Resource\"> <rdf:value>new value</rdf:value> <ns2:Qual1>Qual1 value</ns2:Qual1> <ns2:Qual3>Qual3 value</ns2:Qual3> </ns1:QualProp1> <ns1:QualProp2 xml:lang=\"x-attr\">new value</ns1:QualProp2> <ns1:QualProp3 xml:lang=\"en-US\" rdf:parseType=\"Resource\"> <rdf:value>new value</rdf:value> <ns2:Qual>Qual value</ns2:Qual> </ns1:QualProp3> <ns1:QualProp4 xml:lang=\"en-US\" rdf:parseType=\"Resource\"> <rdf:value>new value</rdf:value> <ns2:Qual>Qual value</ns2:Qual> </ns1:QualProp4> </rdf:Description> </rdf:RDF> </x:xmpmeta>");

        //-------------------------------------------------------------------------

        write_major_label("Test property and related methods");

        // Start with fresh qualifiers.
        meta.delete_property(NS1, "QualProp1").unwrap();
        meta.delete_property(NS1, "ns1:QualProp2").unwrap();
        meta.delete_property(NS1, "ns1:QualProp3").unwrap();
        meta.delete_property(NS1, "QualProp4").unwrap();

        meta.set_property(NS1, "QualProp1", &"Prop value".into())
            .unwrap();
        meta.set_qualifier(NS1, "QualProp1", NS2, "Qual1", &"Qual1 value".into())
            .unwrap();

        meta.set_property(NS1, "QualProp2", &"Prop value".into())
            .unwrap();
        meta.set_qualifier(NS1, "QualProp2", xmp_ns::XML, "lang", &"en-us".into())
            .unwrap();

        meta.set_property(NS1, "QualProp3", &"Prop value".into())
            .unwrap();
        meta.set_qualifier(NS1, "QualProp3", xmp_ns::XML, "lang", &"en-us".into())
            .unwrap();

        meta.set_qualifier(NS1, "QualProp3", NS2, "Qual", &"Qual value".into())
            .unwrap();

        meta.set_property(NS1, "QualProp4", &"Prop value".into())
            .unwrap();

        meta.set_qualifier(NS1, "QualProp4", NS2, "Qual", &"Qual value".into())
            .unwrap();

        meta.set_qualifier(NS1, "QualProp4", xmp_ns::XML, "lang", &"en-us".into())
            .unwrap();

        println!("XMP object = {:#?}", meta);

        assert_eq!(meta.to_string(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"\" xmlns:ns1=\"ns:test1/\" xmlns:ns2=\"ns:test2/\"> <ns1:Prop>Prop value</ns1:Prop> <ns1:XMLProp>&lt;PropValue/&gt;</ns1:XMLProp> <ns1:URIProp rdf:resource=\"URI:value/\"/> <ns1:Bag> <rdf:Bag> <rdf:li>BagItem 1</rdf:li> <rdf:li>BagItem 2</rdf:li> <rdf:li>BagItem 3</rdf:li> <rdf:li>BagItem 4</rdf:li> </rdf:Bag> </ns1:Bag> <ns1:Seq> <rdf:Seq> <rdf:li>SeqItem value</rdf:li> </rdf:Seq> </ns1:Seq> <ns1:Alt> <rdf:Alt> <rdf:li>AltItem value</rdf:li> </rdf:Alt> </ns1:Alt> <ns1:Struct rdf:parseType=\"Resource\"> <ns2:Field1>Field1 value</ns2:Field1> <ns2:Field2>Field2 value</ns2:Field2> <ns2:Field3>Field3 value</ns2:Field3> </ns1:Struct> <ns1:QualProp1 rdf:parseType=\"Resource\"> <rdf:value>Prop value</rdf:value> <ns2:Qual1>Qual1 value</ns2:Qual1> </ns1:QualProp1> <ns1:QualProp2 xml:lang=\"en-US\">Prop value</ns1:QualProp2> <ns1:QualProp3 xml:lang=\"en-US\" rdf:parseType=\"Resource\"> <rdf:value>Prop value</rdf:value> <ns2:Qual>Qual value</ns2:Qual> </ns1:QualProp3> <ns1:QualProp4 xml:lang=\"en-US\" rdf:parseType=\"Resource\"> <rdf:value>Prop value</rdf:value> <ns2:Qual>Qual value</ns2:Qual> </ns1:QualProp4> </rdf:Description> </rdf:RDF> </x:xmpmeta>");

        assert_eq!(
            meta.property(NS1, "Prop"),
            Some(XmpValue {
                value: "Prop value".to_owned(),
                options: 0
            })
        );

        assert_eq!(meta.property("", "ns1:Prop"), None);

        assert_eq!(
            meta.property(NS1, "ns1:XMLProp"),
            Some(XmpValue {
                value: "<PropValue/>".to_owned(),
                options: 0
            })
        );

        assert_eq!(
            meta.property(NS1, "ns1:URIProp"),
            Some(XmpValue {
                value: "URI:value/".to_owned(),
                options: xmp_prop::VALUE_IS_URI
            })
        );

        assert_eq!(
            meta.array_item(NS1, "Bag", 2),
            Some(XmpValue {
                value: "BagItem 2".to_owned(),
                options: 0
            })
        );

        assert_eq!(meta.array_item("", "ns1:Bag", 1), None);

        assert_eq!(
            meta.array_item(NS1, "ns1:Seq", 1),
            Some(XmpValue {
                value: "SeqItem value".to_owned(),
                options: 0
            })
        );

        assert_eq!(
            meta.array_item(NS1, "ns1:Alt", XmpMeta::LAST_ITEM),
            Some(XmpValue {
                value: "AltItem value".to_owned(),
                options: 0
            })
        );

        assert_eq!(
            meta.struct_field(NS1, "Struct", NS2, "Field1"),
            Some(XmpValue {
                value: "Field1 value".to_owned(),
                options: 0
            })
        );

        assert_eq!(
            meta.struct_field(NS1, "ns1:Struct", NS2, "ns2:Field2"),
            Some(XmpValue {
                value: "Field2 value".to_owned(),
                options: 0
            })
        );

        assert_eq!(
            meta.struct_field(NS1, "ns1:Struct", NS2, "ns2:Field3"),
            Some(XmpValue {
                value: "Field3 value".to_owned(),
                options: 0
            })
        );

        assert_eq!(
            meta.qualifier(NS1, "QualProp1", NS2, "Qual1"),
            Some(XmpValue {
                value: "Qual1 value".to_owned(),
                options: xmp_prop::IS_QUALIFIER
            })
        );

        assert_eq!(meta.qualifier("", "ns1:QualProp1", NS2, "Qual1"), None);

        assert_eq!(
            meta.qualifier(NS1, "ns1:QualProp3", xmp_ns::XML, "xml:lang"),
            Some(XmpValue {
                value: "en-US".to_owned(),
                options: xmp_prop::IS_QUALIFIER
            })
        );

        assert_eq!(
            meta.qualifier(NS1, "ns1:QualProp3", NS2, "ns2:Qual"),
            Some(XmpValue {
                value: "Qual value".to_owned(),
                options: xmp_prop::IS_QUALIFIER
            })
        );

        assert_eq!(
            meta.property(NS1, "Bag"),
            Some(XmpValue {
                value: "".to_owned(),
                options: xmp_prop::VALUE_IS_ARRAY
            })
        );

        assert_eq!(
            meta.property(NS1, "Seq"),
            Some(XmpValue {
                value: "".to_owned(),
                options: xmp_prop::VALUE_IS_ARRAY | xmp_prop::ARRAY_IS_ORDERED
            })
        );

        assert_eq!(
            meta.property(NS1, "Alt"),
            Some(XmpValue {
                value: "".to_owned(),
                options: xmp_prop::VALUE_IS_ARRAY
                    | xmp_prop::ARRAY_IS_ORDERED
                    | xmp_prop::ARRAY_IS_ALTERNATE
            })
        );

        assert_eq!(
            meta.property(NS1, "Struct"),
            Some(XmpValue {
                value: "".to_owned(),
                options: xmp_prop::VALUE_IS_STRUCT
            })
        );

        assert_eq!(meta.property("ns:bogus/", "Bogus"), None);
        assert_eq!(meta.property(NS1, "Bogus"), None);
        assert_eq!(meta.array_item(NS1, "Bag", 99), None);
        assert_eq!(meta.struct_field(NS1, "Struct", NS2, "Bogus"), None);
        assert_eq!(meta.qualifier(NS1, "Prop", NS2, "Bogus"), None);

        //-------------------------------------------------------------------------

        write_major_label("Test contains_property, delete_property, and related methods");

        println!("XMP object = {:#?}", meta);

        assert!(meta.contains_property(NS1, "Prop"));
        assert!(!meta.contains_property("", "ns1:Bag"));
        assert!(meta.contains_property(NS1, "ns1:Struct"));

        // Not ported to Rust (use `array_len` instead):
        //    ok = meta.DoesArrayItemExist(NS1, "Bag", 2);
        // 	  ok = meta.DoesArrayItemExist (NS1, "ns1:Seq", kXMP_ArrayLastItem);

        assert!(meta.contains_struct_field(NS1, "Struct", NS2, "Field1"));

        assert!(meta.contains_qualifier(NS1, "QualProp1", NS2, "Qual1"));
        assert!(meta.contains_qualifier(NS1, "QualProp2", xmp_ns::XML, "lang"));
        assert!(!meta.contains_property("ns:bogus/", "Bogus"));
        assert!(!meta.contains_property(NS1, "Bogus"));

        // Not ported to Rust (use `array_len` instead):
        // 	  ok = meta.DoesArrayItemExist ( NS1, "Bag", 99 );
        // 		ok = meta.DoesArrayItemExist ( 0, "ns1:Bag", kXMP_ArrayLastItem );

        assert!(!meta.contains_struct_field(NS1, "Struct", NS2, "Bogus"));
        assert!(!meta.contains_qualifier(NS1, "Prop", NS2, "Bogus"));

        meta.delete_property(NS1, "Prop").unwrap();
        meta.delete_array_item(NS1, "Bag", 2).unwrap();
        meta.delete_struct_field(NS1, "Struct", NS2, "Field1")
            .unwrap();

        println!("Delete Prop, Bag[2], and Struct1/Field1 = {:#?}", meta);

        assert_eq!(meta.to_string(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"\" xmlns:ns1=\"ns:test1/\" xmlns:ns2=\"ns:test2/\"> <ns1:XMLProp>&lt;PropValue/&gt;</ns1:XMLProp> <ns1:URIProp rdf:resource=\"URI:value/\"/> <ns1:Bag> <rdf:Bag> <rdf:li>BagItem 1</rdf:li> <rdf:li>BagItem 3</rdf:li> <rdf:li>BagItem 4</rdf:li> </rdf:Bag> </ns1:Bag> <ns1:Seq> <rdf:Seq> <rdf:li>SeqItem value</rdf:li> </rdf:Seq> </ns1:Seq> <ns1:Alt> <rdf:Alt> <rdf:li>AltItem value</rdf:li> </rdf:Alt> </ns1:Alt> <ns1:Struct rdf:parseType=\"Resource\"> <ns2:Field2>Field2 value</ns2:Field2> <ns2:Field3>Field3 value</ns2:Field3> </ns1:Struct> <ns1:QualProp1 rdf:parseType=\"Resource\"> <rdf:value>Prop value</rdf:value> <ns2:Qual1>Qual1 value</ns2:Qual1> </ns1:QualProp1> <ns1:QualProp2 xml:lang=\"en-US\">Prop value</ns1:QualProp2> <ns1:QualProp3 xml:lang=\"en-US\" rdf:parseType=\"Resource\"> <rdf:value>Prop value</rdf:value> <ns2:Qual>Qual value</ns2:Qual> </ns1:QualProp3> <ns1:QualProp4 xml:lang=\"en-US\" rdf:parseType=\"Resource\"> <rdf:value>Prop value</rdf:value> <ns2:Qual>Qual value</ns2:Qual> </ns1:QualProp4> </rdf:Description> </rdf:RDF> </x:xmpmeta>");

        meta.delete_qualifier(NS1, "QualProp1", NS2, "Qual1")
            .unwrap();
        meta.delete_qualifier(NS1, "QualProp2", xmp_ns::XML, "lang")
            .unwrap();
        meta.delete_qualifier(NS1, "QualProp3", NS2, "Qual")
            .unwrap();
        meta.delete_qualifier(NS1, "QualProp4", xmp_ns::XML, "lang")
            .unwrap();

        println!("Delete QualProp1/?ns2:Qual1, QualProp2/?xml:lang, QualProp3:/ns2:Qual, and QualProp4/?xml:lang = {:#?}", meta);

        assert_eq!(meta.to_string(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"\" xmlns:ns1=\"ns:test1/\" xmlns:ns2=\"ns:test2/\"> <ns1:XMLProp>&lt;PropValue/&gt;</ns1:XMLProp> <ns1:URIProp rdf:resource=\"URI:value/\"/> <ns1:Bag> <rdf:Bag> <rdf:li>BagItem 1</rdf:li> <rdf:li>BagItem 3</rdf:li> <rdf:li>BagItem 4</rdf:li> </rdf:Bag> </ns1:Bag> <ns1:Seq> <rdf:Seq> <rdf:li>SeqItem value</rdf:li> </rdf:Seq> </ns1:Seq> <ns1:Alt> <rdf:Alt> <rdf:li>AltItem value</rdf:li> </rdf:Alt> </ns1:Alt> <ns1:Struct rdf:parseType=\"Resource\"> <ns2:Field2>Field2 value</ns2:Field2> <ns2:Field3>Field3 value</ns2:Field3> </ns1:Struct> <ns1:QualProp1>Prop value</ns1:QualProp1> <ns1:QualProp2>Prop value</ns1:QualProp2> <ns1:QualProp3 xml:lang=\"en-US\">Prop value</ns1:QualProp3> <ns1:QualProp4 rdf:parseType=\"Resource\"> <rdf:value>Prop value</rdf:value> <ns2:Qual>Qual value</ns2:Qual> </ns1:QualProp4> </rdf:Description> </rdf:RDF> </x:xmpmeta>");

        meta.delete_property(NS1, "Bag").unwrap();
        meta.delete_property(NS1, "Struct").unwrap();

        println!("Delete all of Bag and Struct = {:#?}", meta);

        assert_eq!(meta.to_string(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"\" xmlns:ns1=\"ns:test1/\" xmlns:ns2=\"ns:test2/\"> <ns1:XMLProp>&lt;PropValue/&gt;</ns1:XMLProp> <ns1:URIProp rdf:resource=\"URI:value/\"/> <ns1:Seq> <rdf:Seq> <rdf:li>SeqItem value</rdf:li> </rdf:Seq> </ns1:Seq> <ns1:Alt> <rdf:Alt> <rdf:li>AltItem value</rdf:li> </rdf:Alt> </ns1:Alt> <ns1:QualProp1>Prop value</ns1:QualProp1> <ns1:QualProp2>Prop value</ns1:QualProp2> <ns1:QualProp3 xml:lang=\"en-US\">Prop value</ns1:QualProp3> <ns1:QualProp4 rdf:parseType=\"Resource\"> <rdf:value>Prop value</rdf:value> <ns2:Qual>Qual value</ns2:Qual> </ns1:QualProp4> </rdf:Description> </rdf:RDF> </x:xmpmeta>");
    }

    //-------------------------------------------------------------------------

    {
        write_major_label("Test set_localized_text and localized_text");

        let mut meta = XmpMeta::default();

        meta.set_localized_text(NS1, "AltText", None, "x-default", "default value")
            .unwrap();

        meta.set_localized_text(NS1, "AltText", Some("en"), "en-us", "en-us value")
            .unwrap();

        meta.set_localized_text(NS1, "AltText", Some("en"), "en-uk", "en-uk value")
            .unwrap();

        assert_eq!(
            meta.localized_text(NS1, "AltText", Some("en"), "en-ca"),
            Some((
                XmpValue {
                    value: "en-us value".to_owned(),
                    options: xmp_prop::HAS_LANG | xmp_prop::HAS_QUALIFIERS,
                },
                "x-default".to_owned()
            ))
        );

        assert_eq!(
            meta.property(NS1, "AltText"),
            Some(XmpValue {
                value: "".to_owned(),
                options: xmp_prop::ARRAY_IS_ALT_TEXT
                    | xmp_prop::ARRAY_IS_ALTERNATE
                    | xmp_prop::ARRAY_IS_ORDERED
                    | xmp_prop::VALUE_IS_ARRAY
            })
        );

        println!("After set_localized_text = {:#?}", meta);

        assert_eq!(meta.to_string(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"\" xmlns:ns1=\"ns:test1/\"> <ns1:AltText> <rdf:Alt> <rdf:li xml:lang=\"x-default\">en-us value</rdf:li> <rdf:li xml:lang=\"en-US\">en-us value</rdf:li> <rdf:li xml:lang=\"en-UK\">en-uk value</rdf:li> </rdf:Alt> </ns1:AltText> </rdf:Description> </rdf:RDF> </x:xmpmeta>");
    }

    //-------------------------------------------------------------------------

    {
        write_major_label(
            "Test set_property... and property... methods (set/get with binary values)",
        );

        let mut meta = XmpMeta::from_str(DATE_TIME_RDF).unwrap();

        let date_value = XmpDateTime {
            date: Some(XmpDate {
                year: 2000,
                month: 1,
                day: 2,
            }),
            time: Some(XmpTime {
                hour: 3,
                minute: 4,
                second: 5,
                nanosecond: 0,
                time_zone: None,
            }),
        };

        meta.set_property_bool(NS1, "Bool0", &false.into()).unwrap();
        meta.set_property_bool(NS1, "Bool1", &true.into()).unwrap();
        meta.set_property_i32(NS1, "Int", &42.into()).unwrap();
        meta.set_property_f64(NS1, "Float", &4.2.into()).unwrap();

        meta.set_property_date(NS1, "Date10", &date_value.into())
            .unwrap();

        let date_value = XmpDateTime {
            date: Some(XmpDate {
                year: 2000,
                month: 1,
                day: 2,
            }),
            time: Some(XmpTime {
                hour: 3,
                minute: 4,
                second: 5,
                nanosecond: 0,
                time_zone: Some(XmpTimeZone { hour: 6, minute: 7 }),
            }),
        };

        meta.set_property_date(NS1, "Date11", &date_value.into())
            .unwrap();

        let date_value = XmpDateTime {
            date: Some(XmpDate {
                year: 2000,
                month: 1,
                day: 2,
            }),
            time: Some(XmpTime {
                hour: 3,
                minute: 4,
                second: 5,
                nanosecond: 0,
                time_zone: Some(XmpTimeZone {
                    hour: -6,
                    minute: 7,
                }),
            }),
        };

        meta.set_property_date(NS1, "Date12", &date_value.into())
            .unwrap();

        let date_value = XmpDateTime {
            date: Some(XmpDate {
                year: 2000,
                month: 1,
                day: 2,
            }),
            time: Some(XmpTime {
                hour: 3,
                minute: 4,
                second: 5,
                nanosecond: 9,
                time_zone: Some(XmpTimeZone {
                    hour: -6,
                    minute: 7,
                }),
            }),
        };

        meta.set_property_date(NS1, "Date13", &date_value.into())
            .unwrap();

        println!("A few basic binary set... calls = {:#?}", meta);

        assert_eq!(meta.to_string(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"Test:XMPCoreCoverage/kDateTimeRDF\" xmlns:ns1=\"ns:test1/\"> <ns1:Date1>2003</ns1:Date1> <ns1:Date2>2003-12</ns1:Date2> <ns1:Date3>2003-12-31</ns1:Date3> <ns1:Date4>2003-12-31T12:34Z</ns1:Date4> <ns1:Date5>2003-12-31T12:34:56Z</ns1:Date5> <ns1:Date6>2003-12-31T12:34:56.001Z</ns1:Date6> <ns1:Date7>2003-12-31T12:34:56.000000001Z</ns1:Date7> <ns1:Date8>2003-12-31T10:04:56-02:30</ns1:Date8> <ns1:Date9>2003-12-31T15:49:56+03:15</ns1:Date9> <ns1:Bool0>False</ns1:Bool0> <ns1:Bool1>True</ns1:Bool1> <ns1:Int>42</ns1:Int> <ns1:Float>4.200000</ns1:Float> <ns1:Date10>2000-01-02T03:04:05</ns1:Date10> <ns1:Date11>2000-01-02T03:04:05+06:07</ns1:Date11> <ns1:Date12>2000-01-02T03:04:05-06:07</ns1:Date12> <ns1:Date13>2000-01-02T03:04:05.000000009-06:07</ns1:Date13> </rdf:Description> </rdf:RDF> </x:xmpmeta>");

        assert_eq!(
            meta.property_bool(NS1, "Bool0"),
            Some(XmpValue {
                value: false,
                options: 0
            })
        );

        assert_eq!(
            meta.property_bool(NS1, "Bool1"),
            Some(XmpValue {
                value: true,
                options: 0
            })
        );

        assert_eq!(
            meta.property_i32(NS1, "Int"),
            Some(XmpValue {
                value: 42i32,
                options: 0
            })
        );

        assert_eq!(
            meta.property_f64(NS1, "Float"),
            Some(XmpValue {
                value: 4.2f64,
                options: 0
            })
        );
    }

    //-------------------------------------------------------------------------

    write_major_label("Test parsing with multiple buffers and various options");

    {
        // TODO (https://github.com/adobe/xmp-toolkit-rs/issues/135):
        // I think this should be an error response, not a silent
        // Ok(default) response.
        let meta = XmpMeta::from_str_requiring_xmp_meta(SIMPLE_RDF, true).unwrap();
        
        println!(
            "Parse and require xmpmeta element, which is missing = {:#?}",
            meta
        );

        assert_eq!(meta.to_string(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"\"/> </rdf:RDF> </x:xmpmeta>");
    }

    {
        let meta = XmpMeta::from_str(NAMESPACE_RDF).unwrap();

        println!("Parse RDF with multiple nested namespaces = {:#?}", meta);
        
        assert_eq!(meta.to_string(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"Test:XMPCoreCoverage/kNamespaceRDF\" xmlns:ns1=\"ns:test1/\" xmlns:ns2=\"ns:test2/\" xmlns:ns3=\"ns:test3/\" xmlns:ns4=\"ns:test4/\" xmlns:ns5=\"ns:test5/\" xmlns:ns6=\"ns:test6/\"> <ns1:NestedStructProp rdf:parseType=\"Resource\"> <ns2:Outer rdf:parseType=\"Resource\"> <ns3:Middle rdf:parseType=\"Resource\"> <ns4:Inner rdf:parseType=\"Resource\"> <ns5:Field1>Field1 value</ns5:Field1> <ns6:Field2>Field2 value</ns6:Field2> </ns4:Inner> </ns3:Middle> </ns2:Outer> </ns1:NestedStructProp> </rdf:Description> </rdf:RDF> </x:xmpmeta>");
    }

    {
        let meta = XmpMeta::from_str_requiring_xmp_meta(XMP_META_RDF, true).unwrap();

        println!(
            "Parse and require xmpmeta element, which is present = {:#?}",
            meta
        );

        assert_eq!(meta.to_string(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"Test:XMPCoreCoverage/XMP_META_RDF\" xmlns:ns1=\"ns:test1/\"> <ns1:XMPMetaProp>xmpmeta packet</ns1:XMPMetaProp> </rdf:Description> </rdf:RDF> </x:xmpmeta>");
    }

    {
        let meta = XmpMeta::from_str(INCONSISTENT_RDF).unwrap();

        println!("Parse and reconcile inconsistent aliases = {:#?}", meta);

        assert_eq!(meta.to_string(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"Test:XMPCoreCoverage/kInconsistentRDF\" xmlns:dc=\"http://purl.org/dc/elements/1.1/\"> <dc:creator> <rdf:Seq> <rdf:li>DC Creator [1]</rdf:li> </rdf:Seq> </dc:creator> </rdf:Description> </rdf:RDF> </x:xmpmeta>");
    }

    // try {
    // 	SXMPMeta meta;
    // 	meta.ParseFromBuffer ( INCONSISTENT_RDF, strlen(INCONSISTENT_RDF),
    // kXMP_StrictAliasing ); 	DumpXMPObj ( log, meta, "ERROR: Parse and do not
    // reconcile inconsistent aliases - should have thrown an exception" );
    // } catch ( XMP_Error & excep ) {
    // 	fprintf ( log, "\nParse and do not reconcile inconsistent aliases - threw
    // XMP_Error #%d : %s\n", excep.GetID(), excep.GetErrMsg() ); } catch ( ...
    // ) { 	fprintf ( log, "\nParse and do not reconcile inconsistent aliases -
    // threw unknown exception\n" ); }

    // {
    // 	write_major_label("Test CR and LF in values" );

    // 	const char *	kValueWithCR	= "ASCII \x0D CR";
    // 	const char *	kValueWithLF	= "ASCII \x0A LF";
    // 	const char *	kValueWithCRLF	= "ASCII \x0D\x0A CRLF";

    // 	SXMPMeta meta ( NEWLINE_RDF, kXMP_UseNullTermination );

    // 	meta.set_property ( NS2, "HasCR", kValueWithCR );
    // 	meta.set_property ( NS2, "HasLF", kValueWithLF );
    // 	meta.set_property ( NS2, "HasCRLF", kValueWithCRLF );

    // 	tmpStr1.erase();
    // 	meta.SerializeToBuffer ( &tmpStr1, kXMP_OmitPacketWrapper );
    // 	fprintf ( log, "\n%s\n", tmpStr1.c_str() );

    // 	tmpStr1.erase();  tmpStr2.erase();
    // 	ok = meta.property ( NS1, "HasCR", &tmpStr1, 0 );
    // 	ok = meta.property ( NS2, "HasCR", &tmpStr2, 0 );
    // 	if ( (tmpStr1 != kValueWithCR) || (tmpStr2 != kValueWithCR) ) fprintf ( log,
    // "\n ## HasCR values are bad\n" );

    // 	tmpStr1.erase();  tmpStr2.erase();
    // 	ok = meta.property ( NS1, "HasLF", &tmpStr1, 0 );
    // 	ok = meta.property ( NS2, "HasLF", &tmpStr2, 0 );
    // 	if ( (tmpStr1 != kValueWithLF) || (tmpStr2 != kValueWithLF) ) fprintf ( log,
    // "\n ## HasLF values are bad\n" );

    // 	tmpStr1.erase();  tmpStr2.erase();
    // 	ok = meta.property ( NS1, "HasCRLF", &tmpStr1, 0 );
    // 	ok = meta.property ( NS2, "HasCRLF", &tmpStr2, 0 );
    // 	if ( (tmpStr1 != kValueWithCRLF) || (tmpStr2 != kValueWithCRLF) ) fprintf (
    // log, "\n ## HasCRLF values are bad\n" ); }

    // {
    // 	write_major_label("Test serialization with various options" );

    // 	SXMPMeta meta ( SIMPLE_RDF, strlen(SIMPLE_RDF) );
    // 	meta.set_property ( NS2, "Another", "Something in another schema" );
    // 	meta.set_property ( NS2, "Yet/pdf:More", "Yet more in another schema" );

    // 	DumpXMPObj ( log, meta, "Parse simple RDF, serialize with various options" );

    // 	tmpStr1.erase();
    // 	meta.SerializeToBuffer ( &tmpStr1 );
    // 	WriteMinorLabel ( log, "Default serialize" );
    // 	fprintf ( log, "%s\n", tmpStr1.c_str() );  fflush ( log );
    // 	VerifyNewlines ( log, tmpStr1, "\x0A" );

    // 	SXMPMeta meta2 ( tmpStr1.c_str(), tmpStr1.size() );
    // 	DumpXMPObj ( log, meta2, "Reparse default serialization" );

    // 	tmpStr1.erase();
    // 	meta.SerializeToBuffer ( &tmpStr1, kXMP_OmitPacketWrapper |
    // kXMP_UseCompactFormat ); 	WriteMinorLabel ( log, "Compact RDF, no packet
    // serialize" ); 	fprintf ( log, "%s\n", tmpStr1.c_str() );

    // 	SXMPMeta meta3 ( tmpStr1.c_str(), tmpStr1.size() );
    // 	DumpXMPObj ( log, meta3, "Reparse compact serialization" );

    // 	{
    // 		SXMPMeta meta2;

    // 		meta2.set_property ( kXMP_NS_PDF, "Author", "PDF Author" );

    // 		tmpStr1.erase();
    // 		meta2.SerializeToBuffer ( &tmpStr1, kXMP_ReadOnlyPacket );
    // 		WriteMinorLabel ( log, "Read-only serialize with alias comments" );
    // 		fprintf ( log, "%s\n", tmpStr1.c_str() );

    // 		meta2.set_property ( kXMP_NS_PDF, "Actual", "PDF Actual" );
    // 		meta2.set_property ( kXMP_NS_XMP, "Actual", "XMP Actual" );

    // 		tmpStr1.erase();
    // 		meta2.SerializeToBuffer ( &tmpStr1, kXMP_ReadOnlyPacket );
    // 		WriteMinorLabel ( log, "Read-only serialize with alias comments (more
    // actuals)" ); 		fprintf ( log, "%s\n", tmpStr1.c_str() );
    // 	}

    // 	tmpStr1.erase();
    // 	meta.SerializeToBuffer ( &tmpStr1, kXMP_OmitPacketWrapper, 0, "\x0D" );
    // 	WriteMinorLabel ( log, "CR newline serialize" );
    // 	fprintf ( log, "%s\n", tmpStr1.c_str() );
    // 	VerifyNewlines ( log, tmpStr1, "\x0D" );

    // 	tmpStr1.erase();
    // 	meta.SerializeToBuffer ( &tmpStr1, kXMP_OmitPacketWrapper, 0, "\x0D\x0A" );
    // 	WriteMinorLabel ( log, "CRLF newline serialize" );
    // 	fprintf ( log, "%s\n", tmpStr1.c_str() );
    // 	VerifyNewlines ( log, tmpStr1, "\x0D\x0A" );

    // 	tmpStr1.erase();
    // 	meta.SerializeToBuffer ( &tmpStr1, kXMP_OmitPacketWrapper, 0, "<->" );
    // 	WriteMinorLabel ( log, "Alternate newline serialize" );
    // 	fprintf ( log, "%s\n", tmpStr1.c_str() );

    // 	tmpStr1.erase();
    // 	meta.SerializeToBuffer ( &tmpStr1, kXMP_OmitPacketWrapper, 0, "", "#", 3 );
    // 	WriteMinorLabel ( log, "Alternate indent serialize" );
    // 	fprintf ( log, "%s\n", tmpStr1.c_str() );

    // 	tmpStr1.erase();
    // 	meta.SerializeToBuffer ( &tmpStr1, 0, 10 );
    // 	WriteMinorLabel ( log, "Small padding serialize" );
    // 	fprintf ( log, "%s\n", tmpStr1.c_str() );

    // 	tmpStr1.erase();
    // 	tmpStr2.erase();
    // 	meta.SerializeToBuffer ( &tmpStr1 );
    // 	meta.SerializeToBuffer ( &tmpStr2, kXMP_IncludeThumbnailPad );
    // 	fprintf ( log, "Thumbnailpad adds %zd bytes\n", tmpStr2.size()-tmpStr1.size()
    // );

    // 	tmpStr1.erase();
    // 	meta.SerializeToBuffer ( &tmpStr1, kXMP_ReadOnlyPacket );
    // 	size_t minSize = tmpStr1.size();
    // 	fprintf ( log, "Minimum packet size is %zd bytes\n", minSize );

    // 	tmpStr1.erase();
    // 	meta.SerializeToBuffer ( &tmpStr1, kXMP_ExactPacketLength, minSize+1234 );
    // 	fprintf ( log, "Minimum+1234 packet size is %zd bytes\n", tmpStr1.size() );
    // 	if ( tmpStr1.size() != (minSize + 1234) ) fprintf ( log, "** Bad packet
    // length **\n" );

    // 	tmpStr1.erase();
    // 	meta.SerializeToBuffer ( &tmpStr1, kXMP_ExactPacketLength, minSize );
    // 	fprintf ( log, "Minimum+0 packet size is %zd bytes\n", tmpStr1.size() );
    // 	if ( tmpStr1.size() != minSize ) fprintf ( log, "** Bad packet length **\n"
    // );

    // 	try {
    // 		tmpStr1.erase();
    // 		meta.SerializeToBuffer ( &tmpStr1, kXMP_ExactPacketLength, minSize-1 );
    // 		fprintf ( log, "#ERROR: No exception for minimum-1, size is %zd bytes **\n",
    // tmpStr1.size() ); 	} catch ( XMP_Error & excep ) {
    // 		fprintf ( log, "Serialize in minimum-1 - threw XMP_Error #%d : %s\n",
    // excep.GetID(), excep.GetErrMsg() ); 	} catch ( ... ) {
    // 		fprintf ( log, "Serialize in minimum-1 - threw unknown exception\n" );
    // 	}

    // 	// *** UTF-16 and UTF-32 encodings

    // }

    // // --------------------------------------------------------------------------------------------
    // // Iteration methods
    // // -----------------

    // {
    // 	write_major_label("Test iteration methods" );

    // 	SXMPMeta meta ( RDF_COVERAGE, strlen ( RDF_COVERAGE ) );
    // 	XMP_OptionBits opt2;

    // 	meta.set_property ( NS2, "Prop", "Prop value" );

    // 	meta.set_property ( NS2, "Bag", 0, kXMP_PropValueIsArray );
    // 	meta.SetArrayItem ( NS2, "Bag", 1, "BagItem 2" );
    // 	meta.SetArrayItem ( NS2, "Bag", 1, "BagItem 1", kXMP_InsertBeforeItem );
    // 	meta.SetArrayItem ( NS2, "Bag", 2, "BagItem 3", kXMP_InsertAfterItem );

    // 	DumpXMPObj ( log, meta, "Parse \"coverage\" RDF, add Bag items out of order"
    // );

    // 	{
    // 		SXMPIterator iter ( meta );
    // 		WriteMinorLabel ( log, "Default iteration" );
    // 		while ( true ) {
    // 			tmpStr1.erase();  tmpStr2.erase();  tmpStr3.erase();
    // 			if ( ! iter.Next ( &tmpStr1, &tmpStr2, &tmpStr3, &options ) ) break;
    // 			fprintf ( log, "  %s %s = \"%s\", 0x%X\n", tmpStr1.c_str(), tmpStr2.c_str(),
    // tmpStr3.c_str(), options ); 			if ( ! (options & kXMP_SchemaNode) ) {
    // 				tmpStr4.erase();
    // 				ok = meta.property ( tmpStr1.c_str(), tmpStr2.c_str(), &tmpStr4, &opt2 );
    // 				if ( (! ok) || (tmpStr4 != tmpStr3) || (opt2 != options) ) {
    // 					fprintf ( log, "    ** property failed: %s, \"%s\", 0x%X\n",
    // FoundOrNot(ok), tmpStr4.c_str(), opt2 ); 				}
    // 			}
    // 		}
    // 	}

    // 	{
    // 		SXMPIterator iter ( meta, kXMP_IterOmitQualifiers );
    // 		WriteMinorLabel ( log, "Iterate omitting qualifiers" );
    // 		while ( true ) {
    // 			tmpStr1.erase();  tmpStr2.erase();  tmpStr3.erase();
    // 			if ( ! iter.Next ( &tmpStr1, &tmpStr2, &tmpStr3, &options ) ) break;
    // 			fprintf ( log, "  %s %s = \"%s\", 0x%X\n", tmpStr1.c_str(), tmpStr2.c_str(),
    // tmpStr3.c_str(), options ); 			if ( ! (options & kXMP_SchemaNode) ) {
    // 				tmpStr4.erase();
    // 				ok = meta.property ( tmpStr1.c_str(), tmpStr2.c_str(), &tmpStr4, &opt2 );
    // 				if ( (! ok) || (tmpStr4 != tmpStr3) || (opt2 != options) ) {
    // 					fprintf ( log, "    ** property failed: %s, \"%s\", 0x%X\n",
    // FoundOrNot(ok), tmpStr4.c_str(), opt2 ); 				}
    // 			}
    // 		}
    // 	}

    // 	{
    // 		SXMPIterator iter ( meta, kXMP_IterJustLeafName );
    // 		WriteMinorLabel ( log, "Iterate with just leaf names" );
    // 		while ( true ) {
    // 			tmpStr1.erase();  tmpStr2.erase();  tmpStr3.erase();
    // 			if ( ! iter.Next ( &tmpStr1, &tmpStr2, &tmpStr3, &options ) ) break;
    // 			fprintf ( log, "  %s %s = \"%s\", 0x%X\n", tmpStr1.c_str(), tmpStr2.c_str(),
    // tmpStr3.c_str(), options ); 		}
    // 	}

    // 	{
    // 		SXMPIterator iter ( meta, kXMP_IterJustLeafNodes );
    // 		WriteMinorLabel ( log, "Iterate just the leaf nodes" );
    // 		while ( true ) {
    // 			tmpStr1.erase();  tmpStr2.erase();  tmpStr3.erase();
    // 			if ( ! iter.Next ( &tmpStr1, &tmpStr2, &tmpStr3, &options ) ) break;
    // 			fprintf ( log, "  %s %s = \"%s\", 0x%X\n", tmpStr1.c_str(), tmpStr2.c_str(),
    // tmpStr3.c_str(), options ); 			if ( ! (options & kXMP_SchemaNode) ) {
    // 				tmpStr4.erase();
    // 				ok = meta.property ( tmpStr1.c_str(), tmpStr2.c_str(), &tmpStr4, &opt2 );
    // 				if ( (! ok) || (tmpStr4 != tmpStr3) || (opt2 != options) ) {
    // 					fprintf ( log, "    ** property failed: %s, \"%s\", 0x%X\n",
    // FoundOrNot(ok), tmpStr4.c_str(), opt2 ); 				}
    // 			}
    // 		}
    // 	}

    // 	{
    // 		SXMPIterator iter ( meta, kXMP_IterJustChildren );
    // 		WriteMinorLabel ( log, "Iterate just the schema nodes" );
    // 		while ( true ) {
    // 			tmpStr1.erase();  tmpStr2.erase();  tmpStr3.erase();
    // 			if ( ! iter.Next ( &tmpStr1, &tmpStr2, &tmpStr3, &options ) ) break;
    // 			fprintf ( log, "  %s %s = \"%s\", 0x%X\n", tmpStr1.c_str(), tmpStr2.c_str(),
    // tmpStr3.c_str(), options ); 		}
    // 	}

    // 	{
    // 		SXMPIterator iter ( meta, NS2 );
    // 		WriteMinorLabel ( log, "Iterate the ns2: namespace" );
    // 		while ( true ) {
    // 			tmpStr1.erase();  tmpStr2.erase();  tmpStr3.erase();
    // 			if ( ! iter.Next ( &tmpStr1, &tmpStr2, &tmpStr3, &options ) ) break;
    // 			fprintf ( log, "  %s %s = \"%s\", 0x%X\n", tmpStr1.c_str(), tmpStr2.c_str(),
    // tmpStr3.c_str(), options ); 			if ( ! (options & kXMP_SchemaNode) ) {
    // 				tmpStr4.erase();
    // 				ok = meta.property ( tmpStr1.c_str(), tmpStr2.c_str(), &tmpStr4, &opt2 );
    // 				if ( (! ok) || (tmpStr4 != tmpStr3) || (opt2 != options) ) {
    // 					fprintf ( log, "    ** property failed: %s, \"%s\", 0x%X\n",
    // FoundOrNot(ok), tmpStr4.c_str(), opt2 ); 				}
    // 			}
    // 		}
    // 	}

    // 	{
    // 		SXMPIterator iter ( meta, NS2, "Bag" );
    // 		WriteMinorLabel ( log, "Start at ns2:Bag" );
    // 		while ( true ) {
    // 			tmpStr1.erase();  tmpStr2.erase();  tmpStr3.erase();
    // 			if ( ! iter.Next ( &tmpStr1, &tmpStr2, &tmpStr3, &options ) ) break;
    // 			fprintf ( log, "  %s %s = \"%s\", 0x%X\n", tmpStr1.c_str(), tmpStr2.c_str(),
    // tmpStr3.c_str(), options ); 			if ( ! (options & kXMP_SchemaNode) ) {
    // 				tmpStr4.erase();
    // 				ok = meta.property ( tmpStr1.c_str(), tmpStr2.c_str(), &tmpStr4, &opt2 );
    // 				if ( (! ok) || (tmpStr4 != tmpStr3) || (opt2 != options) ) {
    // 					fprintf ( log, "    ** property failed: %s, \"%s\", 0x%X\n",
    // FoundOrNot(ok), tmpStr4.c_str(), opt2 ); 				}
    // 			}
    // 		}
    // 	}

    // 	{
    // 		SXMPIterator iter ( meta, NS2, "NestedStructProp/ns1:Outer" );
    // 		WriteMinorLabel ( log, "Start at ns2:NestedStructProp/ns1:Outer" );
    // 		while ( true ) {
    // 			tmpStr1.erase();  tmpStr2.erase();  tmpStr3.erase();
    // 			if ( ! iter.Next ( &tmpStr1, &tmpStr2, &tmpStr3, &options ) ) break;
    // 			fprintf ( log, "  %s %s = \"%s\", 0x%X\n", tmpStr1.c_str(), tmpStr2.c_str(),
    // tmpStr3.c_str(), options ); 			if ( ! (options & kXMP_SchemaNode) ) {
    // 				tmpStr4.erase();
    // 				ok = meta.property ( tmpStr1.c_str(), tmpStr2.c_str(), &tmpStr4, &opt2 );
    // 				if ( (! ok) || (tmpStr4 != tmpStr3) || (opt2 != options) ) {
    // 					fprintf ( log, "    ** property failed: %s, \"%s\", 0x%X\n",
    // FoundOrNot(ok), tmpStr4.c_str(), opt2 ); 				}
    // 			}
    // 		}
    // 	}

    // 	{
    // 		SXMPIterator iter ( meta, "ns:empty/" );
    // 		WriteMinorLabel ( log, "Iterate an empty namespace" );
    // 		while ( true ) {
    // 			tmpStr1.erase();  tmpStr2.erase();  tmpStr3.erase();
    // 			if ( ! iter.Next ( &tmpStr1, &tmpStr2, &tmpStr3, &options ) ) break;
    // 			fprintf ( log, "  %s %s = \"%s\", 0x%X\n", tmpStr1.c_str(), tmpStr2.c_str(),
    // tmpStr3.c_str(), options ); 			if ( ! (options & kXMP_SchemaNode) ) {
    // 				tmpStr4.erase();
    // 				ok = meta.property ( tmpStr1.c_str(), tmpStr2.c_str(), &tmpStr4, &opt2 );
    // 				if ( (! ok) || (tmpStr4 != tmpStr3) || (opt2 != options) ) {
    // 					fprintf ( log, "    ** property failed: %s, \"%s\", 0x%X\n",
    // FoundOrNot(ok), tmpStr4.c_str(), opt2 ); 				}
    // 			}
    // 		}
    // 	}

    // 	{
    // 		SXMPIterator iter ( meta, NS2, "", kXMP_IterJustChildren |
    // kXMP_IterJustLeafName ); 		WriteMinorLabel ( log, "Iterate the top of the
    // ns2: namespace with just leaf names" ); 		while ( true ) {
    // 			tmpStr1.erase();  tmpStr2.erase();  tmpStr3.erase();
    // 			if ( ! iter.Next ( &tmpStr1, &tmpStr2, &tmpStr3, &options ) ) break;
    // 			fprintf ( log, "  %s %s = \"%s\", 0x%X\n", tmpStr1.c_str(), tmpStr2.c_str(),
    // tmpStr3.c_str(), options ); 		}
    // 	}

    // 	{
    // 		SXMPIterator iter ( meta, NS2, "", kXMP_IterJustChildren |
    // kXMP_IterJustLeafNodes ); 		WriteMinorLabel ( log, "Iterate the top of the
    // ns2: namespace visiting just leaf nodes" ); 		while ( true ) {
    // 			tmpStr1.erase();  tmpStr2.erase();  tmpStr3.erase();
    // 			if ( ! iter.Next ( &tmpStr1, &tmpStr2, &tmpStr3, &options ) ) break;
    // 			fprintf ( log, "  %s %s = \"%s\", 0x%X\n", tmpStr1.c_str(), tmpStr2.c_str(),
    // tmpStr3.c_str(), options ); 			if ( ! (options & kXMP_SchemaNode) ) {
    // 				tmpStr4.erase();
    // 				ok = meta.property ( tmpStr1.c_str(), tmpStr2.c_str(), &tmpStr4, &opt2 );
    // 				if ( (! ok) || (tmpStr4 != tmpStr3) || (opt2 != options) ) {
    // 					fprintf ( log, "    ** property failed: %s, \"%s\", 0x%X\n",
    // FoundOrNot(ok), tmpStr4.c_str(), opt2 ); 				}
    // 			}
    // 		}
    // 	}

    // 	{
    // 		SXMPIterator iter ( meta, NS2, "Bag", kXMP_IterJustChildren );
    // 		WriteMinorLabel ( log, "Iterate just the children of ns2:Bag" );
    // 		while ( true ) {
    // 			tmpStr1.erase();  tmpStr2.erase();  tmpStr3.erase();
    // 			if ( ! iter.Next ( &tmpStr1, &tmpStr2, &tmpStr3, &options ) ) break;
    // 			fprintf ( log, "  %s %s = \"%s\", 0x%X\n", tmpStr1.c_str(), tmpStr2.c_str(),
    // tmpStr3.c_str(), options ); 			if ( ! (options & kXMP_SchemaNode) ) {
    // 				tmpStr4.erase();
    // 				ok = meta.property ( tmpStr1.c_str(), tmpStr2.c_str(), &tmpStr4, &opt2 );
    // 				if ( (! ok) || (tmpStr4 != tmpStr3) || (opt2 != options) ) {
    // 					fprintf ( log, "    ** property failed: %s, \"%s\", 0x%X\n",
    // FoundOrNot(ok), tmpStr4.c_str(), opt2 ); 				}
    // 			}
    // 		}
    // 	}

    // 	{
    // 		SXMPIterator iter ( meta, NS2, "Bag", kXMP_IterJustChildren |
    // kXMP_IterJustLeafName ); 		WriteMinorLabel ( log, "Iterate just the
    // children of ns2:Bag with just leaf names" ); 		while ( true ) {
    // 			tmpStr1.erase();  tmpStr2.erase();  tmpStr3.erase();
    // 			if ( ! iter.Next ( &tmpStr1, &tmpStr2, &tmpStr3, &options ) ) break;
    // 			fprintf ( log, "  %s %s = \"%s\", 0x%X\n", tmpStr1.c_str(), tmpStr2.c_str(),
    // tmpStr3.c_str(), options ); 		}
    // 	}

    // 	{
    // 		SXMPIterator iter ( meta, NS2, "NestedStructProp/ns1:Outer/ns1:Middle",
    // kXMP_IterJustChildren ); 		WriteMinorLabel ( log, "Iterate just the
    // children of ns2:NestedStructProp/ns1:Outer/ns1:Middle" ); 		while ( true )
    // { 			tmpStr1.erase();  tmpStr2.erase();  tmpStr3.erase();
    // 			if ( ! iter.Next ( &tmpStr1, &tmpStr2, &tmpStr3, &options ) ) break;
    // 			fprintf ( log, "  %s %s = \"%s\", 0x%X\n", tmpStr1.c_str(), tmpStr2.c_str(),
    // tmpStr3.c_str(), options ); 			if ( ! (options & kXMP_SchemaNode) ) {
    // 				tmpStr4.erase();
    // 				ok = meta.property ( tmpStr1.c_str(), tmpStr2.c_str(), &tmpStr4, &opt2 );
    // 				if ( (! ok) || (tmpStr4 != tmpStr3) || (opt2 != options) ) {
    // 					fprintf ( log, "    ** property failed: %s, \"%s\", 0x%X\n",
    // FoundOrNot(ok), tmpStr4.c_str(), opt2 ); 				}
    // 			}
    // 		}
    // 	}

    // 	{
    // 		SXMPIterator iter ( meta );
    // 		WriteMinorLabel ( log, "Skip children of ArrayProp2, and siblings after
    // StructProp" ); 		while ( true ) {
    // 			tmpStr1.erase();  tmpStr2.erase();  tmpStr3.erase();
    // 			if ( ! iter.Next ( &tmpStr1, &tmpStr2, &tmpStr3, &options ) ) break;
    // 			fprintf ( log, "  %s %s = \"%s\", 0x%X\n", tmpStr1.c_str(), tmpStr2.c_str(),
    // tmpStr3.c_str(), options ); 			if ( ! (options & kXMP_SchemaNode) ) {
    // 				tmpStr4.erase();
    // 				ok = meta.property ( tmpStr1.c_str(), tmpStr2.c_str(), &tmpStr4, &opt2 );
    // 				if ( (! ok) || (tmpStr4 != tmpStr3) || (opt2 != options) ) {
    // 					fprintf ( log, "    ** property failed: %s, \"%s\", 0x%X\n",
    // FoundOrNot(ok), tmpStr4.c_str(), opt2 ); 				}
    // 			}
    // 			if ( tmpStr2 == "ns1:ArrayProp2" ) iter.Skip ( kXMP_IterSkipSubtree );
    // 			if ( tmpStr2 == "ns1:StructProp" ) iter.Skip ( kXMP_IterSkipSiblings );
    // 		}
    // 	}

    // 	{
    // 		SXMPMeta meta;

    // 		meta.set_property ( kXMP_NS_PDF, "Author", "PDF Author" );
    // 		meta.set_property ( kXMP_NS_PDF, "PDFProp", "PDF Prop" );
    // 		meta.set_property ( kXMP_NS_XMP, "XMPProp", "XMP Prop" );
    // 		meta.set_property ( kXMP_NS_DC, "DCProp", "DC Prop" );

    // 		SXMPIterator iter1 ( meta );
    // 		WriteMinorLabel ( log, "Iterate without showing aliases" );
    // 		while ( true ) {
    // 			tmpStr1.erase();  tmpStr2.erase();  tmpStr3.erase();
    // 			if ( ! iter1.Next ( &tmpStr1, &tmpStr2, &tmpStr3, &options ) ) break;
    // 			fprintf ( log, "  %s %s = \"%s\", 0x%X\n", tmpStr1.c_str(), tmpStr2.c_str(),
    // tmpStr3.c_str(), options ); 			if ( ! (options & kXMP_SchemaNode) ) {
    // 				tmpStr4.erase();
    // 				options &= kXMP_PropHasAliases;	// So the comparison below works.
    // 				ok = meta.property ( tmpStr1.c_str(), tmpStr2.c_str(), &tmpStr4, &opt2 );
    // 				if ( (! ok) || (tmpStr4 != tmpStr3) || (opt2 != options) ) {
    // 					fprintf ( log, "    ** property failed: %s, \"%s\", 0x%X\n",
    // FoundOrNot(ok), tmpStr4.c_str(), opt2 ); 				}
    // 			}
    // 		}
    // 	}

    // }

    // // --------------------------------------------------------------------------------------------
    // // XPath composition utilities
    // // ---------------------------

    // {
    // 	write_major_label("Test XPath composition utilities" );

    // 	SXMPMeta meta ( SIMPLE_RDF, strlen(SIMPLE_RDF) );
    // 	DumpXMPObj ( log, meta, "Parse simple RDF" );
    // 	fprintf ( log, "\n" );

    // 	tmpStr1.erase();
    // 	SXMPUtils::ComposeArrayItemPath ( NS1, "ArrayProp", 2, &tmpStr1 );
    // 	fprintf ( log, "ComposeArrayItemPath ns1:ArrayProp[2] : %s\n",
    // tmpStr1.c_str() ); 	meta.set_property ( NS1, tmpStr1.c_str(), "new
    // ns1:ArrayProp[2] value" );

    // 	fprintf ( log, "\n" );

    // 	tmpStr1.erase();
    // 	SXMPUtils::ComposeStructFieldPath ( NS1, "StructProp", NS2, "Field3",
    // &tmpStr1 ); 	fprintf ( log, "ComposeStructFieldPath
    // ns1:StructProp/ns2:Field3 : %s\n", tmpStr1.c_str() ); 	meta.set_property (
    // NS1, tmpStr1.c_str(), "new ns1:StructProp/ns2:Field3 value" );

    // 	tmpStr1.erase();
    // 	SXMPUtils::ComposeQualifierPath ( NS1, "QualProp", NS2, "Qual", &tmpStr1 );
    // 	fprintf ( log, "ComposeQualifierPath ns1:QualProp/?ns2:Qual : %s\n",
    // tmpStr1.c_str() ); 	meta.set_property ( NS1, tmpStr1.c_str(), "new
    // ns1:QualProp/?ns2:Qual value" );

    // 	fprintf ( log, "\n" );

    // 	tmpStr1.erase();
    // 	SXMPUtils::ComposeQualifierPath ( NS1, "AltTextProp", xmp_ns::XML, "lang",
    // &tmpStr1 ); 	fprintf ( log, "ComposeQualifierPath
    // ns1:AltTextProp/?xml:lang : %s\n", tmpStr1.c_str() ); 	meta.set_property (
    // NS1, tmpStr1.c_str(), "new ns1:AltTextProp/?xml:lang value" );

    // 	tmpStr1.erase();
    // 	tmpStr2 = "x-two";
    // 	SXMPUtils::ComposeLangSelector ( NS1, "AltTextProp", tmpStr2, &tmpStr1 );
    // 	fprintf ( log, "ComposeLangSelector ns1:AltTextProp['x-two'] : %s\n",
    // tmpStr1.c_str() ); 	meta.set_property ( NS1, tmpStr1.c_str(), "new
    // ns1:AltTextProp['x-two'] value" );

    // 	fprintf ( log, "\n" );

    // 	fprintf ( log, "Check field selector usage\n" ); fflush ( log );

    // 	tmpStr1.erase();
    // 	ok = meta.property ( NS1, "ArrayOfStructProp[ns2:Field1='Item-2']",
    // &tmpStr1, &options );
    // 	fprintf ( log, "property ArrayOfStructProp[ns2:Field1='Item-2'] : %s, \"%s\",
    // 0x%X\n", FoundOrNot ( ok ), tmpStr1.c_str(), options ); fflush ( log );

    // 	tmpStr1.erase();
    // 	ok = meta.property ( NS1,
    // "ArrayOfStructProp[ns2:Field1='Item-2']/ns2:Field2", &tmpStr1, &options );
    // 	fprintf ( log, "property ArrayOfStructProp[ns2:Field1='Item-2']/ns2:Field2 : %s, \"%s\", 0x%X\n", FoundOrNot ( ok ), tmpStr1.c_str(), options ); fflush ( log );

    // 	tmpStr1.erase();
    // 	tmpStr2 = "Item-2";
    // 	SXMPUtils::ComposeFieldSelector ( NS1, "ArrayOfStructProp", NS2, "Field1",
    // tmpStr2, &tmpStr1 ); 	fprintf ( log, "ComposeFieldSelector
    // ns1:ArrayOfStructProp[ns2:Field1=Item-2] : %s\n", tmpStr1.c_str() );

    // 	tmpStr2.erase();
    // 	SXMPUtils::ComposeStructFieldPath ( NS1, tmpStr1.c_str(), NS2, "Field2",
    // &tmpStr2 );
    // 	fprintf ( log, "ComposeStructFieldPath ns1:ArrayOfStructProp[ns2:Field1=Item-2]/ns2:Field2 : %s\n", tmpStr2.c_str() );
    // 	meta.set_property ( NS1, tmpStr2.c_str(), "new
    // ns1:ArrayOfStructProp[ns2:Field1=Item-2]/ns2:Field2 value" );

    // 	DumpXMPObj ( log, meta, "Modified simple RDF" );

    // }

    // // --------------------------------------------------------------------------------------------
    // // Value conversion utilities
    // // --------------------------

    // write_major_label("Test value conversion utilities" );
    // fprintf ( log, "\n" );

    // tmpStr1.erase();
    // SXMPUtils::ConvertFromBool ( true, &tmpStr1 );
    // fprintf ( log, "ConverFromBool true : %s\n", tmpStr1.c_str() );
    // tmpStr1.erase();
    // SXMPUtils::ConvertFromBool ( false, &tmpStr1 );
    // fprintf ( log, "ConverFromBool false : %s\n", tmpStr1.c_str() );

    // fprintf ( log, "\n" );

    // ok = SXMPUtils::ConvertToBool ( kXMP_TrueStr );
    // fprintf ( log, "ConverToBool kXMP_TrueStr : %d\n", (int)ok );
    // ok = SXMPUtils::ConvertToBool ( kXMP_FalseStr );
    // fprintf ( log, "ConverToBool kXMP_FalseStr : %d\n", (int)ok );

    // fprintf ( log, "\n" );

    // tmpStr1 = "true";
    // ok = SXMPUtils::ConvertToBool ( tmpStr1 );
    // fprintf ( log, "ConverToBool true : %d\n", (int)ok );
    // ok = SXMPUtils::ConvertToBool ( "TRUE" );
    // fprintf ( log, "ConverToBool TRUE : %d\n", (int)ok );
    // ok = SXMPUtils::ConvertToBool ( "t" );
    // fprintf ( log, "ConverToBool t : %d\n", (int)ok );
    // ok = SXMPUtils::ConvertToBool ( "1" );
    // fprintf ( log, "ConverToBool 1 : %d\n", (int)ok );

    // fprintf ( log, "\n" );

    // ok = SXMPUtils::ConvertToBool ( "false" );
    // fprintf ( log, "ConverToBool false : %d\n", (int)ok );
    // ok = SXMPUtils::ConvertToBool ( "FALSE" );
    // fprintf ( log, "ConverToBool FALSE : %d\n", (int)ok );
    // ok = SXMPUtils::ConvertToBool ( "f" );
    // fprintf ( log, "ConverToBool f : %d\n", (int)ok );
    // ok = SXMPUtils::ConvertToBool ( "0" );
    // fprintf ( log, "ConverToBool 0 : %d\n", (int)ok );

    // fprintf ( log, "\n" );

    // tmpStr1.erase();
    // SXMPUtils::ConvertFromInt ( 0, 0, &tmpStr1 );
    // fprintf ( log, "ConverFromInt 0 : %s\n", tmpStr1.c_str() );
    // tmpStr1.erase();
    // SXMPUtils::ConvertFromInt ( 42, 0, &tmpStr1 );
    // fprintf ( log, "ConverFromInt 42 : %s\n", tmpStr1.c_str() );
    // tmpStr1.erase();
    // SXMPUtils::ConvertFromInt ( -42, 0, &tmpStr1 );
    // fprintf ( log, "ConverFromInt -42 : %s\n", tmpStr1.c_str() );
    // tmpStr1.erase();
    // SXMPUtils::ConvertFromInt ( 0x7FFFFFFF, 0, &tmpStr1 );
    // fprintf ( log, "ConverFromInt 0x7FFFFFFF : %s\n", tmpStr1.c_str() );
    // tmpStr1.erase();
    // SXMPUtils::ConvertFromInt ( 0x80000000, 0, &tmpStr1 );
    // fprintf ( log, "ConverFromInt 0x80000000 : %s\n", tmpStr1.c_str() );
    // tmpStr1.erase();
    // SXMPUtils::ConvertFromInt ( 0x7FFFFFFF, "%X", &tmpStr1 );
    // fprintf ( log, "ConverFromInt 0x7FFFFFFF as hex : %s\n", tmpStr1.c_str() );
    // tmpStr1.erase();
    // SXMPUtils::ConvertFromInt ( 0x80000000, "%X", &tmpStr1 );
    // fprintf ( log, "ConverFromInt 0x80000000 as hex : %s\n", tmpStr1.c_str() );

    // fprintf ( log, "\n" );

    // long	int1;

    // tmpStr1 = "0";
    // int1 = SXMPUtils::ConvertToInt ( tmpStr1 );
    // fprintf ( log, "ConvertToInt 0 : %ld\n", int1 );
    // int1 = SXMPUtils::ConvertToInt ( "42" );
    // fprintf ( log, "ConvertToInt 42 : %ld\n", int1 );
    // int1 = SXMPUtils::ConvertToInt ( "-42" );
    // fprintf ( log, "ConvertToInt -42 : %ld\n", int1 );
    // int1 = SXMPUtils::ConvertToInt ( "0x7FFFFFFF" );
    // fprintf ( log, "ConvertToInt 0x7FFFFFFF : %ld\n", int1 );
    // int1 = SXMPUtils::ConvertToInt ( "0x80000000" );
    // fprintf ( log, "ConvertToInt 0x80000000 : %ld\n", int1 );
    // int1 = SXMPUtils::ConvertToInt ( "0x7FFFFFFF" );
    // fprintf ( log, "ConvertToInt 0x7FFFFFFF as hex : %lX\n", int1 );
    // int1 = SXMPUtils::ConvertToInt ( "0x80000000" );
    // fprintf ( log, "ConvertToInt 0x80000000 as hex : %lX\n", int1 );

    // fprintf ( log, "\n" );

    // tmpStr1.erase();
    // SXMPUtils::ConvertFromFloat ( 0, 0, &tmpStr1 );
    // fprintf ( log, "ConverFromFloat 0 : %s\n", tmpStr1.c_str() );
    // tmpStr1.erase();
    // SXMPUtils::ConvertFromFloat ( 4.2, 0, &tmpStr1 );
    // fprintf ( log, "ConverFromFloat 4.2 : %s\n", tmpStr1.c_str() );
    // tmpStr1.erase();
    // SXMPUtils::ConvertFromFloat ( -4.2, 0, &tmpStr1 );
    // fprintf ( log, "ConverFromFloat -4.2 : %s\n", tmpStr1.c_str() );
    // tmpStr1.erase();
    // SXMPUtils::ConvertFromFloat ( (int)0x7FFFFFFF, 0, &tmpStr1 );
    // fprintf ( log, "ConverFromFloat 0x7FFFFFFF : %s\n", tmpStr1.c_str() );
    // tmpStr1.erase();
    // SXMPUtils::ConvertFromFloat ( (int)0x80000000, 0, &tmpStr1 );
    // fprintf ( log, "ConverFromFloat 0x80000000 : %s\n", tmpStr1.c_str() );
    // tmpStr1.erase();
    // SXMPUtils::ConvertFromFloat ( (int)0x7FFFFFFF, "%f", &tmpStr1 );
    // fprintf ( log, "ConverFromFloat 0x7FFFFFFF as f : %s\n", tmpStr1.c_str() );
    // tmpStr1.erase();
    // SXMPUtils::ConvertFromFloat ( (int)0x80000000, "%f", &tmpStr1 );
    // fprintf ( log, "ConverFromFloat 0x80000000 as f : %s\n", tmpStr1.c_str() );

    // fprintf ( log, "\n" );

    // double	float1;

    // tmpStr1 = "0";
    // float1 = SXMPUtils::ConvertToFloat ( tmpStr1 );
    // fprintf ( log, "ConvertToFloat 0 : %f\n", float1 );
    // float1 = SXMPUtils::ConvertToFloat ( "4.2" );
    // fprintf ( log, "ConvertToFloat 4.2 : %f\n", float1 );
    // float1 = SXMPUtils::ConvertToFloat ( "-4.2" );
    // fprintf ( log, "ConvertToFloat -4.2 : %f\n", float1 );

    // fprintf ( log, "\n" );

    // XMP_DateTime date1, date2;
    // FillDateTime ( &date1, 2000, 1, 31, 12, 34, 56, true, true, true, -1, 8, 0, 0
    // );

    // tmpStr1.erase();
    // SXMPUtils::ConvertFromDate ( date1, &tmpStr1 );
    // fprintf ( log, "ConvertFromDate 2000 Jan 31 12:34:56 PST : %s\n",
    // tmpStr1.c_str() );

    // SXMPUtils::ConvertToDate ( tmpStr1, &date2 );
    // fprintf ( log, "ConvertToDate : %d-%02d-%02d %02d:%02d:%02d %d*%02d:%02d
    // %d\n", 		  date2.year, date2.month, date2.day, date2.hour, date2.minute,
    // date2.second, 		  date2.tzSign, date2.tzHour, date2.tzMinute,
    // date2.nanoSecond );

    // // --------------------------------------------------------------------------------------------
    // // Date/Time utilities
    // // -------------------

    // {
    // 	write_major_label("Test date/time utilities and special values" );
    // 	fprintf ( log, "\n" );

    // 	XMP_DateTime utcNow, localNow;

    // 	SXMPUtils::SetTimeZone ( &utcNow );
    // 	fprintf ( log, "SetTimeZone : %d-%02d-%02d %02d:%02d:%02d %d*%02d:%02d %d\n",
    // 			  utcNow.year, utcNow.month, utcNow.day, utcNow.hour, utcNow.minute,
    // utcNow.second, 			  utcNow.tzSign, utcNow.tzHour, utcNow.tzMinute,
    // utcNow.nanoSecond );

    // 	SXMPUtils::CurrentDateTime ( &utcNow );
    // 	fprintf ( log, "CurrentDateTime : %d-%02d-%02d %02d:%02d:%02d %d*%02d:%02d
    // %d\n", 			  utcNow.year, utcNow.month, utcNow.day, utcNow.hour,
    // utcNow.minute, utcNow.second, 			  utcNow.tzSign, utcNow.tzHour,
    // utcNow.tzMinute, utcNow.nanoSecond );

    // 	localNow = utcNow;
    // 	SXMPUtils::ConvertToLocalTime ( &localNow );
    // 	fprintf ( log, "ConvertToLocalTime : %d-%02d-%02d %02d:%02d:%02d %d*%02d:%02d
    // %d\n", 			  localNow.year, localNow.month, localNow.day, localNow.hour,
    // localNow.minute, localNow.second, 			  localNow.tzSign, localNow.tzHour,
    // localNow.tzMinute, localNow.nanoSecond );

    // 	utcNow = localNow;
    // 	SXMPUtils::ConvertToUTCTime ( &utcNow );
    // 	fprintf ( log, "ConvertToUTCTime : %d-%02d-%02d %02d:%02d:%02d %d*%02d:%02d
    // %d\n", 			  utcNow.year, utcNow.month, utcNow.day, utcNow.hour,
    // utcNow.minute, utcNow.second, 			  utcNow.tzSign, utcNow.tzHour,
    // utcNow.tzMinute, utcNow.nanoSecond );

    // 	fprintf ( log, "\n" );

    // 	i = SXMPUtils::CompareDateTime ( utcNow, localNow );
    // 	fprintf ( log, "CompareDateTime with a == b : %d\n", i );

    // 	utcNow.second = 0;
    // 	localNow.second = 30;
    // 	i = SXMPUtils::CompareDateTime ( utcNow, localNow );
    // 	fprintf ( log, "CompareDateTime with a < b : %d\n", i );

    // 	utcNow.second = 59;
    // 	i = SXMPUtils::CompareDateTime ( utcNow, localNow );
    // 	fprintf ( log, "CompareDateTime with a > b : %d\n", i );

    // }

    // // --------------------------------------------------------------------------------------------
    // // Miscellaneous utilities
    // // -----------------------

    // {
    // 	write_major_label("Test CatenateArrayItems and SeparateArrayItems" );
    // 	fprintf ( log, "\n" );

    // 	SXMPMeta meta;

    // 	meta.AppendArrayItem ( NS1, "Array1", kXMP_PropValueIsArray, "one" );
    // 	meta.AppendArrayItem ( NS1, "Array1", 0, "two" );
    // 	meta.AppendArrayItem ( NS1, "Array1", kXMP_PropValueIsArray, "3, three" );
    // 	meta.AppendArrayItem ( NS1, "Array1", 0, "4; four" );

    // 	DumpXMPObj ( log, meta, "Initial array" );
    // 	fprintf ( log, "\n" );

    // 	tmpStr1.erase();
    // 	SXMPUtils::CatenateArrayItems ( meta, NS1, "Array1", "; ", "\"",
    // kXMP_NoOptions, &tmpStr1 ); 	fprintf ( log, "CatenateArrayItems, no commas
    // : %s\n", tmpStr1.c_str() );

    // 	tmpStr2.erase();
    // 	SXMPUtils::CatenateArrayItems ( meta, NS1, "Array1", " ; ", "\"",
    // kXMPUtil_AllowCommas, &tmpStr2 ); 	fprintf ( log, "CatenateArrayItems,
    // allow commas : %s\n", tmpStr2.c_str() );

    // 	SXMPUtils::SeparateArrayItems ( &meta, NS1, "Array2-1", kXMP_NoOptions,
    // tmpStr1.c_str() ); 	SXMPUtils::SeparateArrayItems ( &meta, NS1,
    // "Array2-2", kXMPUtil_AllowCommas, tmpStr1.c_str() );

    // 	SXMPUtils::SeparateArrayItems ( &meta, NS1, "Array3-1",
    // kXMP_PropArrayIsOrdered, tmpStr2 ); 	SXMPUtils::SeparateArrayItems (
    // &meta, NS1, "Array3-2", (kXMP_PropArrayIsOrdered | kXMPUtil_AllowCommas),
    // tmpStr2 );

    // 	DumpXMPObj ( log, meta, "Set Array1, cat and split into others" );

    // 	SXMPUtils::SeparateArrayItems ( &meta, NS1, "Array2-2", kXMP_NoOptions,
    // tmpStr1.c_str() );	// Repeat into existing arrays.
    // 	SXMPUtils::SeparateArrayItems ( &meta, NS1, "Array3-2",
    // kXMP_PropArrayIsOrdered, tmpStr2.c_str() );

    // }

    // // --------------------------------------------------------------------------------------------

    // {
    // 	write_major_label("Test RemoveProperties and AppendProperties" );

    // 	SXMPMeta meta1 ( SIMPLE_RDF, strlen(SIMPLE_RDF) );

    // 	meta1.set_property ( NS2, "Prop", "value" );
    // 	DumpXMPObj ( log, meta1, "Parse simple RDF, add ns2:Prop" );

    // 	SXMPUtils::RemoveProperties ( &meta1, NS1, "ArrayOfStructProp" );
    // 	DumpXMPObj ( log, meta1, "Remove ns1:ArrayOfStructProp" );

    // 	SXMPUtils::RemoveProperties ( &meta1, NS1 );
    // 	DumpXMPObj ( log, meta1, "Remove all of ns1:" );

    // 	meta1.set_property ( kXMP_NS_XMP, "CreatorTool", "XMPCoverage" );
    // 	meta1.set_property ( kXMP_NS_XMP, "Nickname", "TXMP test" );
    // 	DumpXMPObj ( log, meta1, "Set xmp:CreatorTool (internal) and xmp:Nickname
    // (external)" );

    // 	SXMPUtils::RemoveProperties ( &meta1 );
    // 	DumpXMPObj ( log, meta1, "Remove all external properties" );

    // 	SXMPUtils::RemoveProperties ( &meta1, 0, 0, kXMPUtil_DoAllProperties );
    // 	DumpXMPObj ( log, meta1, "Remove all properties, including internal" );

    // 	meta1.set_property ( kXMP_NS_XMP, "CreatorTool", "XMPCoverage" );
    // 	meta1.set_property ( kXMP_NS_XMP, "Nickname", "TXMP test" );
    // 	DumpXMPObj ( log, meta1, "Set xmp:CreatorTool and xmp:Nickname again" );

    // 	SXMPMeta meta2 ( SIMPLE_RDF, strlen(SIMPLE_RDF) );

    // 	meta2.set_property ( kXMP_NS_XMP, "CreatorTool", "new CreatorTool" );
    // 	meta2.set_property ( kXMP_NS_XMP, "Nickname", "new Nickname" );
    // 	meta2.set_property ( kXMP_NS_XMP, "Format", "new Format" );
    // 	DumpXMPObj ( log, meta2, "Create 2nd XMP object with new values" );

    // 	SXMPUtils::ApplyTemplate ( &meta1, meta2, kXMPTemplate_AddNewProperties );
    // 	DumpXMPObj ( log, meta1, "Append 2nd to 1st, keeping old values, external
    // only" );

    // 	meta2.set_property ( kXMP_NS_XMP, "CreatorTool", "newer CreatorTool" );
    // 	meta2.set_property ( kXMP_NS_XMP, "Nickname", "newer Nickname" );
    // 	meta2.set_property ( kXMP_NS_XMP, "Format", "newer Format" );
    // 	SXMPUtils::ApplyTemplate ( &meta1, meta2, kXMPTemplate_AddNewProperties |
    // kXMPTemplate_IncludeInternalProperties ); 	DumpXMPObj ( log, meta1,
    // "Append 2nd to 1st, keeping old values, internal also" );

    // 	meta2.set_property ( kXMP_NS_XMP, "CreatorTool", "newest CreatorTool" );
    // 	meta2.set_property ( kXMP_NS_XMP, "Nickname", "newest Nickname" );
    // 	meta2.set_property ( kXMP_NS_XMP, "Format", "newest Format" );
    // 	SXMPUtils::ApplyTemplate ( &meta1, meta2, kXMPTemplate_AddNewProperties |
    // kXMPTemplate_ReplaceExistingProperties ); 	DumpXMPObj ( log, meta1,
    // "Append 2nd to 1st, replacing old values, external only" );

    // 	meta2.set_property ( kXMP_NS_XMP, "CreatorTool", "final CreatorTool" );
    // 	meta2.set_property ( kXMP_NS_XMP, "Nickname", "final Nickname" );
    // 	meta2.set_property ( kXMP_NS_XMP, "Format", "final Format" );
    // 	SXMPUtils::ApplyTemplate ( &meta1, meta2, kXMPTemplate_AddNewProperties |
    // kXMPTemplate_ReplaceExistingProperties |
    // kXMPTemplate_IncludeInternalProperties ); 	DumpXMPObj ( log, meta1,
    // "Append 2nd to 1st, replacing old values, internal also" );

    // }

    // // --------------------------------------------------------------------------------------------

    // {
    // 	write_major_label("Test DuplicateSubtree" );

    // 	SXMPMeta meta1 ( SIMPLE_RDF, strlen(SIMPLE_RDF) );
    // 	SXMPMeta meta2;

    // 	SXMPUtils::DuplicateSubtree ( meta1, &meta2, NS1, "ArrayOfStructProp" );
    // 	DumpXMPObj ( log, meta2, "DuplicateSubtree to default destination" );

    // 	#if 1	// The underlying old toolkit does not support changing the schema
    // namespace.

    // 		SXMPUtils::DuplicateSubtree ( meta1, &meta2, NS1, "ArrayOfStructProp", NS2,
    // "NewAoS" ); 		DumpXMPObj ( log, meta2, "DuplicateSubtree to different
    // destination" );

    // 		SXMPUtils::DuplicateSubtree ( meta1, &meta1, NS1, "ArrayOfStructProp", NS2,
    // "NewAoS" ); 		DumpXMPObj ( log, meta1, "DuplicateSubtree to different
    // destination in same object" );

    // 	#else

    // 		SXMPUtils::DuplicateSubtree ( meta1, &meta2, NS1, "ArrayOfStructProp", NS1,
    // "NewAoS" ); 		DumpXMPObj ( log, meta2, "DuplicateSubtree to different
    // destination" );

    // 		SXMPUtils::DuplicateSubtree ( meta1, &meta1, NS1, "ArrayOfStructProp", NS1,
    // "NewAoS" ); 		DumpXMPObj ( log, meta1, "DuplicateSubtree to different
    // destination in same object" );

    // 	#endif

    // }

    // // --------------------------------------------------------------------------------------------

    // {
    // 	write_major_label("Test EncodeToBase64 and DecodeFromBase64" );
    // 	fprintf ( log, "\n" );

    // 	unsigned long m;

    // 	#if UseStringPushBack
    // 		#define PushBack(s,c)	s.push_back ( c )
    // 	#else
    // 		#define PushBack(s,c)	s.insert ( s.end(), c );
    // 	#endif

    // 	tmpStr1.erase();
    // 	for ( i = 0; i < 64; i += 4 ) {
    // 		m = (i << 18) + ((i+1) << 12) + ((i+2) << 6) + (i+3);
    // 		PushBack ( tmpStr1, ((char) (m >> 16)) );
    // 		PushBack ( tmpStr1, ((char) ((m >> 8) & 0xFF)) );
    // 		PushBack ( tmpStr1, ((char) (m & 0xFF)) );
    // 	}

    // 	tmpStr2.erase();
    // 	SXMPUtils::EncodeToBase64 ( tmpStr1, &tmpStr2 );
    // 	fprintf ( log, "Encoded sequence (should be A-Za-z0-9+/) : %s\n",
    // tmpStr2.c_str() );

    // 	tmpStr3.erase();
    // 	SXMPUtils::DecodeFromBase64 ( tmpStr2, &tmpStr3 );
    // 	if ( tmpStr1 != tmpStr3 ) fprintf ( log, "** Error in base 64 round trip\n"
    // );

    // }

    // // --------------------------------------------------------------------------------------------

    // write_major_label("XMPCoreCoverage done" );
    // fprintf ( log, "\n" );

    panic!("aborting test for now so we can inspect output");
}
