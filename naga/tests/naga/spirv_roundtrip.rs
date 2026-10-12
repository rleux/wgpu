fn words_to_bytes(words: &[u32]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(words.len() * 4);
    for word in words {
        bytes.extend_from_slice(&word.to_le_bytes());
    }
    bytes
}

#[test]
fn atomic_compare_exchange_roundtrip_to_spirv() {
    let source = include_str!("../in/wgsl/atomicCompareExchange.wgsl");
    let module = naga::front::wgsl::parse_str(source).unwrap();

    let mut validator = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    );
    let info = validator.validate(&module).unwrap();

    let pipeline_options = naga::back::spv::PipelineOptions {
        entry_point: "test_atomic_compare_exchange_u32".to_string(),
        shader_stage: naga::ShaderStage::Compute,
    };
    let options = naga::back::spv::Options::default();
    let spv_words =
        naga::back::spv::write_vec(&module, &info, &options, Some(&pipeline_options)).unwrap();

    let spv_bytes = words_to_bytes(&spv_words);
    let parsed = naga::front::spv::parse_u8_slice(
        &spv_bytes,
        &naga::front::spv::Options {
            adjust_coordinate_space: true,
            strict_capabilities: false,
            block_ctx_dump_prefix: None,
        },
    )
    .unwrap();

    let parsed_info = validator.validate(&parsed).unwrap();
    naga::back::spv::write_vec(&parsed, &parsed_info, &options, Some(&pipeline_options)).unwrap();
}

#[test]
fn glsl_scalar_field_read_next_to_large_array() {
    // Reconstructing the unused array would require 65,536 words in one
    // OpCompositeConstruct, exceeding SPIR-V's 16-bit instruction word count.
    let module = naga::front::glsl::Frontend::default()
        .parse(
            &naga::front::glsl::Options::from(naga::ShaderStage::Vertex),
            r#"
                #version 450
                struct Data { float unused[65533]; float value; };
                layout(set = 0, binding = 0, std140) readonly buffer Buffer {
                    Data data;
                } buffer_data;
                void main() {
                    gl_Position = vec4(buffer_data.data.value);
                }
            "#,
        )
        .unwrap();

    let info = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::all(),
    )
    .validate(&module)
    .unwrap();
    let pipeline_options = naga::back::spv::PipelineOptions {
        entry_point: "main".to_string(),
        shader_stage: naga::ShaderStage::Vertex,
    };
    let words = naga::back::spv::write_vec(
        &module,
        &info,
        &naga::back::spv::Options::default(),
        Some(&pipeline_options),
    )
    .unwrap();

    rspirv::dr::load_words(&words).expect("scalar field read produced malformed SPIR-V");
}
