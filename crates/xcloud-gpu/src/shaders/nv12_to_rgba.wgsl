// NV12 -> RGBA8 colour conversion.
//
// Render-pipeline variant for old GPUs (Intel HD 3000 / Mesa GL 3.3) that do not
// expose storage textures. The compute variant wrote to a texture_storage_2d;
// here the same maths runs in a fragment shader writing a colour attachment.
// Fullscreen triangle, so no vertex buffer is needed.

struct ConvertParams {
    p0: vec4<f32>,
    p1: vec4<f32>,
};

@group(0) @binding(0) var y_plane: texture_2d<f32>;
@group(0) @binding(1) var uv_plane: texture_2d<f32>;
@group(0) @binding(2) var uv_sampler: sampler;
@group(0) @binding(3) var<uniform> params: ConvertParams;

@vertex
fn vs_fullscreen(@builtin(vertex_index) vi: u32) -> @builtin(position) vec4<f32> {
    var verts = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0),
    );
    return vec4<f32>(verts[vi], 0.0, 1.0);
}

@fragment
fn convert_video(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let xy = vec2<u32>(pos.xy);
    let size = vec2<u32>(textureDimensions(y_plane));
    let l = (textureLoad(y_plane, xy, 0).r - params.p0.x) * params.p0.y;
    let uv_coord = (vec2<f32>(xy) + vec2<f32>(0.5, 0.5)) / vec2<f32>(size);
    let c = (textureSampleLevel(uv_plane, uv_sampler, uv_coord, 0.0).rg - vec2<f32>(0.5, 0.5)) * params.p0.z;
    let color = vec3<f32>(
        l + params.p1.x * c.y,
        l + params.p1.y * c.x + params.p1.z * c.y,
        l + params.p1.w * c.x,
    );
    return vec4<f32>(clamp(color, vec3<f32>(0.0), vec3<f32>(1.0)), 1.0);
}
