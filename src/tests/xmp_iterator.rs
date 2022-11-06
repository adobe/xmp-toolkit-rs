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

use std::str::FromStr;

use crate::{
    tests::fixtures::*, xmp_ns, ItemPlacement, IterOptions, XmpMeta, XmpProperty, XmpValue,
};

const NS2: &str = "ns:test2/";

fn test_fixture() -> XmpMeta {
    let mut meta = XmpMeta::from_str(RDF_COVERAGE).unwrap();

    meta.set_property(NS2, "Prop", &"Prop value".into())
        .unwrap();

    meta.set_property(NS2, "Bag", &XmpValue::default().set_is_array(true))
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
        "Parse \"coverage\" RDF, add Bag items out of sequence = {:#?}",
        meta
    );

    meta
}

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

#[test]
fn default() {
    let meta = test_fixture();

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

#[test]
fn init_fail() {
    let meta = XmpMeta::new_fail();
    let mut prop_iter = meta.iter(IterOptions::default());
    assert!(prop_iter.next().is_none());
}

#[test]
fn omit_qualifiers() {
    let meta = test_fixture();

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

#[test]
fn leaf_name_only() {
    let meta = test_fixture();

    let props: Vec<XmpProperty> = meta.iter(IterOptions::default().leaf_name_only()).collect();

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

#[test]
fn leaf_nodes_only() {
    let meta = test_fixture();

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

#[test]
fn immediate_children_only() {
    let meta = test_fixture();

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

#[test]
fn schema_ns() {
    let meta = test_fixture();

    let props: Vec<XmpProperty> = meta.iter(IterOptions::default().schema_ns(NS2)).collect();
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

#[test]
fn property() {
    let meta = test_fixture();

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

#[test]
fn nested_struct_property() {
    let meta = test_fixture();

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
                name: "ns2:NestedStructProp/ns1:Outer/ns1:Middle/ns1:Inner/ns1:Field1".to_owned(),
                value: XmpValue {
                    value: "Field1 value".to_owned(),
                    options: 0
                }
            },
            XmpProperty {
                schema_ns: "ns:test2/".to_owned(),
                name: "ns2:NestedStructProp/ns1:Outer/ns1:Middle/ns1:Inner/ns2:Field2".to_owned(),
                value: XmpValue {
                    value: "Field2 value".to_owned(),
                    options: 0
                }
            }
        ]
    );
}

#[test]
fn empty_namespace() {
    let meta = test_fixture();

    let mut prop_iter = meta.iter(IterOptions::default().schema_ns("ns:empty/"));
    assert!(prop_iter.next().is_none());
}

#[test]
fn schema_ns_immediate_children_leaf_name_only() {
    let meta = test_fixture();

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

#[test]
fn namespace_immediate_children_only() {
    let meta = test_fixture();

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

#[test]
fn namespace_children_and_leaf_nodes() {
    let meta = test_fixture();

    let props: Vec<XmpProperty> = meta
        .iter(
            IterOptions::default()
                .schema_ns(NS2)
                .immediate_children_only()
                .leaf_nodes_only(),
        )
        .collect();

    check_props_exist(&meta, &props);

    assert_eq!(
        props,
        [XmpProperty {
            schema_ns: "ns:test2/".to_owned(),
            name: "ns2:Prop".to_owned(),
            value: XmpValue {
                value: "Prop value".to_owned(),
                options: 0
            }
        }]
    );
}

#[test]
fn property_immediate_children_leaf_name_only() {
    let meta = test_fixture();

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

#[test]
fn property_immediate_children() {
    let meta = test_fixture();

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

#[test]
fn middle_property_just_children() {
    let meta = test_fixture();

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

#[test]
fn skip_children_and_siblings() {
    let meta = test_fixture();

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
                .unwrap_or_else(|| panic!("Property {} {} was missing", prop.schema_ns, prop.name));

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
                name: "ns2:NestedStructProp/ns1:Outer/ns1:Middle/ns1:Inner/ns1:Field1".to_owned(),
                value: XmpValue {
                    value: "Field1 value".to_owned(),
                    options: 0
                }
            },
            XmpProperty {
                schema_ns: "ns:test2/".to_owned(),
                name: "ns2:NestedStructProp/ns1:Outer/ns1:Middle/ns1:Inner/ns2:Field2".to_owned(),
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

#[test]
fn init_fail_skip_subtre() {
    let meta = XmpMeta::new_fail();
    let mut prop_iter = meta.iter(IterOptions::default());
    prop_iter.skip_subtree(); // no-op
    assert!(prop_iter.next().is_none());
}

#[test]
fn init_fail_skip_siblings() {
    let meta = XmpMeta::new_fail();
    let mut prop_iter = meta.iter(IterOptions::default());
    prop_iter.skip_siblings(); // no-op
    assert!(prop_iter.next().is_none());
}

#[test]
fn iterate_without_aliases() {
    let mut meta = XmpMeta::default();

    meta.set_property(xmp_ns::PDF, "Author", &"PDF Author".into())
        .unwrap();
    meta.set_property(xmp_ns::PDF, "PDFProp", &"PDF Prop".into())
        .unwrap();
    meta.set_property(xmp_ns::XMP, "XMPProp", &"XMP Prop".into())
        .unwrap();
    meta.set_property(xmp_ns::DC, "DCProp", &"DC Prop".into())
        .unwrap();

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
