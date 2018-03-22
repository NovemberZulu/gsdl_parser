use graphql::generated_lalrpop::{parse_Directive, parse_Enum, parse_Field, parse_Interface,
                                 parse_Name, parse_OuterType, parse_Type, parse_Union};
use graphql::types::{Argument, InnerTypeKind, Value};

#[test]
fn parse_name_start_with_letter() {
    assert_eq!(parse_Name("AB12").unwrap(), "AB12");
}

#[test]
fn parse_name_starts_digit_fails() {
    assert!(parse_Name("5AB12!").is_err());
}

#[test]
fn parse_name_starts_non_letter_fails() {
    assert!(parse_Name("@AB12").is_err());
}

#[test]
fn parse_name_non_letter_fails() {
    assert!(parse_Name("AB12!").is_err());
}

#[test]
fn parse_scalar_type_non_nullable() {
    let t = parse_OuterType("Type!").unwrap();

    assert_eq!(t.nullable, false);
    assert_eq!(t.inner.name, "Type");
    assert_eq!(t.inner.kind, InnerTypeKind::Scalar);
}

#[test]
fn parse_scalar_type_nullable() {
    let t = parse_OuterType("Type").unwrap();

    assert_eq!(t.nullable, true);
    assert_eq!(t.inner.name, "Type");
    assert_eq!(t.inner.kind, InnerTypeKind::Scalar);
}

#[test]
fn parse_scalar_type_with_garbage_fails() {
    assert!(parse_OuterType("Type?").is_err());
    assert!(parse_OuterType("Type!?").is_err());
    assert!(parse_OuterType("Type?!").is_err());
}

#[test]
fn parse_vector_type_not_nullable_inner_not_nullable() {
    let t = parse_OuterType("[Type!]!").unwrap();

    assert_eq!(t.nullable, false);
    assert_eq!(t.inner.name, "Type");
    assert_eq!(t.inner.kind, InnerTypeKind::Vector { nullable: false });
}

#[test]
fn parse_vector_type_not_nullable_inner_nullable() {
    let t = parse_OuterType("[Type]!").unwrap();

    assert_eq!(t.nullable, false);
    assert_eq!(t.inner.name, "Type");
    assert_eq!(t.inner.kind, InnerTypeKind::Vector { nullable: true });
}

#[test]
fn parse_vector_type_nullable_inner_not_nullable() {
    let t = parse_OuterType("[Type!]").unwrap();

    assert_eq!(t.nullable, true);
    assert_eq!(t.inner.name, "Type");
    assert_eq!(t.inner.kind, InnerTypeKind::Vector { nullable: false });
}

#[test]
fn parse_vector_type_nullable_inner_nullable() {
    let t = parse_OuterType("[Type]").unwrap();

    assert_eq!(t.nullable, true);
    assert_eq!(t.inner.name, "Type");
    assert_eq!(t.inner.kind, InnerTypeKind::Vector { nullable: true });
}

#[test]
fn parse_vector_type_with_garbage_fails() {
    assert!(parse_OuterType("[Type]?").is_err());
    assert!(parse_OuterType("[Type]!?").is_err());
    assert!(parse_OuterType("[Type]?!").is_err());
}

#[test]
fn parse_vector_type_with_inner_garbage_fails() {
    assert!(parse_OuterType("[Type?]").is_err());
    assert!(parse_OuterType("[Type!?]").is_err());
    assert!(parse_OuterType("[Type?!]").is_err());
}

#[test]
fn parse_directive_zero_parameters() {
    assert!(parse_Directive("@directive").is_ok());
}

#[test]
fn parse_directive_one_parameter() {
    assert!(parse_Directive("@directive(a:b)").is_ok());
}

#[test]
fn parse_directive_two_parameters() {
    assert!(parse_Directive("@directive(a:b c:d)").is_ok());
}

#[test]
fn parse_directive_3_parameters() {
    assert!(parse_Directive("@directive(a:b c:d e:f)").is_ok());
}

#[test]
fn parse_field_no_arguments() {
    let t = parse_Field("aaa:bbb! @ccc").unwrap();

    assert_eq!(t.name, "aaa");
    assert_eq!(t.arguments, Vec::<Argument>::new());
    assert_eq!(t.field_type.nullable, false);
    assert_eq!(t.field_type.inner.name, "bbb");
    assert_eq!(t.field_type.inner.kind, InnerTypeKind::Scalar);
}

#[test]
fn parse_field_one_argument_no_default() {
    let t = parse_Field("aaa:bbb! (ccc:ddd) @eee").unwrap();

    assert_eq!(t.name, "aaa");
    assert_eq!(t.field_type.nullable, false);
    assert_eq!(t.field_type.inner.name, "bbb");
    assert_eq!(t.field_type.inner.kind, InnerTypeKind::Scalar);
    assert_eq!(t.arguments.len(), 1);

    let a = &t.arguments[0];
    assert_eq!(a.name, "ccc");
    assert_eq!(a.argument_type.nullable, true);
    assert_eq!(a.argument_type.inner.name, "ddd");
    assert_eq!(a.argument_type.inner.kind, InnerTypeKind::Scalar);
    assert!(a.default.is_none());
}

#[test]
fn parse_field_one_argument_with_default() {
    let t = parse_Field("aaa:bbb! (ccc:ddd = eee) @fff").unwrap();

    assert_eq!(t.name, "aaa");
    assert_eq!(t.field_type.nullable, false);
    assert_eq!(t.field_type.inner.name, "bbb");
    assert_eq!(t.field_type.inner.kind, InnerTypeKind::Scalar);
    assert_eq!(t.arguments.len(), 1);

    let a = &t.arguments[0];
    assert_eq!(a.name, "ccc");
    assert_eq!(a.argument_type.nullable, true);
    assert_eq!(a.argument_type.inner.name, "ddd");
    assert_eq!(a.argument_type.inner.kind, InnerTypeKind::Scalar);
    assert_eq!(a.default, Option::Some(Value::new("eee")));
}

#[test]
fn parse_field_two_arguments_no_default() {
    let t = parse_Field("aaa:bbb! (ccc:ddd eee: [fff!]) @ggg").unwrap();

    assert_eq!(t.name, "aaa");
    assert_eq!(t.field_type.nullable, false);
    assert_eq!(t.field_type.inner.name, "bbb");
    assert_eq!(t.field_type.inner.kind, InnerTypeKind::Scalar);
    assert_eq!(t.arguments.len(), 2);

    let a = &t.arguments[0];
    assert_eq!(a.name, "ccc");
    assert_eq!(a.argument_type.nullable, true);
    assert_eq!(a.argument_type.inner.name, "ddd");
    assert_eq!(a.argument_type.inner.kind, InnerTypeKind::Scalar);
    assert!(a.default.is_none());

    let a = &t.arguments[1];
    assert_eq!(a.name, "eee");
    assert_eq!(a.argument_type.nullable, true);
    assert_eq!(a.argument_type.inner.name, "fff");
    assert_eq!(
        a.argument_type.inner.kind,
        InnerTypeKind::Vector { nullable: false }
    );
    assert!(a.default.is_none());
}

#[test]
fn parse_field_two_arguments_first_default() {
    let t = parse_Field("aaa:bbb! (ccc:ddd = eee fff: ggg!) @hhh").unwrap();

    assert_eq!(t.name, "aaa");
    assert_eq!(t.field_type.nullable, false);
    assert_eq!(t.field_type.inner.name, "bbb");
    assert_eq!(t.field_type.inner.kind, InnerTypeKind::Scalar);
    assert_eq!(t.arguments.len(), 2);

    let a = &t.arguments[0];
    assert_eq!(a.name, "ccc");
    assert_eq!(a.argument_type.nullable, true);
    assert_eq!(a.argument_type.inner.name, "ddd");
    assert_eq!(a.argument_type.inner.kind, InnerTypeKind::Scalar);
    assert_eq!(a.default, Option::Some(Value::new("eee")));

    let a = &t.arguments[1];
    assert_eq!(a.name, "fff");
    assert_eq!(a.argument_type.nullable, false);
    assert_eq!(a.argument_type.inner.name, "ggg");
    assert_eq!(a.argument_type.inner.kind, InnerTypeKind::Scalar);
    assert!(a.default.is_none());
}

#[test]
fn parse_field_two_arguments_second_default() {
    let t = parse_Field("aaa:bbb! (ccc:[ddd] eee: fff! =ggg ) @hhh").unwrap();

    assert_eq!(t.name, "aaa");
    assert_eq!(t.field_type.nullable, false);
    assert_eq!(t.field_type.inner.name, "bbb");
    assert_eq!(t.field_type.inner.kind, InnerTypeKind::Scalar);
    assert_eq!(t.arguments.len(), 2);

    let a = &t.arguments[0];
    assert_eq!(a.name, "ccc");
    assert_eq!(a.argument_type.nullable, true);
    assert_eq!(a.argument_type.inner.name, "ddd");
    assert_eq!(
        a.argument_type.inner.kind,
        InnerTypeKind::Vector { nullable: true }
    );
    assert!(a.default.is_none());

    let a = &t.arguments[1];
    assert_eq!(a.name, "eee");
    assert_eq!(a.argument_type.nullable, false);
    assert_eq!(a.argument_type.inner.name, "fff");
    assert_eq!(a.argument_type.inner.kind, InnerTypeKind::Scalar);
    assert_eq!(a.default, Option::Some(Value::new("ggg")));
}

#[test]
fn parse_field_two_arguments_both_default() {
    let t = parse_Field("aaa:bbb! (ccc:[ddd!]!=eee fff: ggg=hhh) @iii").unwrap();

    assert_eq!(t.name, "aaa");
    assert_eq!(t.field_type.nullable, false);
    assert_eq!(t.field_type.inner.name, "bbb");
    assert_eq!(t.field_type.inner.kind, InnerTypeKind::Scalar);
    assert_eq!(t.arguments.len(), 2);

    let a = &t.arguments[0];
    assert_eq!(a.name, "ccc");
    assert_eq!(a.argument_type.nullable, false);
    assert_eq!(a.argument_type.inner.name, "ddd");
    assert_eq!(
        a.argument_type.inner.kind,
        InnerTypeKind::Vector { nullable: false }
    );
    assert_eq!(a.default, Option::Some(Value::new("eee")));

    let a = &t.arguments[1];
    assert_eq!(a.name, "fff");
    assert_eq!(a.argument_type.nullable, true);
    assert_eq!(a.argument_type.inner.name, "ggg");
    assert_eq!(a.argument_type.inner.kind, InnerTypeKind::Scalar);
    assert_eq!(a.default, Option::Some(Value::new("hhh")));
}

#[test]
fn parse_interface_() {
    let t = parse_Interface(
        "interface Inter{
        var1: type1
        var2: type2!
        var3: [type3]
        var4: [type4!]
        var5: [type5]!
        var6: [type6!]!
        }",
    ).unwrap();

    assert_eq!(t.name, "Inter");
    assert_eq!(t.fields.len(), 6);

    let f = &t.fields[0];
    assert_eq!(f.name, "var1");
    assert_eq!(f.field_type.nullable, true);
    assert_eq!(f.field_type.inner.name, "type1");
    assert_eq!(f.field_type.inner.kind, InnerTypeKind::Scalar);

    let f = &t.fields[1];
    assert_eq!(f.name, "var2");
    assert_eq!(f.field_type.nullable, false);
    assert_eq!(f.field_type.inner.name, "type2");
    assert_eq!(f.field_type.inner.kind, InnerTypeKind::Scalar);

    let f = &t.fields[2];
    assert_eq!(f.name, "var3");
    assert_eq!(f.field_type.nullable, true);
    assert_eq!(f.field_type.inner.name, "type3");
    assert_eq!(
        f.field_type.inner.kind,
        InnerTypeKind::Vector { nullable: true }
    );

    let f = &t.fields[3];
    assert_eq!(f.name, "var4");
    assert_eq!(f.field_type.nullable, true);
    assert_eq!(f.field_type.inner.name, "type4");
    assert_eq!(
        f.field_type.inner.kind,
        InnerTypeKind::Vector { nullable: false }
    );

    let f = &t.fields[4];
    assert_eq!(f.name, "var5");
    assert_eq!(f.field_type.nullable, false);
    assert_eq!(f.field_type.inner.name, "type5");
    assert_eq!(
        f.field_type.inner.kind,
        InnerTypeKind::Vector { nullable: true }
    );

    let f = &t.fields[5];
    assert_eq!(f.name, "var6");
    assert_eq!(f.field_type.nullable, false);
    assert_eq!(f.field_type.inner.name, "type6");
    assert_eq!(
        f.field_type.inner.kind,
        InnerTypeKind::Vector { nullable: false }
    );
}

#[test]
fn parse_interface__directives_ignored() {
    let t = parse_Interface("interface Inter{
        var1: type1 @directive1
        var2: type2! @directive2(aaa2: bbb2)
        var3: [type3] @directive3(aaa3: bbb3 ccc3: ddd3)
        var4: [type4!] @directive4(aaa4: bbb4) @directive5(aaa5: bbb5)
        var5: [type5]! @directive6(aaa6: bbb6) @directive7(aaa7: bbb7 ccc7: ddd7 eee7: fff7)
        var6: [type6!]! @directive8(aaa8: bbb8) @directive9(aaa9: bbb9 ccc9:ddd9) @directive10(aaa10: bbb10
            ccc10: ddd10 eee10: fff10)
        var7: type7 @directive11 @directive12
        }").unwrap();

    assert_eq!(t.fields.len(), 7)
}

#[test]
fn parse_type_() {
    let t = parse_Type(
        "type Type implements inter1 inter2 inter3 {
        var1: type1
        var2: type2!
        var3: [type3]
        var4: [type4!]
        var5: [type5]!
        var6: [type6!]!
        }",
    ).unwrap();

    assert_eq!(t.name, "Type");
    assert_eq!(t.implements, vec!["inter1", "inter2", "inter3"]);
    assert_eq!(t.fields.len(), 6);

    let f = &t.fields[0];
    assert_eq!(f.name, "var1");
    assert_eq!(f.field_type.nullable, true);
    assert_eq!(f.field_type.inner.name, "type1");
    assert_eq!(f.field_type.inner.kind, InnerTypeKind::Scalar);

    let f = &t.fields[1];
    assert_eq!(f.name, "var2");
    assert_eq!(f.field_type.nullable, false);
    assert_eq!(f.field_type.inner.name, "type2");
    assert_eq!(f.field_type.inner.kind, InnerTypeKind::Scalar);

    let f = &t.fields[2];
    assert_eq!(f.name, "var3");
    assert_eq!(f.field_type.nullable, true);
    assert_eq!(f.field_type.inner.name, "type3");
    assert_eq!(
        f.field_type.inner.kind,
        InnerTypeKind::Vector { nullable: true }
    );

    let f = &t.fields[3];
    assert_eq!(f.name, "var4");
    assert_eq!(f.field_type.nullable, true);
    assert_eq!(f.field_type.inner.name, "type4");
    assert_eq!(
        f.field_type.inner.kind,
        InnerTypeKind::Vector { nullable: false }
    );

    let f = &t.fields[4];
    assert_eq!(f.name, "var5");
    assert_eq!(f.field_type.nullable, false);
    assert_eq!(f.field_type.inner.name, "type5");
    assert_eq!(
        f.field_type.inner.kind,
        InnerTypeKind::Vector { nullable: true }
    );

    let f = &t.fields[5];
    assert_eq!(f.name, "var6");
    assert_eq!(f.field_type.nullable, false);
    assert_eq!(f.field_type.inner.name, "type6");
    assert_eq!(
        f.field_type.inner.kind,
        InnerTypeKind::Vector { nullable: false }
    );
}

#[test]
fn parse_type__implements_nothing() {
    let t = parse_Type(
        "type Type {
        var1: type1
        var2: type2!
        var3: [type3]
        var4: [type4!]
        var5: [type5]!
        var6: [type6!]!
        }",
    ).unwrap();

    assert_eq!(t.name, "Type");
    assert_eq!(t.implements, Vec::<&str>::new());
    assert_eq!(t.fields.len(), 6);
}

#[test]
fn parse_type__implements_one() {
    let t = parse_Type(
        "type Type implements inter{
        var1: type1
        var2: type2!
        var3: [type3]
        var4: [type4!]
        var5: [type5]!
        var6: [type6!]!
        }",
    ).unwrap();

    assert_eq!(t.name, "Type");
    assert_eq!(t.implements, vec!["inter"]);
    assert_eq!(t.fields.len(), 6);
}

#[test]
fn parse_type__implements_error() {
    let t = parse_Type(
        "type Type implements{
        var1: type1
        }",
    );

    assert!(t.is_err());
}

#[test]
fn parse_type__directives_ignored() {
    let t = parse_Type("type typ implements inter1 inter2{
        var1: type1 @directive1
        var2: type2! @directive2(aaa2: bbb2)
        var3: [type3] @directive3(aaa3: bbb3 ccc3: ddd3)
        var4: [type4!] @directive4(aaa4: bbb4) @directive5(aaa5: bbb5)
        var5: [type5]! @directive6(aaa6: bbb6) @directive7(aaa7: bbb7 ccc7: ddd7 eee7: fff7)
        var6: [type6!]! @directive8(aaa8: bbb8) @directive9(aaa9: bbb9 ccc9:ddd9) @directive10(aaa10: bbb10
            ccc10: ddd10 eee10: fff10)
        var7: type7 @directive11 @directive12
        }").unwrap();

    assert_eq!(t.implements.len(), 2);
    assert_eq!(t.fields.len(), 7)
}

#[test]
fn parse_type_enum__zero_values() {
    let t = parse_Enum(
        "enum Enu {
        }",
    ).unwrap();

    assert_eq!(t.name, "Enu");
    assert_eq!(t.values, Vec::<String>::new());
}

#[test]
fn parse_type_enum__one_value() {
    let t = parse_Enum(
        "enum Enu {
        enu
        }",
    ).unwrap();

    assert_eq!(t.name, "Enu");
    assert_eq!(t.values, vec!["enu"]);
}

#[test]
fn parse_type_enum__three_values() {
    let t = parse_Enum(
        "enum Enu {
        enu1
        enu2 enu3
        }",
    ).unwrap();

    assert_eq!(t.name, "Enu");
    assert_eq!(t.values, vec!["enu1", "enu2", "enu3"]);
}

#[test]
fn parse_type_enum__two_values() {
    let t = parse_Enum(
        "enum Enu {
        enu1
        enu2
        }",
    ).unwrap();

    assert_eq!(t.name, "Enu");
    assert_eq!(t.values, vec!["enu1", "enu2"]);
}

#[test]
fn parse_union__one_member() {
    let t = parse_Union("union un = uni").unwrap();

    assert_eq!(t.name, "un");
    assert_eq!(t.members, vec!["uni"]);
}

#[test]
fn parse_union__two_members() {
    let t = parse_Union("union un = uni1 | uni2").unwrap();

    assert_eq!(t.name, "un");
    assert_eq!(t.members, vec!["uni1", "uni2"]);
}

#[test]
fn parse_union__three_members() {
    let t = parse_Union("union un = uni1 | uni2 | uni3").unwrap();

    assert_eq!(t.name, "un");
    assert_eq!(t.members, vec!["uni1", "uni2", "uni3"]);
}

#[test]
fn parse_union__no_members_fails() {
    assert!(parse_Union("union un = ").is_err());
}

#[test]
fn parse_union__no_pipe_fails() {
    assert!(parse_Union("union un = uni1 uni2").is_err());
}
