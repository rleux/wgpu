struct FragmentOutput {
    @location(1) color: vec4<f32>,
}

var<private> color: vec4<f32>;

fn main_1() {
    color = vec4(1f);
    return;
}

@fragment
fn main() -> FragmentOutput {
    main_1();
    let _e1 = color;
    return FragmentOutput(_e1);
}
