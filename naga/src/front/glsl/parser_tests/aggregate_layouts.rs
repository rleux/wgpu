use super::{parse_and_validate, Frontend, Options, ShaderStage};
use alloc::vec;

#[test]
fn value_and_buffer_struct_layouts() {
    let module = parse_and_validate(
        include_str!("../../../../tests/in/glsl/value-struct-layout.vert"),
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
fn field_reads_from_aggregate_values() {
    parse_and_validate(
        r#"
            #version 450
            struct Values { float items[2]; };
            struct Data { Values values; };
            layout(set = 0, binding = 0, std140) uniform Input {
                Data data;
            } input_data;
            Data copy_data(Data data) { return data; }
            void main() {
                Data local = input_data.data;
                float value = (true ? input_data.data : local).values.items[1];
                value += copy_data(input_data.data).values.items[0];
                value += Data(Values(float[2](1.0, 2.0))).values.items[0];
                gl_Position = vec4(value);
            }
        "#,
        ShaderStage::Vertex,
    );
}
