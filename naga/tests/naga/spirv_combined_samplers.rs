use naga::{back::spv, valid, Module, ResourceBinding, ShaderStage};
use rspirv::{binary::Disassemble, dr::Operand};

fn binding(binding: u32) -> ResourceBinding {
    ResourceBinding { group: 0, binding }
}

fn write(writer: &mut spv::Writer, module: &Module) -> Result<Vec<u32>, spv::Error> {
    write_pipeline(writer, module, None)
}

fn write_pipeline(
    writer: &mut spv::Writer,
    module: &Module,
    pipeline: Option<&spv::PipelineOptions>,
) -> Result<Vec<u32>, spv::Error> {
    let info = valid::Validator::new(valid::ValidationFlags::all(), valid::Capabilities::all())
        .validate(module)
        .unwrap();
    let mut words = Vec::new();
    writer.write(module, &info, pipeline, &None, &mut words)?;
    Ok(words)
}

fn combined_writer() -> spv::Writer {
    let mut writer = spv::Writer::new(&spv::Options::default()).unwrap();
    writer.set_combined_image_samplers([(binding(0), binding(1))]);
    writer
}

const RESOURCES: &str = "
    @group(0) @binding(0) var image: texture_2d<f32>;
    @group(0) @binding(1) var sampling: sampler;
";

#[test]
fn combined_glsl_snapshot() {
    let mut frontend = naga::front::glsl::Frontend::default();
    let mut module = frontend
        .parse(
            &naga::front::glsl::Options::from(ShaderStage::Vertex),
            include_str!("../in/glsl/combined-samplers.vert"),
        )
        .unwrap();
    let pairs = frontend.metadata().combined_samplers.clone();
    let mut writer = spv::Writer::new(&spv::Options::default()).unwrap();
    writer.set_combined_image_samplers(pairs.clone());
    write(&mut writer, &module).unwrap();
    naga::compact::compact(&mut module, naga::compact::KeepUnused::No);
    assert!(!module
        .global_variables
        .iter()
        .any(|(_, v)| v.binding == Some(pairs[1].1)));
    let words = write(&mut writer, &module).unwrap();
    assert_eq!(words, write(&mut writer, &module).unwrap());
    let spv_module = rspirv::dr::load_words(&words).unwrap();
    let globals = &spv_module.types_global_values;
    assert_eq!(
        globals
            .iter()
            .filter(|i| i.class.opcode == spirv::Op::TypeSampledImage)
            .count(),
        3
    );
    assert_eq!(
        globals
            .iter()
            .filter(|i| i.class.opcode == spirv::Op::Variable
                && i.operands[0] == Operand::StorageClass(spirv::StorageClass::UniformConstant))
            .count(),
        3
    );
    for variable in globals.iter().filter(|i| {
        i.class.opcode == spirv::Op::Variable
            && i.operands[0] == Operand::StorageClass(spirv::StorageClass::UniformConstant)
    }) {
        let pointer = globals
            .iter()
            .find(|i| i.result_id == variable.result_type)
            .unwrap();
        let Operand::IdRef(pointee) = pointer.operands[1] else {
            panic!("missing pointer type")
        };
        assert!(
            globals
                .iter()
                .any(|i| i.result_id == Some(pointee)
                    && i.class.opcode == spirv::Op::TypeSampledImage)
        );
    }
    assert!(!spv_module
        .all_inst_iter()
        .any(|i| i.class.opcode == spirv::Op::SampledImage));
    let input = naga_test::Input {
        keep_input_extension: true,
        ..naga_test::Input::new("glsl", "combined-samplers", "vert")
    };
    input.write_output_file(
        "spv",
        "spvasm",
        spv_module.disassemble(),
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/out"),
    );

    let mut writer = spv::Writer::new(&spv::Options {
        lang_version: (1, 4),
        ..Default::default()
    })
    .unwrap();
    writer.set_combined_image_samplers(pairs);
    let words = write(&mut writer, &module).unwrap();
    let spv_module = rspirv::dr::load_words(words).unwrap();
    input.write_output_file(
        "spv",
        "spv14.spvasm",
        spv_module.disassemble(),
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/out"),
    );
}

fn parse_with_resources(body: &str) -> Module {
    naga::front::wgsl::parse_str(&format!("{RESOURCES}\n{body}")).unwrap()
}

#[test]
fn combined_bindings_reused_between_entry_points() {
    let module = naga::front::wgsl::parse_str(
        r#"
        @group(0) @binding(0) var first_image: texture_2d<f32>;
        @group(0) @binding(1) var first_sampler: sampler;
        @group(0) @binding(0) var second_image: texture_3d<f32>;
        @group(0) @binding(1) var second_sampler: sampler;
        @vertex fn first() -> @builtin(position) vec4f {
            return textureSampleLevel(first_image, first_sampler, vec2f(0), 0);
        }
        @vertex fn second() -> @builtin(position) vec4f {
            return textureSampleLevel(second_image, second_sampler, vec3f(0), 0);
        }
    "#,
    )
    .unwrap();
    let mut writer = combined_writer();
    let mut first_output = None;
    for entry_point in ["first", "second", "first"] {
        let pipeline = spv::PipelineOptions {
            shader_stage: ShaderStage::Vertex,
            entry_point: entry_point.into(),
        };
        let words = write_pipeline(&mut writer, &module, Some(&pipeline)).unwrap();
        if entry_point == "first" {
            if let Some(ref previous) = first_output {
                assert_eq!(&words, previous);
            } else {
                first_output = Some(words.clone());
            }
        }
        let output = rspirv::dr::load_words(words).unwrap();
        assert_eq!(output.entry_points.len(), 1);
        assert_eq!(
            output.entry_points[0].operands[2],
            Operand::LiteralString(entry_point.into())
        );
        let combined_type = output
            .types_global_values
            .iter()
            .find(|instruction| instruction.class.opcode == spirv::Op::TypeSampledImage)
            .unwrap();
        let Operand::IdRef(image_type) = combined_type.operands[0] else {
            panic!("missing image type")
        };
        let image_type = output
            .types_global_values
            .iter()
            .find(|instruction| instruction.result_id == Some(image_type))
            .unwrap();
        assert_eq!(
            image_type.operands[1],
            Operand::Dim(if entry_point == "first" {
                spirv::Dim::Dim2D
            } else {
                spirv::Dim::Dim3D
            })
        );
        assert_eq!(
            output
                .types_global_values
                .iter()
                .filter(|instruction| {
                    instruction.class.opcode == spirv::Op::Variable
                        && instruction.operands[0]
                            == Operand::StorageClass(spirv::StorageClass::UniformConstant)
                })
                .count(),
            1
        );
        assert!(!output
            .all_inst_iter()
            .any(|instruction| instruction.class.opcode == spirv::Op::SampledImage));
    }
    assert!(matches!(
        write(&mut writer, &module),
        Err(spv::Error::Validation(
            "ambiguous combined image/sampler binding"
        ))
    ));
}

#[test]
fn combined_sampler_uses_are_scoped_to_entry_point() {
    let other_value =
        "textureSampleLevel(other, sampling, vec2f(0), 0) + textureLoad(image, vec2i(0), 0)";
    for second_value in [other_value, "value_for_second()"] {
        let module = parse_with_resources(&format!(
            r#"
            @group(0) @binding(2) var other: texture_2d<f32>;
            fn value_for_second() -> vec4f {{ return {other_value}; }}
            @vertex fn first() -> @builtin(position) vec4f {{
                return textureSampleLevel(image, sampling, vec2f(0), 0);
            }}
            @fragment fn second() -> @location(0) vec4f {{ return {second_value}; }}
        "#
        ));
        let mut writer = combined_writer();
        let first = spv::PipelineOptions {
            shader_stage: ShaderStage::Vertex,
            entry_point: "first".into(),
        };
        write_pipeline(&mut writer, &module, Some(&first)).unwrap();
        let second = spv::PipelineOptions {
            shader_stage: ShaderStage::Fragment,
            entry_point: "second".into(),
        };
        assert!(matches!(
            write_pipeline(&mut writer, &module, Some(&second)),
            Err(spv::Error::Validation(
                "combined sampler used with a different image"
            ))
        ));
    }
}

#[test]
fn combined_resource_validation_respects_function_stage() {
    let module = parse_with_resources(
        r#"
        fn load(t: texture_2d<f32>) -> vec4f { return textureLoad(t, vec2i(0), 0); }
        fn fragment_only() -> vec4f {
            if textureDimensions(image).x == 0u { discard; }
            return load(image);
        }
        @vertex fn first() -> @builtin(position) vec4f {
            return textureSampleLevel(image, sampling, vec2f(0), 0)
                + vec4f(f32(textureDimensions(image).x));
        }
        @fragment fn second() -> @location(0) vec4f { return fragment_only(); }
    "#,
    );
    let info = valid::Validator::new(valid::ValidationFlags::all(), valid::Capabilities::all())
        .validate(&module)
        .unwrap();
    let (fragment, _) = module
        .functions
        .iter()
        .find(|(_, function)| function.name.as_deref() == Some("fragment_only"))
        .unwrap();
    assert!(info
        .get_entry_point(0)
        .dominates_global_use(&info[fragment]));
    let mut writer = combined_writer();
    let first = spv::PipelineOptions {
        shader_stage: ShaderStage::Vertex,
        entry_point: "first".into(),
    };
    write_pipeline(&mut writer, &module, Some(&first)).unwrap();
    let second = spv::PipelineOptions {
        shader_stage: ShaderStage::Fragment,
        entry_point: "second".into(),
    };
    assert!(matches!(
        write_pipeline(&mut writer, &module, Some(&second)),
        Err(spv::Error::Validation(
            "combined resources cannot be passed to functions"
        ))
    ));
}

#[test]
fn reject_combined_resource_arguments() {
    for body in [
        r#"
            fn helper(t: texture_2d<f32>) -> vec4f {
                return textureLoad(t, vec2i(0), 0);
            }
            @vertex fn main() -> @builtin(position) vec4f {
                return helper(image);
            }
        "#,
        r#"
            fn helper(s: sampler) -> vec4f {
                return textureSampleLevel(image, s, vec2f(0), 0);
            }
            @vertex fn main() -> @builtin(position) vec4f {
                if true {
                    return helper(sampling);
                }
                return vec4f(0);
            }
        "#,
    ] {
        let module = parse_with_resources(body);
        let mut writer = combined_writer();
        assert!(matches!(
            write(&mut writer, &module),
            Err(spv::Error::Validation(
                "combined resources cannot be passed to functions"
            ))
        ));
        writer.set_combined_image_samplers([]);
        write(&mut writer, &module).unwrap();
    }
}

#[test]
fn reject_mismatched_combined_sampler() {
    let module = parse_with_resources(
        r#"
        @group(0) @binding(2) var other: texture_2d<f32>;
        @vertex fn main() -> @builtin(position) vec4f {
            return textureSampleLevel(other, sampling, vec2f(0), 0);
        }
    "#,
    );
    assert!(matches!(
        write(&mut combined_writer(), &module),
        Err(spv::Error::Validation(
            "combined sampler used with a different image"
        ))
    ));
}

#[test]
fn validate_combined_bindings_and_reset() {
    let module = parse_with_resources(
        r#"
        @group(0) @binding(2) var other: texture_2d<f32>;
        @vertex fn main() -> @builtin(position) vec4f {
            return textureSampleLevel(image, sampling, vec2f(0), 0);
        }
    "#,
    );
    let mut writer = combined_writer();
    for (case, pairs) in [
        (
            "same image and sampler binding",
            vec![(binding(0), binding(0))],
        ),
        (
            "duplicate image",
            vec![(binding(0), binding(1)), (binding(0), binding(2))],
        ),
        (
            "duplicate sampler",
            vec![(binding(0), binding(1)), (binding(2), binding(1))],
        ),
        ("sampler used as image", vec![(binding(1), binding(2))]),
        ("image used as sampler", vec![(binding(0), binding(2))]),
        ("missing image", vec![(binding(9), binding(1))]),
    ] {
        writer.set_combined_image_samplers(pairs);
        assert!(
            matches!(write(&mut writer, &module), Err(spv::Error::Validation(_))),
            "{case}"
        );
    }
    writer.set_combined_image_samplers([(binding(0), binding(1))]);
    write(&mut writer, &module).unwrap();
    writer.set_combined_image_samplers([]);
    let separate = write(&mut writer, &module).unwrap();
    let mut fresh = spv::Writer::new(&spv::Options::default()).unwrap();
    assert_eq!(separate, write(&mut fresh, &module).unwrap());
}

#[test]
fn unused_combined_bindings_after_compaction() {
    let mut module = parse_with_resources(
        r#"
        @vertex fn main() -> @builtin(position) vec4f {
            return vec4f(1);
        }
    "#,
    );
    naga::compact::compact(&mut module, naga::compact::KeepUnused::No);
    assert!(module.global_variables.is_empty());
    write(&mut combined_writer(), &module).unwrap();
}

#[test]
fn combined_image_with_separate_sampler() {
    let module = parse_with_resources(
        r#"
        @group(0) @binding(2) var other: sampler;
        @vertex fn main() -> @builtin(position) vec4f {
            return textureSampleLevel(image, sampling, vec2f(0), 0)
                + textureSampleLevel(image, other, vec2f(0), 0);
        }
    "#,
    );
    let words = write(&mut combined_writer(), &module).unwrap();
    let module = rspirv::dr::load_words(words).unwrap();
    assert_eq!(
        module
            .all_inst_iter()
            .filter(|i| i.class.opcode == spirv::Op::SampledImage)
            .count(),
        1
    );
}

#[test]
fn reject_unsupported_combined_types() {
    for (image_type, sampler_type) in [
        ("texture_depth_2d", "sampler"),
        ("texture_multisampled_2d<f32>", "sampler"),
        ("texture_2d<f32>", "sampler_comparison"),
    ] {
        let module = naga::front::wgsl::parse_str(&format!(
            r#"
            @group(0) @binding(0) var image: {image_type};
            @group(0) @binding(1) var sampling: {sampler_type};
            @vertex fn main() -> @builtin(position) vec4f {{
                return vec4f(0);
            }}
        "#
        ))
        .unwrap();
        assert!(matches!(
            write(&mut combined_writer(), &module),
            Err(spv::Error::Validation(_))
        ));
    }
}
