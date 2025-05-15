#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var lut_texture: texture_2d<f32>;
@group(0) @binding(2) var tex_sampler: sampler;

const DOWNSCALE_AMOUNT = 3.0;

const BAYER = mat4x4<f32>(
    -8.0,  0.0, -6.0,  2.0,
     4.0, -4.0,  6.0, -2.0,
    -5.0,  3.0, -7.0,  1.0,
     7.0, -1.0,  5.0, -3.0
) * 0.25;

/*
const BAYER = mat4x4<f32>(
    -2.0,  0.0, -2.0,  0.0,
     1.0, -1.0,  2.0, -0.0,
    -1.0,  1.0, -2.0,  0.0,
     2.0, -0.0,  1.0, -1.0
);
*/

fn srgb2lin(cs: vec3<f32>) -> vec3<f32> {
	let c_lo = cs / 12.92;
	let c_hi = pow( (cs + 0.055) / 1.055, vec3(2.4));
	let s = step(vec3(0.04045), cs);
	return mix(c_lo, c_hi, s);
}

fn lin2srgb(cl: vec3<f32>) -> vec3<f32> {
	let c_lo = 12.92 * cl;
	let c_hi = 1.055 * pow(cl, vec3(0.41666)) - 0.055;
	let s = step(vec3(0.0031308), cl);
	return mix(c_lo, c_hi, s);
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let resolution = vec2<f32>(textureDimensions(screen_texture));
    let downscale_factor = resolution / DOWNSCALE_AMOUNT;
    let downscaled_uv = floor(in.uv * downscale_factor) / downscale_factor;

    let srgb_screen_color = textureSample(screen_texture, tex_sampler, downscaled_uv);

    //var grad = in.uv.x * 2.0;
    //var fade = fract(in.uv.y * 3.0);
    //var screen_color = vec4(fade * (1.0 - grad), fade * grad, 0.0, 1.0);

    let screen_color = vec4(lin2srgb(srgb_screen_color.rgb), 1.0);

    /*
    if in.uv.x > 0.5 {
        grad = (in.uv.x - 0.5) * 2.0;
        screen_color = vec4(0.0, fade * (1.0 - grad), fade * grad, 1.0);
    }
    */

    let lut_size = vec2<f32>(textureDimensions(lut_texture));
    let lut_texel = 1.0 / lut_size.xy;

    let r = floor(screen_color.r * lut_size.x) * lut_texel.x;
    let g = floor(screen_color.g * lut_size.x) * lut_texel.y;
    let b = floor(screen_color.b * lut_size.x) * lut_texel.x;

    let bp = vec2<u32>(downscaled_uv * downscale_factor) % 4;
    let m = BAYER[bp.x][bp.y];

    // dithering of input color
    var dr = clamp(screen_color.r + (m * lut_texel.x), 0.0, (lut_size.x - 1.0) * lut_texel.x);
    var dg = clamp(screen_color.g + (m * lut_texel.x), 0.0, (lut_size.x - 1.0) * lut_texel.x);
    var db = clamp(screen_color.b + (m * lut_texel.x), 0.0, (lut_size.x - 1.0) * lut_texel.x);

    // convert dithered color to index for palette LUT
    dr = floor(dr * lut_size.x) * lut_texel.x;
    dg = floor(dg * lut_size.x) * lut_texel.y;
    db = floor(db * lut_size.x) * lut_texel.x;

    // slight offset added because sampling right on the edge of a texel can result in incorrect color lookups
    let lut_uv = vec2<f32>(dr, dg + db) + (0.5 * lut_texel);
    let lut_color = textureSample(lut_texture, tex_sampler, lut_uv);

    return lut_color;

    /*
    if in.uv.y < 1.0 / 3.0 {
        return srgb_screen_color;
    } else if in.uv.y < 2.0 / 3.0 {
        let lut_uv = vec2<f32>(r, g + b) + (0.5 * lut_texel);
        let lut_color = textureSample(lut_texture, tex_sampler, lut_uv);
        return lut_color;
    } else {
        let lut_uv = vec2<f32>(dr, dg + db) + (0.5 * lut_texel);
        let lut_color = textureSample(lut_texture, tex_sampler, lut_uv);
        return lut_color;
    }
    */
}
