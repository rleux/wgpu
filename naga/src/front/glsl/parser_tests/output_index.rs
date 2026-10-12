use super::{parse_and_validate, Frontend, Options, ShaderStage};

#[test]
fn fragment_output_indices() {
    let module = parse_and_validate(
        include_str!("../../../../tests/in/glsl/output-index-zero.frag"),
        ShaderStage::Fragment,
    );
    let output = module.entry_points[0].function.result.as_ref().unwrap();
    let crate::TypeInner::Struct { ref members, .. } = module.types[output.ty].inner else {
        panic!("missing output struct")
    };
    assert!(matches!(
        members[0].binding,
        Some(crate::Binding::Location {
            location: 1,
            blend_src: None,
            ..
        })
    ));
    for first_index in ["", ",index=0"] {
        let source = alloc::format!(
            r#"
            #version 450
            layout(location=0{first_index}) out vec4 a;
            layout(location=0,index=1) out vec4 b;
            void main() {{
                a = vec4(1.0);
                b = vec4(0.5);
            }}
        "#
        );
        parse_and_validate(&source, ShaderStage::Fragment);
    }
    for (stage, storage) in [
        (ShaderStage::Vertex, "in"),
        (ShaderStage::Vertex, "out"),
        (ShaderStage::Fragment, "in"),
    ] {
        let source = alloc::format!(
            r#"
            #version 450
            layout(location=0,index=0) {storage} vec4 a;
            void main() {{}}
        "#
        );
        assert!(Frontend::default()
            .parse(&Options::from(stage), &source)
            .is_err());
    }
}
