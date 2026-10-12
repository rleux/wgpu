use super::{parse_and_validate, ErrorKind, Frontend, Options, ShaderStage};

#[test]
fn precision_qualifiers() {
    parse_and_validate(
        include_str!("../../../../tests/in/glsl/precision-qualifiers.vert"),
        ShaderStage::Vertex,
    );
    for qualifier in [
        "highp in",
        "in highp",
        "const highp in",
        "in const highp",
        "highp",
        "in out highp",
    ] {
        let source = alloc::format!(
            r#"
            #version 450
            void f({qualifier} float x) {{}}
            void main() {{}}
        "#
        );
        parse_and_validate(&source, ShaderStage::Fragment);
    }
    for qualifier in [
        "highp in mediump",
        "highp highp",
        "const out highp",
        "highp inout const",
        "in in highp",
    ] {
        let source = alloc::format!(
            r#"
            #version 450
            void f({qualifier} float x) {{}}
            void main() {{}}
        "#
        );
        assert!(
            Frontend::default()
                .parse(&Options::from(ShaderStage::Vertex), &source)
                .is_err(),
            "{qualifier}"
        );
    }
    assert!(Frontend::default()
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
                #version 450
                struct S { highp mediump float x; };
                void main() {}
            "#
        )
        .is_err());
}

#[test]
fn precision_parameter_types() {
    for precision in ["highp", "mediump", "lowp"] {
        for ty in ["bool", "bvec3", "double", "dvec2", "dmat3", "Data"] {
            for declaration in [
                alloc::format!("void f({precision} {ty} value) {{}}"),
                alloc::format!("void f(in {precision} {ty} value[2]) {{}}"),
                alloc::format!("void f({precision} {ty}[2] value) {{}}"),
                alloc::format!("void f({precision} {ty});"),
            ] {
                assert_invalid_precision_type(&declaration, precision);
            }
        }
    }
}

#[test]
fn precision_member_types() {
    for precision in ["highp", "mediump", "lowp"] {
        for ty in ["bool", "bvec3", "double", "dvec2", "dmat3", "Data"] {
            for declaration in [
                alloc::format!("struct Container {{ {precision} {ty} value; }};"),
                alloc::format!("struct Container {{ {precision} {ty} value[2]; }};"),
                alloc::format!("struct Container {{ {precision} {ty}[2] value; }};"),
            ] {
                assert_invalid_precision_type(&declaration, precision);
            }
        }
    }
}

fn assert_invalid_precision_type(declaration: &str, precision: &str) {
    let source = alloc::format!(
        "#version 450\nstruct Data {{ float value; }};\n{declaration}\nvoid main() {{}}"
    );
    let result = Frontend::default().parse(&Options::from(ShaderStage::Fragment), &source);
    assert!(result.is_err(), "accepted {declaration}");
    let errors = result.unwrap_err();
    let error = errors
        .errors
        .iter()
        .find(|error| {
            matches!(&error.kind, ErrorKind::SemanticError(message) if message.contains("precision qualifier"))
        })
        .expect("missing precision qualifier error");
    assert_eq!(&source[error.meta.to_range().unwrap()], precision);
}

#[test]
fn precision_qualified_numeric_and_opaque_types() {
    for precision in ["highp", "mediump", "lowp"] {
        for ty in ["float", "vec2", "mat3", "int", "ivec2", "uint", "uvec3"] {
            let source = alloc::format!(
                r#"
                #version 450
                struct Data {{
                    {precision} {ty} after[2];
                    {precision} {ty}[2] before;
                }};
                void first({precision} {ty} value[2]) {{}}
                void second({precision} {ty}[2] value) {{}}
                void main() {{}}
            "#
            );
            parse_and_validate(&source, ShaderStage::Fragment);
        }
        for ty in ["sampler", "texture2D", "image2D"] {
            let source = alloc::format!(
                "#version 450\nvoid f({precision} {ty} value) {{}}\nvoid main() {{}}"
            );
            parse_and_validate(&source, ShaderStage::Fragment);
        }
    }
    parse_and_validate(
        r#"
            #version 450
            struct Data { bool flag; bvec3 mask; double value; dvec2 vector; dmat3 matrix; };
            struct Container { Data value; };
            void f(bool flag, bvec3 mask, double value, dvec2 vector, dmat3 matrix, Data record) {}
            void main() {}
        "#,
        ShaderStage::Fragment,
    );
}
