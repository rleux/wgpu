use alloc::{borrow::ToOwned, vec};

use pp_rs::token::PreprocessorError;

use super::{
    ast::Profile,
    error::ExpectedToken,
    error::{Error, ErrorKind, ParseErrors},
    token::TokenValue,
    Frontend, Options, Span,
};
use crate::ShaderStage;

#[cfg(test)]
use std::println;

#[test]
fn version() {
    let mut frontend = Frontend::default();

    // invalid versions
    assert_eq!(
        frontend
            .parse(
                &Options::from(ShaderStage::Vertex),
                "#version 99000\n void main(){}",
            )
            .err()
            .unwrap(),
        ParseErrors {
            errors: vec![Error {
                kind: ErrorKind::InvalidVersion(99000),
                meta: Span::new(9, 14)
            }],
        },
    );

    assert_eq!(
        frontend
            .parse(
                &Options::from(ShaderStage::Vertex),
                "#version 449\n void main(){}",
            )
            .err()
            .unwrap(),
        ParseErrors {
            errors: vec![Error {
                kind: ErrorKind::InvalidVersion(449),
                meta: Span::new(9, 12)
            }]
        },
    );

    assert_eq!(
        frontend
            .parse(
                &Options::from(ShaderStage::Vertex),
                "#version 450 smart\n void main(){}",
            )
            .err()
            .unwrap(),
        ParseErrors {
            errors: vec![Error {
                kind: ErrorKind::InvalidProfile("smart".into()),
                meta: Span::new(13, 18),
            }]
        },
    );

    assert_eq!(
        frontend
            .parse(
                &Options::from(ShaderStage::Vertex),
                "#version 450\nvoid main(){} #version 450",
            )
            .err()
            .unwrap(),
        ParseErrors {
            errors: vec![
                Error {
                    kind: ErrorKind::PreprocessorError(PreprocessorError::UnexpectedHash,),
                    meta: Span::new(27, 28),
                },
                Error {
                    kind: ErrorKind::InvalidToken(
                        TokenValue::Identifier("version".into()),
                        vec![ExpectedToken::Eof]
                    ),
                    meta: Span::new(28, 35)
                }
            ]
        },
    );

    // valid versions
    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            "  #  version 450\nvoid main() {}",
        )
        .unwrap();
    assert_eq!(
        (frontend.metadata().version, frontend.metadata().profile),
        (450, Profile::Core)
    );

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            "#version 450\nvoid main() {}",
        )
        .unwrap();
    assert_eq!(
        (frontend.metadata().version, frontend.metadata().profile),
        (450, Profile::Core)
    );

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            "#version 450 core\nvoid main(void) {}",
        )
        .unwrap();
    assert_eq!(
        (frontend.metadata().version, frontend.metadata().profile),
        (450, Profile::Core)
    );
}

#[test]
fn control_flow() {
    let mut frontend = Frontend::default();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void main() {
            if (true) {
                return 1;
            } else {
                return 2;
            }
        }
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void main() {
            if (true) {
                return 1;
            }
        }
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void main() {
            int x;
            int y = 3;
            switch (5) {
                case 2:
                    x = 2;
                case 5:
                    x = 5;
                    y = 2;
                    break;
                default:
                    x = 0;
            }
        }
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void main() {
            int x = 0;
            while(x < 5) {
                x = x + 1;
            }
            do {
                x = x - 1;
            } while(x >= 4)
        }
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void main() {
            int x = 0;
            for(int i = 0; i < 10;) {
                x = x + 2;
            }
            for(;;);
            return x;
        }
        "#,
        )
        .unwrap();
}

#[test]
fn declarations() {
    let mut frontend = Frontend::default();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #version 450
        layout(location = 0) in vec2 v_uv;
        layout(location = 0) out vec4 o_color;
        layout(set = 1, binding = 1) uniform texture2D tex;
        layout(set = 1, binding = 2) uniform sampler tex_sampler;

        layout(early_fragment_tests) in;

        void main() {}
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #version 450
        layout(std140, set = 2, binding = 0)
        uniform u_locals {
            vec3 model_offs;
            float load_time;
            ivec4 atlas_offs;
        };

        void main() {}
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #version 450
        layout(push_constant)
        uniform u_locals {
            vec3 model_offs;
            float load_time;
            ivec4 atlas_offs;
        };

        void main() {}
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #version 450
        layout(std430, set = 2, binding = 0)
        uniform u_locals {
            vec3 model_offs;
            float load_time;
            ivec4 atlas_offs;
        };

        void main() {}
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #version 450
        layout(std140, set = 2, binding = 0)
        uniform u_locals {
            vec3 model_offs;
            float load_time;
        } block_var;

        void main() {
            load_time * model_offs;
            block_var.load_time * block_var.model_offs;
        }
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #version 450
        float vector = vec4(1.0 / 17.0,  9.0 / 17.0,  3.0 / 17.0, 11.0 / 17.0);

        void main() {}
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #version 450
        precision highp float;

        void main() {}
        "#,
        )
        .unwrap();
}

#[test]
fn textures() {
    let mut frontend = Frontend::default();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #version 450
        layout(location = 0) in vec2 v_uv;
        layout(location = 0) out vec4 o_color;
        layout(set = 1, binding = 1) uniform texture2D tex;
        layout(set = 1, binding = 2) uniform sampler tex_sampler;
        void main() {
            o_color = texture(sampler2D(tex, tex_sampler), v_uv);
            o_color.a = texture(sampler2D(tex, tex_sampler), v_uv, 2.0).a;
        }
        "#,
        )
        .unwrap();
}

#[test]
fn functions() {
    let mut frontend = Frontend::default();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void test1(float);
        void test1(float) {}

        void main() {}
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void test2(float a) {}
        void test3(float a, float b) {}
        void test4(float, float) {}

        void main() {}
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        float test(float a) { return a; }

        void main() {}
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        float test(vec4 p) {
            return p.x;
        }

        void main() {}
        "#,
        )
        .unwrap();

    // Function overloading
    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        float test(vec2 p);
        float test(vec3 p);
        float test(vec4 p);

        float test(vec2 p) {
            return p.x;
        }

        float test(vec3 p) {
            return p.x;
        }

        float test(vec4 p) {
            return p.x;
        }

        void main() {}
        "#,
        )
        .unwrap();

    assert_eq!(
        frontend
            .parse(
                &Options::from(ShaderStage::Vertex),
                r#"
                #  version 450
                int test(vec4 p) {
                    return p.x;
                }

                float test(vec4 p) {
                    return p.x;
                }

                void main() {}
                "#,
            )
            .err()
            .unwrap(),
        ParseErrors {
            errors: vec![Error {
                kind: ErrorKind::SemanticError("Function already defined".into()),
                meta: Span::new(134, 152),
            }]
        },
    );

    println!();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        float callee(uint q) {
            return float(q);
        }

        float caller() {
            callee(1u);
        }

        void main() {}
        "#,
        )
        .unwrap();

    // Nested function call
    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
            #  version 450
            layout(set = 0, binding = 1) uniform texture2D t_noise;
            layout(set = 0, binding = 2) uniform sampler s_noise;

            void main() {
                textureLod(sampler2D(t_noise, s_noise), vec2(1.0), 0);
            }
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void fun(vec2 in_parameter, out float out_parameter) {
            ivec2 _ = ivec2(in_parameter);
        }

        void main() {
            float a;
            fun(vec2(1.0), a);
        }
        "#,
        )
        .unwrap();
}

#[test]
fn constants() {
    use crate::{Constant, Expression, Type, TypeInner};

    let mut frontend = Frontend::default();

    let module = frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        const float a = 1.0;
        float global = a;
        const float b = a;

        void main() {}
        "#,
        )
        .unwrap();

    let mut types = module.types.iter();
    let mut constants = module.constants.iter();
    let mut global_expressions = module.global_expressions.iter();

    let (ty_handle, ty) = types.next().unwrap();
    assert_eq!(
        ty,
        &Type {
            name: None,
            inner: TypeInner::Scalar(crate::Scalar::F32)
        }
    );

    let (init_handle, init) = global_expressions.next().unwrap();
    assert_eq!(init, &Expression::Literal(crate::Literal::F32(1.0)));

    assert_eq!(
        constants.next().unwrap().1,
        &Constant {
            name: Some("a".to_owned()),
            ty: ty_handle,
            init: init_handle
        }
    );

    assert_eq!(
        constants.next().unwrap().1,
        &Constant {
            name: Some("b".to_owned()),
            ty: ty_handle,
            init: init_handle
        }
    );

    assert!(constants.next().is_none());
}

#[test]
fn function_overloading() {
    let mut frontend = Frontend::default();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450

        float saturate(float v) { return clamp(v, 0.0, 1.0); }
        vec2 saturate(vec2 v) { return clamp(v, vec2(0.0), vec2(1.0)); }
        vec3 saturate(vec3 v) { return clamp(v, vec3(0.0), vec3(1.0)); }
        vec4 saturate(vec4 v) { return clamp(v, vec4(0.0), vec4(1.0)); }

        void main() {
            float v1 = saturate(1.5);
            vec2 v2 = saturate(vec2(0.5, 1.5));
            vec3 v3 = saturate(vec3(0.5, 1.5, 2.5));
            vec3 v4 = saturate(vec4(0.5, 1.5, 2.5, 3.5));
        }
        "#,
        )
        .unwrap();
}

#[test]
fn implicit_conversions() {
    let mut frontend = Frontend::default();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void main() {
            mat4 a = mat4(1);
            float b = 1u;
            float c = 1 + 2.0;
        }
        "#,
        )
        .unwrap();

    assert_eq!(
        frontend
            .parse(
                &Options::from(ShaderStage::Vertex),
                r#"
                #  version 450
                void test(int a) {}
                void test(uint a) {}

                void main() {
                    test(1.0);
                }
                "#,
            )
            .err()
            .unwrap(),
        ParseErrors {
            errors: vec![Error {
                kind: ErrorKind::SemanticError("Unknown function \'test\'".into()),
                meta: Span::new(156, 165),
            }]
        },
    );

    assert_eq!(
        frontend
            .parse(
                &Options::from(ShaderStage::Vertex),
                r#"
                #  version 450
                void test(float a) {}
                void test(uint a) {}

                void main() {
                    test(1);
                }
                "#,
            )
            .err()
            .unwrap(),
        ParseErrors {
            errors: vec![Error {
                kind: ErrorKind::SemanticError("Ambiguous best function for \'test\'".into()),
                meta: Span::new(158, 165),
            }]
        }
    );
}

#[test]
fn structs() {
    let mut frontend = Frontend::default();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        Test {
            vec4 pos;
          } xx;

        void main() {}
        "#,
        )
        .unwrap_err();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        struct Test {
            vec4 pos;
        };

        void main() {}
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        const int NUM_VECS = 42;
        struct Test {
            vec4 vecs[NUM_VECS];
        };

        void main() {}
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        struct Hello {
            vec4 test;
        } test() {
            return Hello( vec4(1.0) );
        }

        void main() {}
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        struct Test {};

        void main() {}
        "#,
        )
        .unwrap_err();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        inout struct Test {
            vec4 x;
        };

        void main() {}
        "#,
        )
        .unwrap_err();
}

#[test]
fn swizzles() {
    let mut frontend = Frontend::default();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void main() {
            vec4 v = vec4(1);
            v.xyz = vec3(2);
            v.x = 5.0;
            v.xyz.zxy.yx.xy = vec2(5.0, 1.0);
        }
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void main() {
            vec4 v = vec4(1);
            v.xx = vec2(5.0);
        }
        "#,
        )
        .unwrap_err();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void main() {
            vec3 v = vec3(1);
            v.w = 2.0;
        }
        "#,
        )
        .unwrap_err();
}

#[test]
fn expressions() {
    let mut frontend = Frontend::default();

    // Vector indexing
    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        float test(int index) {
            vec4 v = vec4(1.0, 2.0, 3.0, 4.0);
            return v[index] + 1.0;
        }

        void main() {}
        "#,
        )
        .unwrap();

    // Prefix increment/decrement
    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void main() {
            uint index = 0;

            --index;
            ++index;
        }
        "#,
        )
        .unwrap();

    // Dynamic indexing of array
    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void main() {
            const vec4 positions[1] = { vec4(0) };

            gl_Position = positions[gl_VertexIndex];
        }
        "#,
        )
        .unwrap();
}

fn parse_and_validate(source: &str, stage: ShaderStage) -> crate::Module {
    let module = Frontend::default()
        .parse(&Options::from(stage), source)
        .unwrap();
    crate::valid::Validator::new(
        crate::valid::ValidationFlags::all(),
        crate::valid::Capabilities::all(),
    )
    .validate(&module)
    .unwrap();
    module
}

#[test]
fn matrix_component_selection() {
    parse_and_validate(
        include_str!("../../../tests/in/glsl/matrix-components.vert"),
        ShaderStage::Vertex,
    );
    for field in ["w", "xx"] {
        let source = alloc::format!(
            r#"
            #version 450
            void main() {{
                mat3 m;
                m[0].{field} = vec2(1.0);
            }}
            "#
        );
        assert!(Frontend::default()
            .parse(&Options::from(ShaderStage::Vertex), &source)
            .is_err());
    }
}

#[test]
fn matrix_interface_locations() {
    let module = parse_and_validate(
        include_str!("../../../tests/in/glsl/matrix-interface.frag"),
        ShaderStage::Fragment,
    );
    let args = &module.entry_points[0].function.arguments;
    assert_eq!(args.len(), 9);
    for (i, arg) in args.iter().enumerate() {
        let Some(crate::Binding::Location {
            location,
            interpolation,
            ..
        }) = arg.binding
        else {
            panic!("missing location")
        };
        assert_eq!(location, i as u32);
        assert_eq!(
            interpolation,
            Some(if i < 6 {
                crate::Interpolation::Flat
            } else {
                crate::Interpolation::Perspective
            })
        );
    }
    let source = r#"
        #version 450
        layout(location=4294967295u) in mat2 m;
        void main() { gl_Position = vec4(m[0], m[1]); }
    "#;
    let error = Frontend::default()
        .parse(&Options::from(ShaderStage::Vertex), source)
        .unwrap_err();
    assert!(error.errors.iter().any(|error| {
        matches!(&error.kind, ErrorKind::SemanticError(message) if message == "interface location overflow")
    }));
    let module = parse_and_validate(
        r#"
            #version 450
            layout(location=0) noperspective centroid in mat2 m[2];
            void main() {}
        "#,
        ShaderStage::Fragment,
    );
    for arg in &module.entry_points[0].function.arguments {
        assert!(matches!(
            arg.binding,
            Some(crate::Binding::Location {
                interpolation: Some(crate::Interpolation::Linear),
                sampling: Some(crate::Sampling::Centroid),
                ..
            })
        ));
    }
}

#[test]
fn interface_sampling_qualifiers() {
    for (qualifier, expected_sampling) in [
        ("", crate::Sampling::Center),
        ("centroid", crate::Sampling::Centroid),
        ("sample", crate::Sampling::Sample),
    ] {
        let source = alloc::format!(
            r#"
            #version 450
            layout(location=0) {qualifier} in float scalar;
            layout(location=1) {qualifier} in vec2 vector;
            layout(location=2) {qualifier} in mat2 matrix;
            layout(location=4) {qualifier} in mat2 matrices[2];
            void main() {{}}
        "#
        );
        let module = parse_and_validate(&source, ShaderStage::Fragment);
        let arguments = &module.entry_points[0].function.arguments;
        assert_eq!(arguments.len(), 8);
        for (index, argument) in arguments.iter().enumerate() {
            let Some(crate::Binding::Location {
                location,
                interpolation,
                sampling,
                ..
            }) = argument.binding
            else {
                panic!("missing location")
            };
            assert_eq!(location, index as u32);
            assert_eq!(interpolation, Some(crate::Interpolation::Perspective));
            assert_eq!(
                sampling,
                Some(expected_sampling),
                "{qualifier} at {location}"
            );
        }
    }
}

#[test]
fn matrix_output_locations() {
    let module = parse_and_validate(
        r#"
            #version 450
            layout(location=2) flat out mat2x3 m[2];
            void main() {
                m[0] = mat2x3(1.0);
                m[1] = mat2x3(2.0);
                gl_Position = vec4(0.0);
            }
        "#,
        ShaderStage::Vertex,
    );
    let result = module.entry_points[0].function.result.as_ref().unwrap();
    let crate::TypeInner::Struct { ref members, .. } = module.types[result.ty].inner else {
        panic!("missing output struct")
    };
    let locations: vec::Vec<_> = members
        .iter()
        .filter_map(|m| match m.binding {
            Some(crate::Binding::Location { location, .. }) => Some(location),
            _ => None,
        })
        .collect();
    assert_eq!(locations, [2, 3, 4, 5]);
}

#[test]
fn function_dependency_order() {
    parse_and_validate(
        include_str!("../../../tests/in/glsl/function-order.vert"),
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

#[test]
fn fragment_output_indices() {
    let module = parse_and_validate(
        include_str!("../../../tests/in/glsl/output-index-zero.frag"),
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

#[test]
fn precision_qualifiers() {
    parse_and_validate(
        include_str!("../../../tests/in/glsl/precision-qualifiers.vert"),
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

#[test]
fn value_and_buffer_struct_layouts() {
    let module = parse_and_validate(
        include_str!("../../../tests/in/glsl/value-struct-layout.vert"),
        ShaderStage::Vertex,
    );
    let strides: vec::Vec<_> = module
        .types
        .iter()
        .filter_map(|(_, ty)| match ty.inner {
            crate::TypeInner::Array { base, stride, .. }
                if module.types[base].inner == crate::TypeInner::Scalar(crate::Scalar::F32) =>
            {
                Some(stride)
            }
            _ => None,
        })
        .collect();
    assert!(strides.contains(&4));
    assert!(strides.contains(&16));
    let source = r#"
        #version 450
        struct A { float x[2]; };
        struct B { float x[2]; };
        void main() {
            A a;
            B b;
            a = b;
            gl_Position = vec4(0.0);
        }
    "#;
    let module = Frontend::default()
        .parse(&Options::from(ShaderStage::Vertex), source)
        .unwrap();
    assert!(crate::valid::Validator::new(
        crate::valid::ValidationFlags::all(),
        crate::valid::Capabilities::all()
    )
    .validate(&module)
    .is_err());
}

#[test]
fn component_output_arguments() {
    parse_and_validate(
        include_str!("../../../tests/in/glsl/component-arguments.vert"),
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
