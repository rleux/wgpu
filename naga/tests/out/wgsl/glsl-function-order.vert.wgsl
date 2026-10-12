struct VertexOutput {
    @builtin(position) gl_Position: vec4<f32>,
}

var<private> gl_Position: vec4<f32>;

fn later(x: f32) -> f32 {
    var x_1: f32;

    x_1 = x;
    let _e2 = x_1;
    return (_e2 + 1f);
}

fn first(x_2: f32) -> f32 {
    var x_3: f32;
    var i: i32 = 0i;

    x_3 = x_2;
    let _e2 = x_3;
    if (_e2 > 0f) {
        {
            let _e5 = x_3;
            let _e6 = later(_e5);
            return _e6;
        }
    }
    loop {
        let _e9 = i;
        if !((_e9 < 2i)) {
            break;
        }
        {
            let _e16 = x_3;
            let _e17 = i;
            let _e19 = later(f32(_e17));
            x_3 = (_e16 + _e19);
        }
        continuing {
            let _e13 = i;
            i = (_e13 + 1i);
        }
    }
    let _e21 = x_3;
    switch i32(_e21) {
        case 0: {
            let _e24 = later(2f);
            x_3 = _e24;
        }
        default: {
        }
    }
    let _e25 = x_3;
    return _e25;
}

fn main_1() {
    let _e2 = first(1f);
    gl_Position = vec4(_e2);
    return;
}

@vertex
fn main() -> VertexOutput {
    main_1();
    let _e1 = gl_Position;
    return VertexOutput(_e1);
}
