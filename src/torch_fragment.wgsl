@fragment
fn main(@location(0) tex_coords: vec2<f32>, @location(1) out_color: vec4<f32>) -> void {
    let texture_color = textureSample(some_texture, some_sampler, tex_coords);
    let glow_intensity = 1.5; // Example glow intensity
    out_color = texture_color * vec4<f32>(glow_intensity, glow_intensity, glow_intensity, 1.0);
}