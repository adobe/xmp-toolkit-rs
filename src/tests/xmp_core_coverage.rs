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

use std::{
    str::FromStr,
    string::{String, ToString},
};

use crate::{
    tests::fixtures::*, xmp_ns, xmp_value::xmp_prop, FromStrOptions, ItemPlacement, IterOptions,
    ToStringOptions, XmpDate, XmpDateTime, XmpError, XmpErrorType, XmpMeta, XmpProperty, XmpTime,
    XmpTimeZone, XmpValue,
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

fn write_minor_label(title: &str) {
    println!();
    println!("// {}", String::from_utf8(vec![b'-'; title.len()]).unwrap());
    println!("// {}", title);
    println!();
}

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

// 	write_minor_label(title );
// 	meta.DumpObject ( DumpToFile, log );

// }	// DumpXMPObj

// // -------------------------------------------------------------------------------------------------

// static void VerifyNewlines ( FILE * log, std::string xmp, const char *
// newline ) {
// 	for ( size_t i = 0; i < xmp.size(); ++i ) {
// 		if ( (xmp[i] == '\x0A') || (xmp[i] == '\x0D') ) {
// 			if ( strncmp ( &xmp[i], newline, strlen(newline) ) != 0 ) {
// 				println!( "** Wrong newline at offset %zd\n", i );
// 			}
// 			if ( strlen(newline) == 2 ) ++i;
// 		}
// 	}
// }

fn check_props_exist(meta: &XmpMeta, props: &[XmpProperty]) {
    for prop in props {
        println!(
            "  {} {} = \"{}\" {:#X}",
            prop.schema_ns, prop.name, prop.value.value, prop.value.options
        );

        if !prop.value.is_schema_node() {
            let value = meta
                .property(&prop.schema_ns, &prop.name)
                .unwrap_or_else(|| panic!("Property {} {} was missing", prop.schema_ns, prop.name));

            assert_eq!(prop.value, value);
        }
    }
}

fn print_props(props: &[XmpProperty]) {
    for prop in props {
        println!(
            "  {} {} = \"{}\" {:#X}",
            prop.schema_ns, prop.name, prop.value.value, prop.value.options
        );
    }
}

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
        let err = XmpMeta::from_str_with_options(
            SIMPLE_RDF,
            FromStrOptions::default().require_xmp_meta(),
        )
        .unwrap_err();

        assert_eq!(
            err,
            XmpError {
                error_type: XmpErrorType::XmpMetaElementMissing,
                debug_message: "x:xmpmeta element not found".to_owned()
            }
        );
    }

    {
        let meta = XmpMeta::from_str(NAMESPACE_RDF).unwrap();

        println!("Parse RDF with multiple nested namespaces = {:#?}", meta);

        assert_eq!(meta.to_string(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"Test:XMPCoreCoverage/kNamespaceRDF\" xmlns:ns1=\"ns:test1/\" xmlns:ns2=\"ns:test2/\" xmlns:ns3=\"ns:test3/\" xmlns:ns4=\"ns:test4/\" xmlns:ns5=\"ns:test5/\" xmlns:ns6=\"ns:test6/\"> <ns1:NestedStructProp rdf:parseType=\"Resource\"> <ns2:Outer rdf:parseType=\"Resource\"> <ns3:Middle rdf:parseType=\"Resource\"> <ns4:Inner rdf:parseType=\"Resource\"> <ns5:Field1>Field1 value</ns5:Field1> <ns6:Field2>Field2 value</ns6:Field2> </ns4:Inner> </ns3:Middle> </ns2:Outer> </ns1:NestedStructProp> </rdf:Description> </rdf:RDF> </x:xmpmeta>");
    }

    {
        let meta = XmpMeta::from_str_with_options(
            XMP_META_RDF,
            FromStrOptions::default().require_xmp_meta(),
        )
        .unwrap();

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

    assert_eq!(
        XmpMeta::from_str_with_options(
            INCONSISTENT_RDF,
            FromStrOptions::default().strict_aliasing()
        )
        .unwrap_err(),
        XmpError {
            error_type: XmpErrorType::BadXmp,
            debug_message: "Mismatch between alias and base nodes".to_owned()
        }
    );

    {
        write_major_label("Test CR and LF in values");

        const VALUE_WITH_CR: &str = "ASCII \r CR";
        const VALUE_WITH_LF: &str = "ASCII \n LF";
        const VALUE_WITH_CRLF: &str = "ASCII \r\n CRLF";

        let mut meta = XmpMeta::from_str(NEWLINE_RDF).unwrap();

        meta.set_property(NS2, "HasCR", &VALUE_WITH_CR.into())
            .unwrap();
        meta.set_property(NS2, "HasLF", &VALUE_WITH_LF.into())
            .unwrap();
        meta.set_property(NS2, "HasCRLF", &VALUE_WITH_CRLF.into())
            .unwrap();

        println!("Parse and reconcile inconsistent aliases = {:#?}", meta);
        assert_eq!(meta.to_string(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"Test:XMPCoreCoverage/kNewlineRDF\" xmlns:ns1=\"ns:test1/\" xmlns:ns2=\"ns:test2/\"> <ns1:HasCR>ASCII &#xA; CR</ns1:HasCR> <ns1:HasLF>ASCII &#xA; LF</ns1:HasLF> <ns1:HasCRLF>ASCII &#xA; CRLF</ns1:HasCRLF> <ns2:HasCR>ASCII &#xD; CR</ns2:HasCR> <ns2:HasLF>ASCII &#xA; LF</ns2:HasLF> <ns2:HasCRLF>ASCII &#xD;&#xA; CRLF</ns2:HasCRLF> </rdf:Description> </rdf:RDF> </x:xmpmeta>");

        assert_eq!(
            meta.property(NS2, "HasCR"),
            Some(XmpValue {
                value: VALUE_WITH_CR.to_owned(),
                options: 0
            })
        );

        assert_eq!(
            meta.property(NS2, "HasLF"),
            Some(XmpValue {
                value: VALUE_WITH_LF.to_owned(),
                options: 0
            })
        );

        assert_eq!(
            meta.property(NS2, "HasCRLF"),
            Some(XmpValue {
                value: VALUE_WITH_CRLF.to_owned(),
                options: 0
            })
        );
    }

    {
        write_major_label("Test serialization with various options");

        let mut meta = XmpMeta::from_str(SIMPLE_RDF).unwrap();

        meta.set_property(NS2, "Another", &"Something in another schema".into())
            .unwrap();
        meta.set_property(NS2, "Yet/pdf:More", &"Yet more in another schema".into())
            .unwrap();

        println!(
            "Parse simple RDF, serialize with various options = {:#?}",
            meta
        );

        assert_eq!(meta.to_string(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"Test:XMPCoreCoverage/kSimpleRDF\" xmlns:ns1=\"ns:test1/\" xmlns:ns2=\"ns:test2/\" xmlns:pdf=\"http://ns.adobe.com/pdf/1.3/\"> <ns1:SimpleProp>Simple value</ns1:SimpleProp> <ns1:ArrayProp> <rdf:Bag> <rdf:li>Item1 value</rdf:li> <rdf:li>Item2 value</rdf:li> </rdf:Bag> </ns1:ArrayProp> <ns1:StructProp rdf:parseType=\"Resource\"> <ns2:Field1>Field1 value</ns2:Field1> <ns2:Field2>Field2 value</ns2:Field2> </ns1:StructProp> <ns1:QualProp rdf:parseType=\"Resource\"> <rdf:value>Prop value</rdf:value> <ns2:Qual>Qual value</ns2:Qual> </ns1:QualProp> <ns1:AltTextProp> <rdf:Alt> <rdf:li xml:lang=\"x-one\">x-one value</rdf:li> <rdf:li xml:lang=\"x-two\">x-two value</rdf:li> </rdf:Alt> </ns1:AltTextProp> <ns1:ArrayOfStructProp> <rdf:Bag> <rdf:li rdf:parseType=\"Resource\"> <ns2:Field1>Item-1</ns2:Field1> <ns2:Field2>Field 1.2 value</ns2:Field2> </rdf:li> <rdf:li rdf:parseType=\"Resource\"> <ns2:Field1>Item-2</ns2:Field1> <ns2:Field2>Field 2.2 value</ns2:Field2> </rdf:li> </rdf:Bag> </ns1:ArrayOfStructProp> <ns2:Another>Something in another schema</ns2:Another> <ns2:Yet rdf:parseType=\"Resource\"> <pdf:More>Yet more in another schema</pdf:More> </ns2:Yet> </rdf:Description> </rdf:RDF> </x:xmpmeta>");

        let meta2 = XmpMeta::from_str(&meta.to_string()).unwrap();

        println!("Reparse default serialization = {:#?}", meta2);
        assert_eq!(meta2.to_string(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"Test:XMPCoreCoverage/kSimpleRDF\" xmlns:ns1=\"ns:test1/\" xmlns:ns2=\"ns:test2/\" xmlns:pdf=\"http://ns.adobe.com/pdf/1.3/\"> <ns1:SimpleProp>Simple value</ns1:SimpleProp> <ns1:ArrayProp> <rdf:Bag> <rdf:li>Item1 value</rdf:li> <rdf:li>Item2 value</rdf:li> </rdf:Bag> </ns1:ArrayProp> <ns1:StructProp rdf:parseType=\"Resource\"> <ns2:Field1>Field1 value</ns2:Field1> <ns2:Field2>Field2 value</ns2:Field2> </ns1:StructProp> <ns1:QualProp rdf:parseType=\"Resource\"> <rdf:value>Prop value</rdf:value> <ns2:Qual>Qual value</ns2:Qual> </ns1:QualProp> <ns1:AltTextProp> <rdf:Alt> <rdf:li xml:lang=\"x-one\">x-one value</rdf:li> <rdf:li xml:lang=\"x-two\">x-two value</rdf:li> </rdf:Alt> </ns1:AltTextProp> <ns1:ArrayOfStructProp> <rdf:Bag> <rdf:li rdf:parseType=\"Resource\"> <ns2:Field1>Item-1</ns2:Field1> <ns2:Field2>Field 1.2 value</ns2:Field2> </rdf:li> <rdf:li rdf:parseType=\"Resource\"> <ns2:Field1>Item-2</ns2:Field1> <ns2:Field2>Field 2.2 value</ns2:Field2> </rdf:li> </rdf:Bag> </ns1:ArrayOfStructProp> <ns2:Another>Something in another schema</ns2:Another> <ns2:Yet rdf:parseType=\"Resource\"> <pdf:More>Yet more in another schema</pdf:More> </ns2:Yet> </rdf:Description> </rdf:RDF> </x:xmpmeta>");

        let m1 = meta
            .to_string_with_options(
                ToStringOptions::default()
                    .omit_packet_wrapper()
                    .use_compact_format(),
            )
            .unwrap();

        println!("Compact RDF, no packet serialize = {}", m1);
        assert_eq!(m1, "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\">\n <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\">\n  <rdf:Description rdf:about=\"Test:XMPCoreCoverage/kSimpleRDF\"\n    xmlns:ns1=\"ns:test1/\"\n    xmlns:ns2=\"ns:test2/\"\n    xmlns:pdf=\"http://ns.adobe.com/pdf/1.3/\"\n   ns1:SimpleProp=\"Simple value\"\n   ns2:Another=\"Something in another schema\">\n   <ns1:ArrayProp>\n    <rdf:Bag>\n     <rdf:li>Item1 value</rdf:li>\n     <rdf:li>Item2 value</rdf:li>\n    </rdf:Bag>\n   </ns1:ArrayProp>\n   <ns1:StructProp\n    ns2:Field1=\"Field1 value\"\n    ns2:Field2=\"Field2 value\"/>\n   <ns1:QualProp rdf:parseType=\"Resource\">\n    <rdf:value>Prop value</rdf:value>\n    <ns2:Qual>Qual value</ns2:Qual>\n   </ns1:QualProp>\n   <ns1:AltTextProp>\n    <rdf:Alt>\n     <rdf:li xml:lang=\"x-one\">x-one value</rdf:li>\n     <rdf:li xml:lang=\"x-two\">x-two value</rdf:li>\n    </rdf:Alt>\n   </ns1:AltTextProp>\n   <ns1:ArrayOfStructProp>\n    <rdf:Bag>\n     <rdf:li\n      ns2:Field1=\"Item-1\"\n      ns2:Field2=\"Field 1.2 value\"/>\n     <rdf:li\n      ns2:Field1=\"Item-2\"\n      ns2:Field2=\"Field 2.2 value\"/>\n    </rdf:Bag>\n   </ns1:ArrayOfStructProp>\n   <ns2:Yet\n    pdf:More=\"Yet more in another schema\"/>\n  </rdf:Description>\n </rdf:RDF>\n</x:xmpmeta>\n");

        let meta3 = XmpMeta::from_str(&m1).unwrap();

        println!("Reparse compact serialization = {:#?}", meta3);
        assert_eq!(meta3.to_string(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"Test:XMPCoreCoverage/kSimpleRDF\" xmlns:ns1=\"ns:test1/\" xmlns:ns2=\"ns:test2/\" xmlns:pdf=\"http://ns.adobe.com/pdf/1.3/\"> <ns1:SimpleProp>Simple value</ns1:SimpleProp> <ns1:ArrayProp> <rdf:Bag> <rdf:li>Item1 value</rdf:li> <rdf:li>Item2 value</rdf:li> </rdf:Bag> </ns1:ArrayProp> <ns1:StructProp rdf:parseType=\"Resource\"> <ns2:Field1>Field1 value</ns2:Field1> <ns2:Field2>Field2 value</ns2:Field2> </ns1:StructProp> <ns1:QualProp rdf:parseType=\"Resource\"> <rdf:value>Prop value</rdf:value> <ns2:Qual>Qual value</ns2:Qual> </ns1:QualProp> <ns1:AltTextProp> <rdf:Alt> <rdf:li xml:lang=\"x-one\">x-one value</rdf:li> <rdf:li xml:lang=\"x-two\">x-two value</rdf:li> </rdf:Alt> </ns1:AltTextProp> <ns1:ArrayOfStructProp> <rdf:Bag> <rdf:li rdf:parseType=\"Resource\"> <ns2:Field1>Item-1</ns2:Field1> <ns2:Field2>Field 1.2 value</ns2:Field2> </rdf:li> <rdf:li rdf:parseType=\"Resource\"> <ns2:Field1>Item-2</ns2:Field1> <ns2:Field2>Field 2.2 value</ns2:Field2> </rdf:li> </rdf:Bag> </ns1:ArrayOfStructProp> <ns2:Another>Something in another schema</ns2:Another> <ns2:Yet rdf:parseType=\"Resource\"> <pdf:More>Yet more in another schema</pdf:More> </ns2:Yet> </rdf:Description> </rdf:RDF> </x:xmpmeta>");

        {
            let mut meta2 = XmpMeta::default();

            meta2
                .set_property(xmp_ns::PDF, "Author", &"PDF Author".into())
                .unwrap();

            println!("Read-only serialize with alias comments = {:#?}", meta2);

            assert_eq!(meta2.to_string(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"\" xmlns:dc=\"http://purl.org/dc/elements/1.1/\"> <dc:creator> <rdf:Seq> <rdf:li>PDF Author</rdf:li> </rdf:Seq> </dc:creator> </rdf:Description> </rdf:RDF> </x:xmpmeta>");

            meta2
                .set_property(xmp_ns::PDF, "Actual", &"PDF Actual".into())
                .unwrap();

            meta2
                .set_property(xmp_ns::XMP, "Actual", &"XMP Actual".into())
                .unwrap();

            println!(
                "Read-only serialize with alias comments (more actuals) = {:#?}",
                meta2
            );

            assert_eq!(meta2.to_string(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"\" xmlns:dc=\"http://purl.org/dc/elements/1.1/\" xmlns:pdf=\"http://ns.adobe.com/pdf/1.3/\" xmlns:xmp=\"http://ns.adobe.com/xap/1.0/\"> <dc:creator> <rdf:Seq> <rdf:li>PDF Author</rdf:li> </rdf:Seq> </dc:creator> <pdf:Actual>PDF Actual</pdf:Actual> <xmp:Actual>XMP Actual</xmp:Actual> </rdf:Description> </rdf:RDF> </x:xmpmeta>");
        }

        let s = meta
            .to_string_with_options(
                ToStringOptions::default()
                    .omit_packet_wrapper()
                    .set_newline("\u{D}".to_owned()),
            )
            .unwrap();

        println!("CR newline serialize = {}", s);
        assert_eq!(s, "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\">\r   <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\">\r      <rdf:Description rdf:about=\"Test:XMPCoreCoverage/kSimpleRDF\"\r            xmlns:ns1=\"ns:test1/\"\r            xmlns:ns2=\"ns:test2/\"\r            xmlns:pdf=\"http://ns.adobe.com/pdf/1.3/\">\r         <ns1:SimpleProp>Simple value</ns1:SimpleProp>\r         <ns1:ArrayProp>\r            <rdf:Bag>\r               <rdf:li>Item1 value</rdf:li>\r               <rdf:li>Item2 value</rdf:li>\r            </rdf:Bag>\r         </ns1:ArrayProp>\r         <ns1:StructProp rdf:parseType=\"Resource\">\r            <ns2:Field1>Field1 value</ns2:Field1>\r            <ns2:Field2>Field2 value</ns2:Field2>\r         </ns1:StructProp>\r         <ns1:QualProp rdf:parseType=\"Resource\">\r            <rdf:value>Prop value</rdf:value>\r            <ns2:Qual>Qual value</ns2:Qual>\r         </ns1:QualProp>\r         <ns1:AltTextProp>\r            <rdf:Alt>\r               <rdf:li xml:lang=\"x-one\">x-one value</rdf:li>\r               <rdf:li xml:lang=\"x-two\">x-two value</rdf:li>\r            </rdf:Alt>\r         </ns1:AltTextProp>\r         <ns1:ArrayOfStructProp>\r            <rdf:Bag>\r               <rdf:li rdf:parseType=\"Resource\">\r                  <ns2:Field1>Item-1</ns2:Field1>\r                  <ns2:Field2>Field 1.2 value</ns2:Field2>\r               </rdf:li>\r               <rdf:li rdf:parseType=\"Resource\">\r                  <ns2:Field1>Item-2</ns2:Field1>\r                  <ns2:Field2>Field 2.2 value</ns2:Field2>\r               </rdf:li>\r            </rdf:Bag>\r         </ns1:ArrayOfStructProp>\r         <ns2:Another>Something in another schema</ns2:Another>\r         <ns2:Yet rdf:parseType=\"Resource\">\r            <pdf:More>Yet more in another schema</pdf:More>\r         </ns2:Yet>\r      </rdf:Description>\r   </rdf:RDF>\r</x:xmpmeta>\r");

        let s = meta
            .to_string_with_options(
                ToStringOptions::default()
                    .omit_packet_wrapper()
                    .set_newline("\r\n".to_owned()),
            )
            .unwrap();

        println!("CRLF newline serialize = {}", s);
        assert_eq!(s, "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\">\r\n   <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\">\r\n      <rdf:Description rdf:about=\"Test:XMPCoreCoverage/kSimpleRDF\"\r\n            xmlns:ns1=\"ns:test1/\"\r\n            xmlns:ns2=\"ns:test2/\"\r\n            xmlns:pdf=\"http://ns.adobe.com/pdf/1.3/\">\r\n         <ns1:SimpleProp>Simple value</ns1:SimpleProp>\r\n         <ns1:ArrayProp>\r\n            <rdf:Bag>\r\n               <rdf:li>Item1 value</rdf:li>\r\n               <rdf:li>Item2 value</rdf:li>\r\n            </rdf:Bag>\r\n         </ns1:ArrayProp>\r\n         <ns1:StructProp rdf:parseType=\"Resource\">\r\n            <ns2:Field1>Field1 value</ns2:Field1>\r\n            <ns2:Field2>Field2 value</ns2:Field2>\r\n         </ns1:StructProp>\r\n         <ns1:QualProp rdf:parseType=\"Resource\">\r\n            <rdf:value>Prop value</rdf:value>\r\n            <ns2:Qual>Qual value</ns2:Qual>\r\n         </ns1:QualProp>\r\n         <ns1:AltTextProp>\r\n            <rdf:Alt>\r\n               <rdf:li xml:lang=\"x-one\">x-one value</rdf:li>\r\n               <rdf:li xml:lang=\"x-two\">x-two value</rdf:li>\r\n            </rdf:Alt>\r\n         </ns1:AltTextProp>\r\n         <ns1:ArrayOfStructProp>\r\n            <rdf:Bag>\r\n               <rdf:li rdf:parseType=\"Resource\">\r\n                  <ns2:Field1>Item-1</ns2:Field1>\r\n                  <ns2:Field2>Field 1.2 value</ns2:Field2>\r\n               </rdf:li>\r\n               <rdf:li rdf:parseType=\"Resource\">\r\n                  <ns2:Field1>Item-2</ns2:Field1>\r\n                  <ns2:Field2>Field 2.2 value</ns2:Field2>\r\n               </rdf:li>\r\n            </rdf:Bag>\r\n         </ns1:ArrayOfStructProp>\r\n         <ns2:Another>Something in another schema</ns2:Another>\r\n         <ns2:Yet rdf:parseType=\"Resource\">\r\n            <pdf:More>Yet more in another schema</pdf:More>\r\n         </ns2:Yet>\r\n      </rdf:Description>\r\n   </rdf:RDF>\r\n</x:xmpmeta>\r\n");

        let s = meta
            .to_string_with_options(
                ToStringOptions::default()
                    .omit_packet_wrapper()
                    .set_newline("<->".to_owned()),
            )
            .unwrap();

        println!("Alternate newline serialize = {}", s);
        assert_eq!(s, "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"><->   <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"><->      <rdf:Description rdf:about=\"Test:XMPCoreCoverage/kSimpleRDF\"<->            xmlns:ns1=\"ns:test1/\"<->            xmlns:ns2=\"ns:test2/\"<->            xmlns:pdf=\"http://ns.adobe.com/pdf/1.3/\"><->         <ns1:SimpleProp>Simple value</ns1:SimpleProp><->         <ns1:ArrayProp><->            <rdf:Bag><->               <rdf:li>Item1 value</rdf:li><->               <rdf:li>Item2 value</rdf:li><->            </rdf:Bag><->         </ns1:ArrayProp><->         <ns1:StructProp rdf:parseType=\"Resource\"><->            <ns2:Field1>Field1 value</ns2:Field1><->            <ns2:Field2>Field2 value</ns2:Field2><->         </ns1:StructProp><->         <ns1:QualProp rdf:parseType=\"Resource\"><->            <rdf:value>Prop value</rdf:value><->            <ns2:Qual>Qual value</ns2:Qual><->         </ns1:QualProp><->         <ns1:AltTextProp><->            <rdf:Alt><->               <rdf:li xml:lang=\"x-one\">x-one value</rdf:li><->               <rdf:li xml:lang=\"x-two\">x-two value</rdf:li><->            </rdf:Alt><->         </ns1:AltTextProp><->         <ns1:ArrayOfStructProp><->            <rdf:Bag><->               <rdf:li rdf:parseType=\"Resource\"><->                  <ns2:Field1>Item-1</ns2:Field1><->                  <ns2:Field2>Field 1.2 value</ns2:Field2><->               </rdf:li><->               <rdf:li rdf:parseType=\"Resource\"><->                  <ns2:Field1>Item-2</ns2:Field1><->                  <ns2:Field2>Field 2.2 value</ns2:Field2><->               </rdf:li><->            </rdf:Bag><->         </ns1:ArrayOfStructProp><->         <ns2:Another>Something in another schema</ns2:Another><->         <ns2:Yet rdf:parseType=\"Resource\"><->            <pdf:More>Yet more in another schema</pdf:More><->         </ns2:Yet><->      </rdf:Description><->   </rdf:RDF><-></x:xmpmeta><->");

        let s = meta
            .to_string_with_options(
                ToStringOptions::default()
                    .omit_packet_wrapper()
                    .set_indent_string("\t".to_owned()),
            )
            .unwrap();

        println!("Alternate indent serialize = {}", s);
        assert_eq!(s, "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\">\n\t<rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\">\n\t\t<rdf:Description rdf:about=\"Test:XMPCoreCoverage/kSimpleRDF\"\n\t\t\t\txmlns:ns1=\"ns:test1/\"\n\t\t\t\txmlns:ns2=\"ns:test2/\"\n\t\t\t\txmlns:pdf=\"http://ns.adobe.com/pdf/1.3/\">\n\t\t\t<ns1:SimpleProp>Simple value</ns1:SimpleProp>\n\t\t\t<ns1:ArrayProp>\n\t\t\t\t<rdf:Bag>\n\t\t\t\t\t<rdf:li>Item1 value</rdf:li>\n\t\t\t\t\t<rdf:li>Item2 value</rdf:li>\n\t\t\t\t</rdf:Bag>\n\t\t\t</ns1:ArrayProp>\n\t\t\t<ns1:StructProp rdf:parseType=\"Resource\">\n\t\t\t\t<ns2:Field1>Field1 value</ns2:Field1>\n\t\t\t\t<ns2:Field2>Field2 value</ns2:Field2>\n\t\t\t</ns1:StructProp>\n\t\t\t<ns1:QualProp rdf:parseType=\"Resource\">\n\t\t\t\t<rdf:value>Prop value</rdf:value>\n\t\t\t\t<ns2:Qual>Qual value</ns2:Qual>\n\t\t\t</ns1:QualProp>\n\t\t\t<ns1:AltTextProp>\n\t\t\t\t<rdf:Alt>\n\t\t\t\t\t<rdf:li xml:lang=\"x-one\">x-one value</rdf:li>\n\t\t\t\t\t<rdf:li xml:lang=\"x-two\">x-two value</rdf:li>\n\t\t\t\t</rdf:Alt>\n\t\t\t</ns1:AltTextProp>\n\t\t\t<ns1:ArrayOfStructProp>\n\t\t\t\t<rdf:Bag>\n\t\t\t\t\t<rdf:li rdf:parseType=\"Resource\">\n\t\t\t\t\t\t<ns2:Field1>Item-1</ns2:Field1>\n\t\t\t\t\t\t<ns2:Field2>Field 1.2 value</ns2:Field2>\n\t\t\t\t\t</rdf:li>\n\t\t\t\t\t<rdf:li rdf:parseType=\"Resource\">\n\t\t\t\t\t\t<ns2:Field1>Item-2</ns2:Field1>\n\t\t\t\t\t\t<ns2:Field2>Field 2.2 value</ns2:Field2>\n\t\t\t\t\t</rdf:li>\n\t\t\t\t</rdf:Bag>\n\t\t\t</ns1:ArrayOfStructProp>\n\t\t\t<ns2:Another>Something in another schema</ns2:Another>\n\t\t\t<ns2:Yet rdf:parseType=\"Resource\">\n\t\t\t\t<pdf:More>Yet more in another schema</pdf:More>\n\t\t\t</ns2:Yet>\n\t\t</rdf:Description>\n\t</rdf:RDF>\n</x:xmpmeta>\n");

        let s = meta
            .to_string_with_options(ToStringOptions::default().set_padding(10))
            .unwrap();

        println!("Small padding serialize = {}", s);
        assert_eq!(s, "<?xpacket begin=\"\u{feff}\" id=\"W5M0MpCehiHzreSzNTczkc9d\"?>\n<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\">\n   <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\">\n      <rdf:Description rdf:about=\"Test:XMPCoreCoverage/kSimpleRDF\"\n            xmlns:ns1=\"ns:test1/\"\n            xmlns:ns2=\"ns:test2/\"\n            xmlns:pdf=\"http://ns.adobe.com/pdf/1.3/\">\n         <ns1:SimpleProp>Simple value</ns1:SimpleProp>\n         <ns1:ArrayProp>\n            <rdf:Bag>\n               <rdf:li>Item1 value</rdf:li>\n               <rdf:li>Item2 value</rdf:li>\n            </rdf:Bag>\n         </ns1:ArrayProp>\n         <ns1:StructProp rdf:parseType=\"Resource\">\n            <ns2:Field1>Field1 value</ns2:Field1>\n            <ns2:Field2>Field2 value</ns2:Field2>\n         </ns1:StructProp>\n         <ns1:QualProp rdf:parseType=\"Resource\">\n            <rdf:value>Prop value</rdf:value>\n            <ns2:Qual>Qual value</ns2:Qual>\n         </ns1:QualProp>\n         <ns1:AltTextProp>\n            <rdf:Alt>\n               <rdf:li xml:lang=\"x-one\">x-one value</rdf:li>\n               <rdf:li xml:lang=\"x-two\">x-two value</rdf:li>\n            </rdf:Alt>\n         </ns1:AltTextProp>\n         <ns1:ArrayOfStructProp>\n            <rdf:Bag>\n               <rdf:li rdf:parseType=\"Resource\">\n                  <ns2:Field1>Item-1</ns2:Field1>\n                  <ns2:Field2>Field 1.2 value</ns2:Field2>\n               </rdf:li>\n               <rdf:li rdf:parseType=\"Resource\">\n                  <ns2:Field1>Item-2</ns2:Field1>\n                  <ns2:Field2>Field 2.2 value</ns2:Field2>\n               </rdf:li>\n            </rdf:Bag>\n         </ns1:ArrayOfStructProp>\n         <ns2:Another>Something in another schema</ns2:Another>\n         <ns2:Yet rdf:parseType=\"Resource\">\n            <pdf:More>Yet more in another schema</pdf:More>\n         </ns2:Yet>\n      </rdf:Description>\n   </rdf:RDF>\n</x:xmpmeta>\n         \n<?xpacket end=\"w\"?>");

        let s1 = meta
            .to_string_with_options(ToStringOptions::default())
            .unwrap();
        let s2 = meta
            .to_string_with_options(ToStringOptions::default().include_thumbnail_pad())
            .unwrap();
        assert_eq!(s2.len() - s1.len(), 10000);

        let s1 = meta
            .to_string_with_options(ToStringOptions::default().read_only_packet())
            .unwrap();
        assert_eq!(s1.len(), 1914);

        let s1 = meta
            .to_string_with_options(
                ToStringOptions::default()
                    .exact_packet_length()
                    .set_padding((s1.len() + 1234) as u32),
            )
            .unwrap();
        assert_eq!(s1.len(), 3148);

        let s1 = meta
            .to_string_with_options(
                ToStringOptions::default()
                    .exact_packet_length()
                    .set_padding(1914),
            )
            .unwrap();
        assert_eq!(s1.len(), 1914);

        assert_eq!(
            meta.to_string_with_options(
                ToStringOptions::default()
                    .exact_packet_length()
                    .set_padding(1913),
            )
            .unwrap_err(),
            XmpError {
                error_type: XmpErrorType::BadSerialize,
                debug_message: "Can't fit into specified packet size".to_owned()
            }
        );
    }

    //-------------------------------------------------------------------------

    {
        write_major_label("Test iteration methods");

        let mut meta = XmpMeta::from_str(RDF_COVERAGE).unwrap();

        meta.set_property(NS2, "Prop", &"Prop value".into())
            .unwrap();

        meta.set_property(NS2, "Bag", &(XmpValue::from("").set_is_array(true)))
            .unwrap();

        meta.set_array_item(
            NS2,
            "Bag",
            ItemPlacement::ReplaceItemAtIndex(1),
            &"BagItem 2".into(),
        )
        .unwrap();

        meta.set_array_item(
            NS2,
            "Bag",
            ItemPlacement::InsertBeforeIndex(1),
            &"BagItem 1".into(),
        )
        .unwrap();

        meta.set_array_item(
            NS2,
            "Bag",
            ItemPlacement::InsertAfterIndex(2),
            &"BagItem 3".into(),
        )
        .unwrap();

        println!(
            "Parse \"coverage\" RDF, add Bag items out of order = {:#?}",
            meta
        );

        assert_eq!(meta.to_string(), "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"Test:XMPCoreCoverage/kRDFCoverage\" xmlns:ns1=\"ns:test1/\" xmlns:ns2=\"ns:test2/\"> <ns1:SimpleProp1>Simple1 value</ns1:SimpleProp1> <ns1:SimpleProp2 xml:lang=\"x-default\">Simple2 value</ns1:SimpleProp2> <ns1:ArrayProp1> <rdf:Bag> <rdf:li>Item1.1 value</rdf:li> <rdf:li>Item1.2 value</rdf:li> </rdf:Bag> </ns1:ArrayProp1> <ns1:ArrayProp2> <rdf:Alt> <rdf:li xml:lang=\"x-one\">Item2.1 value</rdf:li> <rdf:li xml:lang=\"x-two\">Item2.2 value</rdf:li> </rdf:Alt> </ns1:ArrayProp2> <ns1:ArrayProp3> <rdf:Alt> <rdf:li xml:lang=\"x-one\">Item3.1 value</rdf:li> <rdf:li>Item3.2 value</rdf:li> </rdf:Alt> </ns1:ArrayProp3> <ns1:ArrayProp4> <rdf:Alt> <rdf:li>Item4.1 value</rdf:li> <rdf:li xml:lang=\"x-two\">Item4.2 value</rdf:li> </rdf:Alt> </ns1:ArrayProp4> <ns1:ArrayProp5> <rdf:Alt> <rdf:li xml:lang=\"x-xxx\">Item5.1 value</rdf:li> <rdf:li xml:lang=\"x-xxx\">Item5.2 value</rdf:li> </rdf:Alt> </ns1:ArrayProp5> <ns1:StructProp rdf:parseType=\"Resource\"> <ns2:Field1>Field1 value</ns2:Field1> <ns2:Field2>Field2 value</ns2:Field2> </ns1:StructProp> <ns1:QualProp1 rdf:parseType=\"Resource\"> <rdf:value>Prop value</rdf:value> <ns2:Qual>Qual value</ns2:Qual> </ns1:QualProp1> <ns1:QualProp2 xml:lang=\"x-default\" rdf:parseType=\"Resource\"> <rdf:value>Prop value</rdf:value> <ns2:Qual>Qual value</ns2:Qual> </ns1:QualProp2> <ns1:QualProp3 xml:lang=\"x-default\" rdf:parseType=\"Resource\"> <rdf:value>Prop value</rdf:value> <ns2:Qual>Qual value</ns2:Qual> </ns1:QualProp3> <ns1:QualProp4 xml:lang=\"x-default\" rdf:parseType=\"Resource\"> <ns2:Field1>Field1 value</ns2:Field1> <ns2:Field2>Field2 value</ns2:Field2> </ns1:QualProp4> <ns1:QualProp5 xml:lang=\"x-default\"> <rdf:Bag> <rdf:li>Item1.1 value</rdf:li> <rdf:li>Item1.2 value</rdf:li> </rdf:Bag> </ns1:QualProp5> <ns2:NestedStructProp rdf:parseType=\"Resource\"> <ns1:Outer rdf:parseType=\"Resource\"> <ns1:Middle rdf:parseType=\"Resource\"> <ns1:Inner rdf:parseType=\"Resource\"> <ns1:Field1>Field1 value</ns1:Field1> <ns2:Field2>Field2 value</ns2:Field2> </ns1:Inner> </ns1:Middle> </ns1:Outer> </ns2:NestedStructProp> <ns2:Prop>Prop value</ns2:Prop> <ns2:Bag> <rdf:Bag> <rdf:li>BagItem 1</rdf:li> <rdf:li>BagItem 2</rdf:li> <rdf:li>BagItem 3</rdf:li> </rdf:Bag> </ns2:Bag> </rdf:Description> </rdf:RDF> </x:xmpmeta>");

        {
            write_minor_label("Default iteration");

            let props: Vec<XmpProperty> = meta.iter(IterOptions::default()).collect();
            check_props_exist(&meta, &props);

            assert_eq!(
                props[0..5],
                [
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 2147483648
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:SimpleProp1".to_owned(),
                        value: XmpValue {
                            value: "Simple1 value".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:SimpleProp2".to_owned(),
                        value: XmpValue {
                            value: "Simple2 value".to_owned(),
                            options: 80
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:SimpleProp2/?xml:lang".to_owned(),
                        value: XmpValue {
                            value: "x-default".to_owned(),
                            options: 32
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:ArrayProp1".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 512
                        }
                    }
                ]
            );

            assert_eq!(props.len(), 56);
        }

        {
            write_minor_label("Iterate omitting qualifiers");

            let props: Vec<XmpProperty> = meta
                .iter(IterOptions::default().omit_qualifiers())
                .collect();

            check_props_exist(&meta, &props);

            assert_eq!(
                props[0..5],
                [
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 2147483648
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:SimpleProp1".to_owned(),
                        value: XmpValue {
                            value: "Simple1 value".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:SimpleProp2".to_owned(),
                        value: XmpValue {
                            value: "Simple2 value".to_owned(),
                            options: 80
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:ArrayProp1".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 512
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:ArrayProp1[1]".to_owned(),
                        value: XmpValue {
                            value: "Item1.1 value".to_owned(),
                            options: 0
                        }
                    },
                ]
            );

            assert_eq!(props.len(), 42);
        }

        {
            write_minor_label("Iterate with just leaf names");

            let props: Vec<XmpProperty> =
                meta.iter(IterOptions::default().leaf_name_only()).collect();

            print_props(&props);

            assert_eq!(
                props[0..5],
                [
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 2147483648
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:SimpleProp1".to_owned(),
                        value: XmpValue {
                            value: "Simple1 value".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:SimpleProp2".to_owned(),
                        value: XmpValue {
                            value: "Simple2 value".to_owned(),
                            options: 80
                        }
                    },
                    XmpProperty {
                        schema_ns: "http://www.w3.org/XML/1998/namespace".to_owned(),
                        name: "xml:lang".to_owned(),
                        value: XmpValue {
                            value: "x-default".to_owned(),
                            options: 32
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:ArrayProp1".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 512
                        }
                    }
                ]
            );

            assert_eq!(props.len(), 56);
        }

        {
            write_minor_label("Iterate just the leaf nodes");

            let props: Vec<XmpProperty> = meta
                .iter(IterOptions::default().leaf_nodes_only())
                .collect();

            check_props_exist(&meta, &props);

            assert_eq!(
                props[0..5],
                [
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:SimpleProp1".to_owned(),
                        value: XmpValue {
                            value: "Simple1 value".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:SimpleProp2".to_owned(),
                        value: XmpValue {
                            value: "Simple2 value".to_owned(),
                            options: 80
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:SimpleProp2/?xml:lang".to_owned(),
                        value: XmpValue {
                            value: "x-default".to_owned(),
                            options: 32
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:ArrayProp1[1]".to_owned(),
                        value: XmpValue {
                            value: "Item1.1 value".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:ArrayProp1[2]".to_owned(),
                        value: XmpValue {
                            value: "Item1.2 value".to_owned(),
                            options: 0
                        }
                    }
                ]
            );

            assert_eq!(props.len(), 39);
        }

        {
            write_minor_label("Iterate just the schema nodes");

            let props: Vec<XmpProperty> = meta
                .iter(IterOptions::default().immediate_children_only())
                .collect();

            check_props_exist(&meta, &props);

            assert_eq!(
                props,
                [
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 2147483648
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 2147483648
                        }
                    },
                ]
            );
        }

        {
            write_minor_label("Iterate the ns2: namespace");

            let props: Vec<XmpProperty> =
                meta.iter(IterOptions::default().schema_ns(NS2)).collect();

            check_props_exist(&meta, &props);

            assert_eq!(
                props[0..5],
                [
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 2147483648
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:NestedStructProp".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 256
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:NestedStructProp/ns1:Outer".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 256
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:NestedStructProp/ns1:Outer/ns1:Middle".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 256
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:NestedStructProp/ns1:Outer/ns1:Middle/ns1:Inner".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 256
                        }
                    },
                ]
            );

            assert_eq!(props.len(), 12);
        }

        {
            write_minor_label("Start at ns2:Bag");

            let props: Vec<XmpProperty> = meta
                .iter(IterOptions::default().property(NS2, "Bag"))
                .collect();

            check_props_exist(&meta, &props);

            assert_eq!(
                props,
                [
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:Bag".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 512
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:Bag[1]".to_owned(),
                        value: XmpValue {
                            value: "BagItem 1".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:Bag[2]".to_owned(),
                        value: XmpValue {
                            value: "BagItem 2".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:Bag[3]".to_owned(),
                        value: XmpValue {
                            value: "BagItem 3".to_owned(),
                            options: 0
                        }
                    }
                ]
            );
        }

        {
            write_minor_label("Start at ns2:NestedStructProp/ns1:Outer");

            let props: Vec<XmpProperty> = meta
                .iter(IterOptions::default().property(NS2, "NestedStructProp/ns1:Outer"))
                .collect();

            check_props_exist(&meta, &props);

            assert_eq!(
                props,
                [
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:NestedStructProp/ns1:Outer".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 256
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:NestedStructProp/ns1:Outer/ns1:Middle".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 256
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:NestedStructProp/ns1:Outer/ns1:Middle/ns1:Inner".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 256
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:NestedStructProp/ns1:Outer/ns1:Middle/ns1:Inner/ns1:Field1"
                            .to_owned(),
                        value: XmpValue {
                            value: "Field1 value".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:NestedStructProp/ns1:Outer/ns1:Middle/ns1:Inner/ns2:Field2"
                            .to_owned(),
                        value: XmpValue {
                            value: "Field2 value".to_owned(),
                            options: 0
                        }
                    }
                ]
            );
        }

        {
            write_minor_label("Iterate an empty namespace");

            let mut prop_iter = meta.iter(IterOptions::default().schema_ns("ns:empty/"));
            assert!(prop_iter.next().is_none());
        }

        {
            write_minor_label("Iterate the top of the ns2: namespace with just leaf names");

            let props: Vec<XmpProperty> = meta
                .iter(
                    IterOptions::default()
                        .schema_ns(NS2)
                        .immediate_children_only()
                        .leaf_name_only(),
                )
                .collect();

            assert_eq!(
                props,
                [
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:NestedStructProp".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 256
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:Prop".to_owned(),
                        value: XmpValue {
                            value: "Prop value".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:Bag".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 512
                        }
                    }
                ]
            );
        }

        {
            write_minor_label("Iterate the top of the ns2: namespace visiting just leaf nodes");

            let props: Vec<XmpProperty> = meta
                .iter(
                    IterOptions::default()
                        .schema_ns(NS2)
                        .immediate_children_only(),
                )
                .collect();

            check_props_exist(&meta, &props);

            assert_eq!(
                props,
                [
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:NestedStructProp".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 256
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:Prop".to_owned(),
                        value: XmpValue {
                            value: "Prop value".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:Bag".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 512
                        }
                    }
                ]
            );
        }

        {
            write_minor_label("Iterate just the children of ns2:Bag");

            let props: Vec<XmpProperty> = meta
                .iter(
                    IterOptions::default()
                        .property(NS2, "Bag")
                        .immediate_children_only(),
                )
                .collect();

            check_props_exist(&meta, &props);

            assert_eq!(
                props,
                [
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:Bag[1]".to_owned(),
                        value: XmpValue {
                            value: "BagItem 1".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:Bag[2]".to_owned(),
                        value: XmpValue {
                            value: "BagItem 2".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:Bag[3]".to_owned(),
                        value: XmpValue {
                            value: "BagItem 3".to_owned(),
                            options: 0
                        }
                    }
                ]
            );
        }

        {
            write_minor_label(
                "Iterate just the
        children of ns2:Bag with just leaf names",
            );

            let props: Vec<XmpProperty> = meta
                .iter(
                    IterOptions::default()
                        .property(NS2, "Bag")
                        .immediate_children_only()
                        .leaf_name_only(),
                )
                .collect();

            assert_eq!(
                props,
                [
                    XmpProperty {
                        schema_ns: "".to_owned(),
                        name: "[1]".to_owned(),
                        value: XmpValue {
                            value: "BagItem 1".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "".to_owned(),
                        name: "[2]".to_owned(),
                        value: XmpValue {
                            value: "BagItem 2".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "".to_owned(),
                        name: "[3]".to_owned(),
                        value: XmpValue {
                            value: "BagItem 3".to_owned(),
                            options: 0
                        }
                    }
                ]
            );
        }

        {
            write_minor_label(
                "Iterate just the children of ns2:NestedStructProp/ns1:Outer/ns1:Middle",
            );

            let props: Vec<XmpProperty> = meta
                .iter(
                    IterOptions::default()
                        .property(NS2, "NestedStructProp/ns1:Outer/ns1:Middle")
                        .immediate_children_only(),
                )
                .collect();

            assert_eq!(
                props,
                [XmpProperty {
                    schema_ns: "ns:test2/".to_owned(),
                    name: "ns2:NestedStructProp/ns1:Outer/ns1:Middle/ns1:Inner".to_owned(),
                    value: XmpValue {
                        value: "".to_owned(),
                        options: 256
                    }
                }]
            );
        }

        {
            write_minor_label("Skip children of ArrayProp2, and siblings after StructProp");

            let mut prop_iter = meta.iter(IterOptions::default());
            let mut filtered_props: Vec<XmpProperty> = vec![];

            while let Some(prop) = prop_iter.next() {
                println!(
                    "  {} {} = \"{}\" 0x{:#X}",
                    prop.schema_ns, prop.name, prop.value.value, prop.value.options
                );

                if !prop.value.is_schema_node() {
                    let value = meta
                        .property(&prop.schema_ns, &prop.name)
                        .unwrap_or_else(|| {
                            panic!("Property {} {} was missing", prop.schema_ns, prop.name)
                        });

                    assert_eq!(prop.value, value);
                }

                if prop.name == "ns1:ArrayProp2" {
                    println!("skipping subtree of ns1:ArrayProp2");
                    prop_iter.skip_subtree();
                }
                if prop.name == "ns1:StructProp" {
                    println!("skipping subtree of ns1:StructProp");
                    prop_iter.skip_siblings();
                }

                filtered_props.push(prop);
            }

            assert_eq!(
                filtered_props,
                [
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 2147483648
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:SimpleProp1".to_owned(),
                        value: XmpValue {
                            value: "Simple1 value".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:SimpleProp2".to_owned(),
                        value: XmpValue {
                            value: "Simple2 value".to_owned(),
                            options: 80
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:SimpleProp2/?xml:lang".to_owned(),
                        value: XmpValue {
                            value: "x-default".to_owned(),
                            options: 32
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:ArrayProp1".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 512
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:ArrayProp1[1]".to_owned(),
                        value: XmpValue {
                            value: "Item1.1 value".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:ArrayProp1[2]".to_owned(),
                        value: XmpValue {
                            value: "Item1.2 value".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:ArrayProp2".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 7680
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:ArrayProp3".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 3584
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:ArrayProp3[1]".to_owned(),
                        value: XmpValue {
                            value: "Item3.1 value".to_owned(),
                            options: 80
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:ArrayProp3[1]/?xml:lang".to_owned(),
                        value: XmpValue {
                            value: "x-one".to_owned(),
                            options: 32
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:ArrayProp3[2]".to_owned(),
                        value: XmpValue {
                            value: "Item3.2 value".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:ArrayProp4".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 3584
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:ArrayProp4[1]".to_owned(),
                        value: XmpValue {
                            value: "Item4.1 value".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:ArrayProp4[2]".to_owned(),
                        value: XmpValue {
                            value: "Item4.2 value".to_owned(),
                            options: 80
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:ArrayProp4[2]/?xml:lang".to_owned(),
                        value: XmpValue {
                            value: "x-two".to_owned(),
                            options: 32
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:ArrayProp5".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 7680
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:ArrayProp5[1]".to_owned(),
                        value: XmpValue {
                            value: "Item5.1 value".to_owned(),
                            options: 80
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:ArrayProp5[1]/?xml:lang".to_owned(),
                        value: XmpValue {
                            value: "x-xxx".to_owned(),
                            options: 32
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:ArrayProp5[2]".to_owned(),
                        value: XmpValue {
                            value: "Item5.2 value".to_owned(),
                            options: 80
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:ArrayProp5[2]/?xml:lang".to_owned(),
                        value: XmpValue {
                            value: "x-xxx".to_owned(),
                            options: 32
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test1/".to_owned(),
                        name: "ns1:StructProp".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 256
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 2147483648
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:NestedStructProp".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 256
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:NestedStructProp/ns1:Outer".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 256
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:NestedStructProp/ns1:Outer/ns1:Middle".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 256
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:NestedStructProp/ns1:Outer/ns1:Middle/ns1:Inner".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 256
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:NestedStructProp/ns1:Outer/ns1:Middle/ns1:Inner/ns1:Field1"
                            .to_owned(),
                        value: XmpValue {
                            value: "Field1 value".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:NestedStructProp/ns1:Outer/ns1:Middle/ns1:Inner/ns2:Field2"
                            .to_owned(),
                        value: XmpValue {
                            value: "Field2 value".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:Prop".to_owned(),
                        value: XmpValue {
                            value: "Prop value".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:Bag".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 512
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:Bag[1]".to_owned(),
                        value: XmpValue {
                            value: "BagItem 1".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:Bag[2]".to_owned(),
                        value: XmpValue {
                            value: "BagItem 2".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "ns:test2/".to_owned(),
                        name: "ns2:Bag[3]".to_owned(),
                        value: XmpValue {
                            value: "BagItem 3".to_owned(),
                            options: 0
                        }
                    }
                ]
            );
        }

        {
            let mut meta = XmpMeta::default();

            meta.set_property(xmp_ns::PDF, "Author", &"PDF Author".into())
                .unwrap();
            meta.set_property(xmp_ns::PDF, "PDFProp", &"PDF Prop".into())
                .unwrap();
            meta.set_property(xmp_ns::XMP, "XMPProp", &"XMP Prop".into())
                .unwrap();
            meta.set_property(xmp_ns::DC, "DCProp", &"DC Prop".into())
                .unwrap();

            write_minor_label("Iterate without showing aliases");

            let props: Vec<XmpProperty> = meta
                .iter(IterOptions::default())
                .filter(|prop| !(prop.value.is_schema_node() || prop.value.has_aliases()))
                .collect();

            check_props_exist(&meta, &props);

            assert_eq!(
                props,
                [
                    XmpProperty {
                        schema_ns: "http://purl.org/dc/elements/1.1/".to_owned(),
                        name: "dc:creator".to_owned(),
                        value: XmpValue {
                            value: "".to_owned(),
                            options: 1536
                        }
                    },
                    XmpProperty {
                        schema_ns: "http://purl.org/dc/elements/1.1/".to_owned(),
                        name: "dc:creator[1]".to_owned(),
                        value: XmpValue {
                            value: "PDF Author".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "http://purl.org/dc/elements/1.1/".to_owned(),
                        name: "dc:DCProp".to_owned(),
                        value: XmpValue {
                            value: "DC Prop".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "http://ns.adobe.com/pdf/1.3/".to_owned(),
                        name: "pdf:PDFProp".to_owned(),
                        value: XmpValue {
                            value: "PDF Prop".to_owned(),
                            options: 0
                        }
                    },
                    XmpProperty {
                        schema_ns: "http://ns.adobe.com/xap/1.0/".to_owned(),
                        name: "xmp:XMPProp".to_owned(),
                        value: XmpValue {
                            value: "XMP Prop".to_owned(),
                            options: 0
                        }
                    }
                ]
            );
        }
    }

    //-------------------------------------------------------------------------

    {
        write_major_label("Test XPath composition utilities");

        let mut meta = XmpMeta::from_str(SIMPLE_RDF).unwrap();
        println!("Parse simple RDF = {:#?}", meta);

        let path = XmpMeta::compose_array_item_path(NS1, "ArrayProp", 2).unwrap();
        println!("compose_array_item_path ns1:ArrayProp[2] : {}", path);

        assert_eq!(path, "ArrayProp[2]");
        meta.set_property(NS1, &path, &"new ns1:ArrayProp[2] value".into())
            .unwrap();
        println!();

        let path = XmpMeta::compose_struct_field_path(NS1, "StructProp", NS2, "Field3").unwrap();
        println!(
            "compose_struct_field_path ns1:StructProp/ns2:Field3 : {}",
            path
        );

        assert_eq!(path, "StructProp/ns2:Field3");
        meta.set_property(NS1, &path, &"new ns1:StructProp/ns2:Field3 value".into())
            .unwrap();
        println!();

        let path = XmpMeta::compose_qualifier_path(NS1, "QualProp", NS2, "Qual").unwrap();
        println!("compose_qualifier_path ns1:QualProp/?ns2:Qual : {}", path);

        assert_eq!(path, "QualProp/?ns2:Qual");
        meta.set_property(NS1, &path, &"new ns1:QualProp/? ns2:Qual value".into())
            .unwrap();
        println!();

        let path =
            XmpMeta::compose_qualifier_path(NS1, "AltTextProp", xmp_ns::XML, "lang").unwrap();
        println!(
            "compose_qualifier_path ns1:AltTextProp/?xml:lang : {}",
            path
        );

        assert_eq!(path, "AltTextProp/?xml:lang");
        meta.set_property(NS1, &path, &"new ns1:AltTextProp/?xml:lang value".into())
            .unwrap();
        println!();

        let path = XmpMeta::compose_lang_selector(NS1, "AltTextProp", "x-two").unwrap();
        println!("compose_lang_selector ns1:AltTextProp['x-two'] : {}", path);

        assert_eq!(path, "AltTextProp[?xml:lang=\"x-two\"]");
        meta.set_property(NS1, &path, &"new ns1:AltTextProp['x-two'] value".into())
            .unwrap();
        println!();

        println!("Check field selector usage");

        let value = meta
            .property(NS1, "ArrayOfStructProp[ns2:Field1='Item-2']")
            .unwrap();
        println!(
            "property ArrayOfStructProp[ns2:Field1='Item-2'] : \"{}\", {:#X}",
            value.value, value.options
        );

        assert_eq!(
            value,
            XmpValue {
                value: "".to_owned(),
                options: 256
            }
        );

        let value = meta
            .property(NS1, "ArrayOfStructProp[ns2:Field1='Item-2']/ns2:Field2")
            .unwrap();
        println!(
            "property ArrayOfStructProp[ns2:Field1='Item-2']/ns2:Field2 : \"{}\", {:#X}",
            value.value, value.options
        );

        assert_eq!(
            value,
            XmpValue {
                value: "Field 2.2 value".to_owned(),
                options: 0
            }
        );

        let path = XmpMeta::compose_field_selector(
            NS1,
            "ArrayOfStructProp",
            NS2,
            "Field1",
            Some("Item-2"),
        )
        .unwrap();
        println!(
            "compose_field_selector ns1:ArrayOfStructProp[ns2:Field1=Item-2] : {}",
            path
        );

        let path = XmpMeta::compose_struct_field_path(NS1, &path, NS2, "Field2").unwrap();
        println!(
            "compose_struct_field_path ns1:ArrayOfStructProp[ns2:Field1=Item-2]/ns2:Field2 : {}",
            path
        );

        meta.set_property(
            NS1,
            &path,
            &"new ns1:ArrayOfStructProp[ns2:Field1=Item-2]/ns2:Field2 value".into(),
        )
        .unwrap();

        println!();
        println!("Modified simple RDF = {:#?}", meta);

        assert_eq!(meta.to_string(),  "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"XMP Core 6.0.0\"> <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"> <rdf:Description rdf:about=\"Test:XMPCoreCoverage/kSimpleRDF\" xmlns:ns1=\"ns:test1/\" xmlns:ns2=\"ns:test2/\"> <ns1:SimpleProp>Simple value</ns1:SimpleProp> <ns1:ArrayProp> <rdf:Bag> <rdf:li>Item1 value</rdf:li> <rdf:li>new ns1:ArrayProp[2] value</rdf:li> </rdf:Bag> </ns1:ArrayProp> <ns1:StructProp rdf:parseType=\"Resource\"> <ns2:Field1>Field1 value</ns2:Field1> <ns2:Field2>Field2 value</ns2:Field2> <ns2:Field3>new ns1:StructProp/ns2:Field3 value</ns2:Field3> </ns1:StructProp> <ns1:QualProp rdf:parseType=\"Resource\"> <rdf:value>Prop value</rdf:value> <ns2:Qual>new ns1:QualProp/? ns2:Qual value</ns2:Qual> </ns1:QualProp> <ns1:AltTextProp xml:lang=\"new ns1:alttextprop/?xml:lang value\"> <rdf:Alt> <rdf:li xml:lang=\"x-one\">x-one value</rdf:li> <rdf:li xml:lang=\"x-two\">new ns1:AltTextProp['x-two'] value</rdf:li> </rdf:Alt> </ns1:AltTextProp> <ns1:ArrayOfStructProp> <rdf:Bag> <rdf:li rdf:parseType=\"Resource\"> <ns2:Field1>Item-1</ns2:Field1> <ns2:Field2>Field 1.2 value</ns2:Field2> </rdf:li> <rdf:li rdf:parseType=\"Resource\"> <ns2:Field1>Item-2</ns2:Field1> <ns2:Field2>new ns1:ArrayOfStructProp[ns2:Field1=Item-2]/ns2:Field2 value</ns2:Field2> </rdf:li> </rdf:Bag> </ns1:ArrayOfStructProp> </rdf:Description> </rdf:RDF> </x:xmpmeta>");
    }

    //-------------------------------------------------------------------------

    write_major_label("Test value conversion utilities");
    println!("SKIPPING: Value conversion utilities not ported to Rust");

    //-------------------------------------------------------------------------

    {
        write_major_label("Test date/time utilities and special values");
        println!();

        let mut dt = XmpDateTime {
            date: Some(XmpDate {
                year: 2022,
                month: 11,
                day: 5,
            }),
            time: Some(XmpTime {
                hour: 14,
                minute: 40,
                second: 35,
                nanosecond: 42,
                time_zone: None,
            }),
        };

        dt.set_local_time_zone().unwrap();

        // We don't know when writing this test what time zone will be used
        // when running this test. All we can do is verify that *some* time zone
        // was added and that other fields weren't altered. Print the result so
        // it can be manually inspected.

        println!("Manually verify correct local time zone: {:#?}", dt);

        assert_eq!(
            dt.date.unwrap(),
            XmpDate {
                year: 2022,
                month: 11,
                day: 5
            }
        );

        let time = dt.time.unwrap();
        assert_eq!(time.hour, 14);
        assert_eq!(time.minute, 40);
        assert_eq!(time.second, 35);
        assert_eq!(time.nanosecond, 42);
        assert!(time.time_zone.is_some());

        let now = XmpDateTime::current().unwrap();
        println!("Manually verify current date/time: {:#?}", now);

        let mut local_now = now;
        local_now.convert_to_local_time().unwrap();
        println!(
            "Manually verify current date/time (LOCAL): {:#?}",
            local_now
        );

        let mut utc_now = local_now;
        utc_now.convert_to_utc().unwrap();
        println!("Manually verify current date/time (UTC): {:#?}", utc_now);

        println!();

        println!("SKIPPING: Date/time comparisons not ported to Rust");
        // TO DO (https://github.com/adobe/xmp-toolkit-rs/issues/150):
        // Decide how to port the CompareDateTime to Rust. Skipping for now.

        // 	i = SXMPUtils::CompareDateTime ( utcNow, localNow );
        // 	println!( "CompareDateTime with a == b : %d\n", i );

        // 	utcNow.second = 0;
        // 	localNow.second = 30;
        // 	i = SXMPUtils::CompareDateTime ( utcNow, localNow );
        // 	println!( "CompareDateTime with a < b : %d\n", i );

        // 	utcNow.second = 59;
        // 	i = SXMPUtils::CompareDateTime ( utcNow, localNow );
        // 	println!( "CompareDateTime with a > b : %d\n", i );
    }

    //-------------------------------------------------------------------------

    {
        write_major_label("Test CatenateArrayItems and SeparateArrayItems");
        println!("SKIPPING: CatenateArrayItems and SeparateArrayItems not ported to Rust");
        // This can be implemented by using `split` or `join`.

        println!();
    }

    //-------------------------------------------------------------------------

    println!("Remaining items in this test not planned for Rust 1.0 toolkit");

    // {
    // 	write_major_label("Test RemoveProperties and AppendProperties" );

    // 	SXMPMeta meta1 ( SIMPLE_RDF, strlen(SIMPLE_RDF) );

    // 	meta1.set_property ( NS2, "Prop", "value" );
    // 	DumpXMPObj ( log, meta1, "Parse simple RDF, add ns2:Prop" );

    // 	SXMPUtils::RemoveProperties ( &meta1, NS1, "ArrayOfStructProp" );
    // 	DumpXMPObj ( log, meta1, "Remove ns1:ArrayOfStructProp" );

    // 	SXMPUtils::RemoveProperties ( &meta1, NS1 );
    // 	DumpXMPObj ( log, meta1, "Remove all of ns1:" );

    // 	meta1.set_property ( xmp_ns::XMP, "CreatorTool", "XMPCoverage" );
    // 	meta1.set_property ( xmp_ns::XMP, "Nickname", "TXMP test" );
    // 	DumpXMPObj ( log, meta1, "Set xmp:CreatorTool (internal) and xmp:Nickname
    // (external)" );

    // 	SXMPUtils::RemoveProperties ( &meta1 );
    // 	DumpXMPObj ( log, meta1, "Remove all external properties" );

    // 	SXMPUtils::RemoveProperties ( &meta1, 0, 0, kXMPUtil_DoAllProperties );
    // 	DumpXMPObj ( log, meta1, "Remove all properties, including internal" );

    // 	meta1.set_property ( xmp_ns::XMP, "CreatorTool", "XMPCoverage" );
    // 	meta1.set_property ( xmp_ns::XMP, "Nickname", "TXMP test" );
    // 	DumpXMPObj ( log, meta1, "Set xmp:CreatorTool and xmp:Nickname again" );

    // 	SXMPMeta meta2 ( SIMPLE_RDF, strlen(SIMPLE_RDF) );

    // 	meta2.set_property ( xmp_ns::XMP, "CreatorTool", "new CreatorTool" );
    // 	meta2.set_property ( xmp_ns::XMP, "Nickname", "new Nickname" );
    // 	meta2.set_property ( xmp_ns::XMP, "Format", "new Format" );
    // 	DumpXMPObj ( log, meta2, "Create 2nd XMP object with new values" );

    // 	SXMPUtils::ApplyTemplate ( &meta1, meta2, kXMPTemplate_AddNewProperties
    // ); 	DumpXMPObj ( log, meta1, "Append 2nd to 1st, keeping old values,
    // external only" );

    // 	meta2.set_property ( xmp_ns::XMP, "CreatorTool", "newer CreatorTool" );
    // 	meta2.set_property ( xmp_ns::XMP, "Nickname", "newer Nickname" );
    // 	meta2.set_property ( xmp_ns::XMP, "Format", "newer Format" );
    // 	SXMPUtils::ApplyTemplate ( &meta1, meta2, kXMPTemplate_AddNewProperties |
    // kXMPTemplate_IncludeInternalProperties ); 	DumpXMPObj ( log, meta1,
    // "Append 2nd to 1st, keeping old values, internal also" );

    // 	meta2.set_property ( xmp_ns::XMP, "CreatorTool", "newest CreatorTool" );
    // 	meta2.set_property ( xmp_ns::XMP, "Nickname", "newest Nickname" );
    // 	meta2.set_property ( xmp_ns::XMP, "Format", "newest Format" );
    // 	SXMPUtils::ApplyTemplate ( &meta1, meta2, kXMPTemplate_AddNewProperties |
    // kXMPTemplate_ReplaceExistingProperties ); 	DumpXMPObj ( log, meta1,
    // "Append 2nd to 1st, replacing old values, external only" );

    // 	meta2.set_property ( xmp_ns::XMP, "CreatorTool", "final CreatorTool" );
    // 	meta2.set_property ( xmp_ns::XMP, "Nickname", "final Nickname" );
    // 	meta2.set_property ( xmp_ns::XMP, "Format", "final Format" );
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

    // 		SXMPUtils::DuplicateSubtree ( meta1, &meta2, NS1, "ArrayOfStructProp",
    // NS2, "NewAoS" ); 		DumpXMPObj ( log, meta2, "DuplicateSubtree to
    // different destination" );

    // 		SXMPUtils::DuplicateSubtree ( meta1, &meta1, NS1, "ArrayOfStructProp",
    // NS2, "NewAoS" ); 		DumpXMPObj ( log, meta1, "DuplicateSubtree to
    // different destination in same object" );

    // 	#else

    // 		SXMPUtils::DuplicateSubtree ( meta1, &meta2, NS1, "ArrayOfStructProp",
    // NS1, "NewAoS" ); 		DumpXMPObj ( log, meta2, "DuplicateSubtree to
    // different destination" );

    // 		SXMPUtils::DuplicateSubtree ( meta1, &meta1, NS1, "ArrayOfStructProp",
    // NS1, "NewAoS" ); 		DumpXMPObj ( log, meta1, "DuplicateSubtree to
    // different destination in same object" );

    // 	#endif

    // }

    // // --------------------------------------------------------------------------------------------

    // {
    // 	write_major_label("Test EncodeToBase64 and DecodeFromBase64" );
    // 	println!( "\n" );

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
    // 	println!( "Encoded sequence (should be A-Za-z0-9+/) : %s\n",
    // tmpStr2.c_str() );

    // 	tmpStr3.erase();
    // 	SXMPUtils::DecodeFromBase64 ( tmpStr2, &tmpStr3 );
    // 	if ( tmpStr1 != tmpStr3 ) println!( "** Error in base 64 round trip\n"
    // );

    // }

    // // --------------------------------------------------------------------------------------------

    // write_major_label("XMPCoreCoverage done" );
    // println!( "\n" );

    // panic!("\n\n---\n\naborting test for now so we can inspect output");
}
