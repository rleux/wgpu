use super::{parse_and_validate, ShaderStage};
use alloc::vec;

#[test]
fn texel_fetch_spatial_offset() {
    let module = parse_and_validate(
        include_str!("../../../../tests/in/glsl/texel-fetch-offset.vert"),
        ShaderStage::Vertex,
    );
    let function = module
        .functions
        .iter()
        .find(|&(_, f)| f.name.as_deref() == Some("main"))
        .unwrap()
        .1;
    let loads: vec::Vec<_> = function
        .expressions
        .iter()
        .filter_map(|(_, e)| match *e {
            crate::Expression::ImageLoad {
                coordinate,
                array_index,
                ..
            } => Some((coordinate, array_index)),
            _ => None,
        })
        .collect();
    assert_eq!(loads.len(), 3);
    assert!(matches!(
        function.expressions[loads[0].0],
        crate::Expression::Binary {
            op: crate::BinaryOperator::Add,
            ..
        }
    ));
    for &(coordinate, _) in &loads[1..] {
        assert!(!matches!(
            function.expressions[coordinate],
            crate::Expression::Binary { .. }
        ));
    }
    assert!(loads[0].1.is_some());
    assert!(loads[1].1.is_some());
    assert!(loads[2].1.is_none());
    for &(_, layer) in &loads[..2] {
        assert!(matches!(
            function.expressions[layer.unwrap()],
            crate::Expression::AccessIndex { index: 2, .. }
        ));
    }
}

#[test]
fn texel_fetch_offset_dimensions() {
    for (suffix, coordinate, offset) in [
        ("1D", "1", "2"),
        ("1DArray", "ivec2(1,3)", "2"),
        ("2D", "ivec2(1,3)", "ivec2(2,-1)"),
        ("3D", "ivec3(1,3,2)", "ivec3(2,-1,1)"),
    ] {
        let source = alloc::format!(
            r#"
            #version 450
            layout(set=0,binding=0) uniform texture{suffix} t;
            layout(set=0,binding=1) uniform sampler s;
            void main() {{
                gl_Position = texelFetchOffset(
                    sampler{suffix}(t, s), {coordinate}, 0, {offset}
                );
            }}
        "#
        );
        parse_and_validate(&source, ShaderStage::Vertex);
    }
}
