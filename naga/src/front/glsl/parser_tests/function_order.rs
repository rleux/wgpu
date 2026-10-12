use super::{parse_and_validate, ErrorKind, Frontend, Options, ShaderStage, Span};

#[test]
fn function_dependency_order() {
    parse_and_validate(
        include_str!("../../../../tests/in/glsl/function-order.vert"),
        ShaderStage::Vertex,
    );
    for source in [
        r#"
            #version 450
            float f();
            float f() { return f(); }
            void main() {}
        "#,
        r#"
            #version 450
            float f();
            float g() { return f(); }
            float f() { return g(); }
            void main() {}
        "#,
    ] {
        let error = Frontend::default()
            .parse(&Options::from(ShaderStage::Vertex), source)
            .unwrap_err();
        assert!(matches!(
            &error.errors[0].kind,
            ErrorKind::SemanticError(message) if message == "recursive function graph"
        ));
        assert_ne!(error.errors[0].meta, Span::UNDEFINED);
    }
}

#[test]
fn long_function_dependency_chain() {
    use core::fmt::Write;
    let mut source = alloc::string::String::from("#version 450\n");
    for i in 0..1024 {
        writeln!(source, "float f{i}();").unwrap();
    }
    for i in 0..1023 {
        writeln!(source, "float f{i}() {{ return f{}(); }}", i + 1).unwrap();
    }
    source.push_str("float f1023() { return 1.0; } void main() { gl_Position = vec4(f0()); }");
    parse_and_validate(&source, ShaderStage::Vertex);
}
