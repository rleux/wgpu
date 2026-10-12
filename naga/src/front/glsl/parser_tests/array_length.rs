use super::{parse_and_validate, Frontend, Options, ShaderStage};
use alloc::vec;

#[test]
fn array_length_receivers() {
    let module = parse_and_validate(
        include_str!("../../../../tests/in/glsl/array-length.vert"),
        ShaderStage::Vertex,
    );
    let function = module
        .functions
        .iter()
        .find(|&(_, f)| f.name.as_deref() == Some("main"))
        .unwrap()
        .1;
    let receivers: vec::Vec<_> = function
        .expressions
        .iter()
        .filter_map(|(_, e)| match *e {
            crate::Expression::ArrayLength(receiver) => Some(receiver),
            _ => None,
        })
        .collect();
    assert_eq!(receivers.len(), 1);
    assert!(matches!(
        function.expressions[receivers[0]],
        crate::Expression::AccessIndex { .. }
    ));
    let invalid = r#"
        #version 450
        void main() {
            float a[3];
            int n = a.length(1);
        }
    "#;
    assert!(Frontend::default()
        .parse(&Options::from(ShaderStage::Fragment), invalid)
        .is_err());
}
