use super::{parse_and_validate, ErrorKind, Frontend, Options, ShaderStage};

#[test]
fn component_output_arguments() {
    parse_and_validate(
        include_str!("../../../../tests/in/glsl/component-arguments.vert"),
        ShaderStage::Vertex,
    );
}

#[test]
fn output_arguments_require_writable_lvalues() {
    for qualifier in ["out", "inout"] {
        for (argument, expected_error) in [
            ("constant_value", "writable lvalue"),
            ("input_value", "writable lvalue"),
            ("vec2(0.0)", "writable lvalue"),
            ("(value + vec2(1.0)).xy", "writable lvalue"),
            ("value.xx", "duplicate components"),
        ] {
            let source = alloc::format!(
                r#"
                #version 450
                layout(location=0) in vec2 input_value;
                const vec2 constant_value = vec2(0.0);
                void write({qualifier} vec2 target) {{ target = vec2(1.0); }}
                void main() {{
                    vec2 value = vec2(0.0);
                    write({argument});
                    gl_Position = vec4(value, 0.0, 1.0);
                }}
            "#
            );
            let errors = Frontend::default()
                .parse(&Options::from(ShaderStage::Vertex), &source)
                .unwrap_err();
            assert!(errors.errors.iter().any(|error| {
                matches!(&error.kind, ErrorKind::SemanticError(message) if message.contains(expected_error))
            }), "{qualifier} {argument}");
        }
    }
}

#[test]
fn output_overload_does_not_restrict_input_arguments() {
    parse_and_validate(
        r#"
            #version 450
            layout(location=0) in vec2 input_value;
            void consume(vec2 value) {}
            void consume(out float value) { value = 1.0; }
            void main() {
                vec2 value = vec2(0.0);
                consume(value.xx);
                consume(input_value);
                consume(vec2(1.0));
                gl_Position = vec4(value, 0.0, 1.0);
            }
        "#,
        ShaderStage::Vertex,
    );
}
