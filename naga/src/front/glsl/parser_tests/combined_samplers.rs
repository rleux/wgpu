use super::{Frontend, Options, ShaderStage};

#[test]
fn combined_sampler_metadata() {
    let mut frontend = Frontend::default();
    let source = include_str!("../../../../tests/in/glsl/combined-samplers.vert");
    let mut module = frontend
        .parse(&Options::from(ShaderStage::Vertex), source)
        .unwrap();
    crate::valid::Validator::new(
        crate::valid::ValidationFlags::all(),
        crate::valid::Capabilities::all(),
    )
    .validate(&module)
    .unwrap();
    assert_eq!(frontend.metadata().combined_samplers.len(), 3);
    for (index, &(image, sampler)) in frontend.metadata().combined_samplers.iter().enumerate() {
        assert_eq!(
            image,
            crate::ResourceBinding {
                group: 0,
                binding: index as u32
            }
        );
        assert_ne!(image, sampler);
        assert_eq!(
            module
                .global_variables
                .iter()
                .filter(|&(_, v)| v.binding == Some(sampler))
                .count(),
            1
        );
    }
    crate::compact::compact(&mut module, crate::compact::KeepUnused::No);
    for &(image, _) in &frontend.metadata().combined_samplers {
        assert!(module
            .global_variables
            .iter()
            .any(|(_, v)| v.binding == Some(image)));
    }
    frontend
        .parse(
            &Options::from(ShaderStage::Fragment),
            "#version 450\nvoid main() {}",
        )
        .unwrap();
    assert!(frontend.metadata().combined_samplers.is_empty());
}

#[test]
fn combined_sampler_declarations() {
    for declaration in [
        "sampler2D image;",
        "uniform sampler2D image[2];",
        "uniform sampler2D first, second;",
    ] {
        let source = alloc::format!(
            r#"
            #version 450
            {declaration}
            void main() {{}}
        "#
        );
        assert!(Frontend::default()
            .parse(&Options::from(ShaderStage::Fragment), &source)
            .is_err());
    }
    let source = r#"
        #version 450
        layout(set=4294967295u,binding=0) uniform sampler existing;
        layout(set=0,binding=0) uniform sampler2D image;
        void main() { gl_Position = textureLod(image, vec2(0.0), 0.0); }
    "#;
    let mut frontend = Frontend::default();
    frontend
        .parse(&Options::from(ShaderStage::Vertex), source)
        .unwrap();
    assert_eq!(frontend.metadata().combined_samplers[0].1.binding, 1);
}
