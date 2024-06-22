@vertex
fn main(@location(0) position: vec3<f32>, @builtin(position) out_pos: vec4<f32>) -> void {
    out_pos = vec4<f32>(position, 1.0);
}