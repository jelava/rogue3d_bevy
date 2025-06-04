#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var lut_texture: texture_2d<f32>;
//@group(0) @binding(2) var dither_texture: texture_2d<f32>;
@group(0) @binding(2) var tex_sampler: sampler;

const DOWNSCALE_AMOUNT = 3.0;

/*
const BAYER = mat2x2<f32>(
    -2.0,  0.0,
     1.0, -1.0
) / 4.0;
*/

/*
const BAYER = mat4x4<f32>(
    -8.0,  0.0, -6.0,  2.0,
     4.0, -4.0,  6.0, -2.0,
    -5.0,  3.0, -7.0,  1.0,
     7.0, -1.0,  5.0, -3.0
) / 16.0;
*/

const BAYER = mat4x4<f32>(
    -0.39019607843137255, -0.20588235294117646, 0.11176470588235299, 0.4254901960784314,
    0.00588235294117645, 0.3901960784313725, -0.3509803921568627, -0.46862745098039216,
    0.23725490196078436, -0.033333333333333326, 0.27254901960784317, -0.10784313725490197,
    -0.4215686274509804, -0.296078431372549, -0.17058823529411765, 0.4882352941176471
);

const PALETTE = array<vec4<f32>, 59>(
    vec4(1.0, 0.30196078431372547, 0.0, 1.0),
    vec4(0.984313725490196, 0.6078431372549019, 0.4470588235294118, 1.0),
    vec4(0.8823529411764706, 0.4823529411764706, 0.3137254901960784, 1.0),
    vec4(0.8705882352941177, 0.9764705882352941, 0.9882352941176471, 1.0),
    vec4(0.8431372549019608, 0.4823529411764706, 0.7294117647058823, 1.0),
    vec4(0.8313725490196079, 0.09803921568627451, 0.0, 1.0),
    vec4(0.8274509803921568, 0.8509803921568627, 0.6980392156862745, 1.0),
    vec4(0.803921568627451, 0.8196078431372549, 0.9254901960784314, 1.0),
    vec4(0.7294117647058823, 0.6392156862745098, 0.40784313725490196, 1.0),
    vec4(0.6588235294117647, 0.2549019607843137, 0.06666666666666667, 1.0),
    vec4(0.6470588235294118, 0.7098039215686275, 0.8313725490196079, 1.0),
    vec4(0.6, 0.6235294117647059, 0.5568627450980392, 1.0),
    vec4(0.5607843137254902, 0.28627450980392155, 0.4588235294117647, 1.0),
    vec4(0.5098039215686274, 0.6392156862745098, 0.40784313725490196, 1.0),
    vec4(0.4549019607843137, 0.5607843137254902, 0.5764705882352941, 1.0),
    vec4(0.43529411764705883, 0.3411764705882353, 0.33725490196078434, 1.0),
    vec4(0.3607843137254902, 0.5215686274509804, 0.3568627450980392, 1.0),
    vec4(0.9803921568627451, 0.9803921568627451, 0.5098039215686274, 1.0),
    vec4(0.2980392156862745, 0.38823529411764707, 0.8784313725490196, 1.0),
    vec4(0.2784313725490196, 0.12941176470588237, 0.2196078431372549, 1.0),
    vec4(0.23137254901960785, 0.4117647058823529, 0.5137254901960784, 1.0),
    vec4(0.19215686274509805, 0.19215686274509805, 0.29411764705882354, 1.0),
    vec4(0.1843137254901961, 0.11372549019607843, 0.35294117647058826, 1.0),
    vec4(0.1411764705882353, 0.29411764705882354, 0.22745098039215686, 1.0),
    vec4(0.12941176470588237, 0.11372549019607843, 0.11372549019607843, 1.0),
    vec4(0.11764705882352941, 0.10588235294117647, 0.5019607843137255, 1.0),
    vec4(0.10588235294117647, 0.043137254901960784, 0.043137254901960784, 1.0),
    vec4(0.10196078431372549, 0.07450980392156863, 0.058823529411764705, 1.0),
    vec4(0.09019607843137255, 0.2235294117647059, 0.2901960784313726, 1.0),
    vec4(0.07058823529411765, 0.15294117647058825, 0.30196078431372547, 1.0),
    vec4(0.06666666666666667, 0.12156862745098039, 0.07058823529411765, 1.0),
    vec4(0.047058823529411764, 0.07450980392156863, 0.09019607843137255, 1.0),
    vec4(0.027450980392156862, 0.7333333333333333, 0.4745098039215686, 1.0),
    vec4(0.0, 1.0, 0.5490196078431373, 1.0),
    vec4(0.996078431372549, 0.5098039215686274, 0.4392156862745098, 1.0),
    vec4(0.9882352941176471, 0.6588235294117647, 0.615686274509804, 1.0),
    vec4(0.9803921568627451, 0.9098039215686274, 0.596078431372549, 1.0),
    vec4(0.9372549019607843, 0.24313725490196078, 0.42745098039215684, 1.0),
    vec4(0.7098039215686275, 0.596078431372549, 0.6980392156862745, 1.0),
    vec4(0.6313725490196078, 0.3607843137254902, 0.3215686274509804, 1.0),
    vec4(0.5529411764705883, 0.4627450980392157, 0.5411764705882353, 1.0),
    vec4(0.5490196078431373, 0.41568627450980394, 0.33725490196078434, 1.0),
    vec4(0.5294117647058824, 0.2901960784313726, 0.0, 1.0),
    vec4(0.5058823529411764, 0.14901960784313725, 0.06666666666666667, 1.0),
    vec4(0.4627450980392157, 0.25882352941176473, 0.5411764705882353, 1.0),
    vec4(0.4392156862745098, 0.08627450980392157, 0.16862745098039217, 1.0),
    vec4(0.4117647058823529, 0.41568627450980394, 0.41568627450980394, 1.0),
    vec4(0.0, 0.9294117647058824, 1.0, 1.0),
    vec4(0.35294117647058826, 0.34509803921568627, 0.4588235294117647, 1.0),
    vec4(0.27058823529411763, 0.21176470588235294, 0.1450980392156863, 1.0),
    vec4(0.25882352941176473, 0.25882352941176473, 0.5607843137254902, 1.0),
    vec4(0.20392156862745098, 0.5803921568627451, 0.796078431372549, 1.0),
    vec4(0.19607843137254902, 0.23529411764705882, 0.2235294117647059, 1.0),
    vec4(0.13333333333333333, 0.12549019607843137, 0.20392156862745098, 1.0),
    vec4(0.10980392156862745, 0.0784313725490196, 0.1607843137254902, 1.0),
    vec4(0.0784313725490196, 0.3764705882352941, 0.36470588235294116, 1.0),
    vec4(0.06274509803921569, 0.08627450980392157, 0.1803921568627451, 1.0),
    vec4(0.058823529411764705, 0.09411764705882353, 0.2, 1.0),
    vec4(0.0, 0.0, 0.0, 1.0),
);

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

fn interleavedGradientNoise(uv: vec2<f32>) -> f32 {
    let magic = vec3(0.06711056, 0.00583715, 52.9829189);
    return fract(magic.z * fract(dot(uv, magic.xy)));
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let resolution = vec2<f32>(textureDimensions(screen_texture));
    let downscale_factor = resolution / DOWNSCALE_AMOUNT;
    let downscaled_uv = floor(in.uv * downscale_factor) / downscale_factor;

    let srgb_screen_color = textureSample(screen_texture, tex_sampler, downscaled_uv);
    let screen_color = vec4(lin2srgb(srgb_screen_color.rgb), 1.0);

    //var grad = in.uv.x;
    //var fade = fract(in.uv.y * 3.0);
    //var sscreen_color = vec4(in.uv.x, 1.0 - in.uv.x, fract(in.uv.y * 4.0), 1.0);

    //if in.uv.x > 0.5 {
    //    grad = (in.uv.x - 0.5) * 2.0;
    //    sscreen_color = vec4(0.0, fade * (1.0 - grad), fade * grad, 1.0);
    //}

    //let screen_color = vec4(lin2srgb(sscreen_color.rgb), 1.0);

    let lut_size = vec2<f32>(textureDimensions(lut_texture));
    let lut_texel = 1.0 / lut_size.xy;

    let r = floor(screen_color.r * lut_size.x) * lut_texel.x;
    let g = floor(screen_color.g * lut_size.x) * lut_texel.y;
    let b = floor(screen_color.b * lut_size.x) * lut_texel.x;

    /*
    let bp = vec2<u32>(downscaled_uv * downscale_factor) % 4;
    let m = BAYER[bp.x][bp.y];

    // dithering of input color
    var dr = clamp(screen_color.r + (m * lut_texel.x), 0.0, (lut_size.x - 1.0) * lut_texel.x);
    var dg = clamp(screen_color.g + (m * lut_texel.x), 0.0, (lut_size.x - 1.0) * lut_texel.x);
    var db = clamp(screen_color.b + (m * lut_texel.x), 0.0, (lut_size.x - 1.0) * lut_texel.x);
    */

    let noise = interleavedGradientNoise(downscaled_uv * downscale_factor) * lut_texel.x;

    var dr = clamp(screen_color.r + noise, 0.0, (lut_size.x - 1.0) * lut_texel.x);
    var dg = clamp(screen_color.g + noise, 0.0, (lut_size.x - 1.0) * lut_texel.x);
    var db = clamp(screen_color.b + noise, 0.0, (lut_size.x - 1.0) * lut_texel.x);

    // convert dithered color to index for palette LUT
    dr = floor(dr * lut_size.x) * lut_texel.x;
    dg = floor(dg * lut_size.x) * lut_texel.y;
    db = floor(db * lut_size.x) * lut_texel.x;

    // slight offset added because sampling right on the edge of a texel can result in incorrect color lookups
    let undithered_lut_uv = vec2(r, g + b) + (0.5 * lut_texel);
    let dithered_lut_uv = vec2(dr, dg + db) + (0.5 * lut_texel);

    let undithered_lut_color = textureSample(lut_texture, tex_sampler, undithered_lut_uv);
    let dithered_lut_color = textureSample(lut_texture, tex_sampler, dithered_lut_uv);

    let d1 = distance(srgb_screen_color, undithered_lut_color);
    let d2 = distance(srgb_screen_color, dithered_lut_color);

    if d1 < d2 {
        return undithered_lut_color;
    } else {
        return dithered_lut_color;
    }

    /*
    if fract(4.0 * in.uv.x) < 4.0 / resolution.x {
        return vec4(0.0);
    } else if in.uv.x < 1.0 / 4.0 {
        return undithered_lut_color;
    } else if in.uv.x < 2.0 / 4.0 {
        let d1 = distance(srgb_screen_color, undithered_lut_color);
        let d2 = distance(srgb_screen_color, dithered_lut_color);

        if d1 < d2 {
            return undithered_lut_color;
        } else {
            return dithered_lut_color;
        }
    } else if in.uv.x < 3.0 / 4.0 {
        return dithered_lut_color;
    } else {
        return textureSample(screen_texture, tex_sampler, in.uv);
    }
    */

    /*
    if in.uv.y < 1.0 / 4.0 {
        return sscreen_color;
    } else if in.uv.y < 2.0 / 4.0 {
        let lut_uv = vec2<f32>(r, g + b) + (0.5 * lut_texel);
        let lut_color = textureSample(lut_texture, tex_sampler, lut_uv); //vec4(lin2srgb(textureSample(lut_texture, tex_sampler, lut_uv).rgb), 1.0);
        return lut_color;
    } else if in.uv.y < 3.0 / 4.0 {
        let lut_uv = vec2<f32>(dr, dg + db) + (0.5 * lut_texel);
        let lut_color = textureSample(lut_texture, tex_sampler, lut_uv); //vec4(lin2srgb(textureSample(lut_texture, tex_sampler, lut_uv).rgb), 1.0);
        return lut_color;
    } else {
        var min_dist = -1.0;
        var naive_color = vec4(0.0, 0.0, 0.0, 1.0);

        for (var i = 0; i < 59; i++) {
            let palette_color = vec4(srgb2lin(PALETTE[i].rgb), 1.0);
            let dist = distance(palette_color, sscreen_color);

            if dist < min_dist || min_dist < 0.0 {
                min_dist = dist;
                naive_color = palette_color;
            }
        }

        return naive_color;
    }
    */
}
