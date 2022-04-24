// Copyright 2020-2022 Kevin Reid under the terms of the MIT License as detailed
// in the accompanying file README.md or <https://opensource.org/licenses/MIT>.

// --- Interface declarations --------------------------------------------------

// Mirrors `struct WgpuCamera` on the Rust side.
struct WgpuCamera {
    [[location(0)]] projection: mat4x4<f32>;
    [[location(1)]] view_matrix: mat4x4<f32>;
    [[location(2)]] view_position: vec3<f32>;
    [[location(3)]] light_lookup_offset_and_option: vec4<i32>;
    [[location(4)]] fog_color: vec3<f32>;
    [[location(5)]] fog_mode_blend: f32;
    [[location(6)]] fog_distance: f32;
    [[location(7)]] exposure: f32;
};

// Mirrors `struct WgpuBlockVertex` on the Rust side.
struct WgpuBlockVertex {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] cube: vec3<f32>;
    [[location(2)]] normal: vec3<f32>;
    [[location(3)]] color_or_texture: vec4<f32>;
    [[location(4)]] clamp_min: vec3<f32>;
    [[location(5)]] clamp_max: vec3<f32>;
};

// Mirrors `struct WgpuLinesVertex` on the Rust side.
struct WgpuLinesVertex {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] color: vec4<f32>;
};

// This group is named camera_bind_group_layout in the code.
[[group(0), binding(0)]]
var<uniform> camera: WgpuCamera;

// This group is named space_texture_bind_group_layout in the code.
[[group(1), binding(0)]]
var block_texture: texture_3d<f32>;
[[group(1), binding(1)]]
var block_sampler: sampler;
[[group(1), binding(2)]]
var light_texture: texture_3d<u32>;

// --- Fog computation --------------------------------------------------------

// Physically realistic fog, but doesn't ever reach 1 (fully opaque).
fn fog_exponential(distance: f32) -> f32 {
    let fog_density = 1.6;
    return 1.0 - exp(-fog_density * distance);
}

// Fog that goes all the way from fully transparent to fully opaque.
// The correction is smaller the denser the fog.
fn fog_exp_fudged(distance: f32) -> f32 {
    return fog_exponential(distance) / fog_exponential(1.0);
}

fn fog_combo(distance: f32) -> f32 {
    // Combination of realistic exponential (constant density) fog,
    // and slower-starting fog so nearby stuff is clearer.
    return mix(fog_exp_fudged(distance), pow(distance, 4.0), camera.fog_mode_blend);
}

// Returns the opacity (0 to 1) of the fog.
//
// Note: This function is run in the vertex shader, to reduce the cost of the
// computation. This is an approximation that works on the assumption that fog
// is locally close-to-linear on the scale of the largest triangles we draw.
// This assumption will need to be revisited if we start using triangles larger
// than a block.
fn compute_fog(world_position: vec3<f32>) -> f32 {
    // Camera-relative position not transformed by projection.
    let eye_vertex_position = camera.view_matrix * vec4<f32>(world_position, 1.0);
    let distance_from_eye: f32 = length(eye_vertex_position.xyz);

    // TODO: When we implement volumetric transparency, that's another use
    // for the distance_from_eye value, which we will want to pass out (in a struct)

    // Distance in range 0 (camera position) to 1 (opaque fog position/far clip position).
    let normalized_distance: f32 = distance_from_eye / camera.fog_distance;
    let fog_mix = clamp(fog_combo(normalized_distance), 0.0, 1.0);

    return fog_mix;
}

// --- Block vertex shader ----------------------------------------------------

// Vertex-to-fragment data for blocks
struct BlockFragmentInput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec3<f32>;
    [[location(1)]] cube: vec3<f32>;
    [[location(2)]] normal: vec3<f32>;
    [[location(3)]] color_or_texture: vec4<f32>;
    [[location(4)]] clamp_min: vec3<f32>;
    [[location(5)]] clamp_max: vec3<f32>;
    [[location(6)]] fog_mix: f32;
};

[[stage(vertex)]]
fn block_vertex_main(
    input: WgpuBlockVertex,
) -> BlockFragmentInput {
    return BlockFragmentInput(
        camera.projection * camera.view_matrix * vec4<f32>(input.position, 1.0),
        input.position,
        input.cube,
        input.normal,
        input.color_or_texture,
        input.clamp_min,
        input.clamp_max,
        compute_fog(input.position),
    );
}

// --- Block fragment shader ---------------------------------------------------

// Modulo, not remainder (matches GLSL builtin mod())
fn mod(a: f32, b: f32) -> f32 {
    return ((a % b) + 1.0) % b;
}

// Given integer cube coordinates, fetch and unpack a light_texture RGB value.
// The alpha component corresponds to the `LightStatus` enum on the Rust side,
// but indirectly in a way that is useful for blending:
//
// LightStatus::Uninitialized = -1
// LightStatus::Opaque = 0
// LightStatus::NoRays = -1
// LightStatus::Visible = 1
// 
// This encoding allows use of the 0-1 range for smooth lighting's blending
// excluding opaque blocks, while the -1 value indicates values that should be
// truly ignored.
fn light_texture_fetch(fragment_position: vec3<f32>) -> vec4<f32> {
    var lookup_position = vec3<i32>(floor(fragment_position)) + camera.light_lookup_offset_and_option.xyz;
    
    // Implement wrapping (not automatic since we're not using a sampler).
    // Wrapping is used to handle sky light and in the future will be used for
    // circular buffering of the local light in an unbounded world.
    let size: vec3<i32> = textureDimensions(light_texture, 0);
    lookup_position = (lookup_position % size + size) % size;

    let texel: vec4<u32> = textureLoad(light_texture, lookup_position, 0);
    let packed_light = vec4<i32>(texel.rgb);

    // Decode logarithmic representation.
    // Exception: A texel value of exactly 0 is taken as 0, not the lowest power of 2.
    let not_zero: vec3<bool> = packed_light > vec3<i32>(0);
    let unpacked_light: vec3<f32> =
        pow(vec3<f32>(2.0), vec3<f32>(packed_light - 128) / 16.0)
        * vec3<f32>(not_zero);

    // See all_is_cubes::space::LightStatus for the value this is interpreting.
    // The enum values are grouped into approximately {0, 128, 255}, so multiplying by 2 and
    // rounding produces -1, 0, and 1 without any conditionals.
    // TODO: Now that we're reading integer values, this is unnecessarily circuitous
    let status: f32 = round((f32(texel.a) / 255.0) * 2.0 - 1.0);

    // TODO: Return a struct instead
    return vec4<f32>(unpacked_light, status);
}

// Simple directional lighting used to give corners extra definition.
// Note that this algorithm is also implemented in the Rust code.
fn fixed_directional_lighting(normal: vec3<f32>) -> f32 {
  let light_1_direction = vec3<f32>(0.4, -0.1, 0.0);
  let light_2_direction = vec3<f32>(-0.4, 0.35, 0.25);
  return (1.0 - 1.0 / 16.0) + 0.25 * (max(0.0, dot(light_1_direction, normal)) + max(0.0, dot(light_2_direction, normal)));
}

fn valid_light(light: vec4<f32>) -> bool {
  return light.a > 0.5;
}

// Tweak a light value for ambient occlusion -- and convert the light status 
// value returned from light_texture_fetch to an interpolation coefficient.
fn ao_fudge(light_value: vec4<f32>) -> vec4<f32> {
  // TODO: Make this a (uniform) graphics option
  let fudge = 0.25;
  let status = light_value.a;
  // Fudge applies only to opaque cubes, not to no-rays cubes.
  // This multiplication provides a branchless calculation:
  // If status is -1 (no-rays or uninitialized), return 0.
  // If status is 0 (opaque), return fudge value.
  // If status is 1 (normal light value), return that.
  return vec4<f32>(light_value.rgb, f32(status > -0.5) * max(status, fudge));
}

// Compute the interpolated ('smooth') light for the surface from light_texture.
// This implementation is duplicated in Rust at all-is-cubes/src/raytracer.rs
fn interpolated_space_light(in: BlockFragmentInput) -> vec3<f32> {
    // Choose two vectors that are perpendicular to each other and the normal,
    // and in the positive direction on that axis.
    // Assumes in.normal is an axis-aligned unit vector.
    var v_perpendicular_1: vec3<f32>;
    var v_perpendicular_2: vec3<f32>;
    if (in.normal.x != 0.0) {
        v_perpendicular_1 = vec3<f32>(0.0, 1.0, 0.0);
        v_perpendicular_2 = vec3<f32>(0.0, 0.0, 1.0);
    } else {
        v_perpendicular_1 = vec3<f32>(1.0, 0.0, 0.0);
        v_perpendicular_2 = abs(cross(v_perpendicular_1, in.normal));
    }

    // About half the size of the smallest permissible voxel.
    let above_surface_epsilon = 0.5 / 256.0;

    // The position we should start with for texture lookup and interpolation.
    let origin = in.world_position + in.normal * above_surface_epsilon;

    // Find linear interpolation coefficients based on where we are relative to
    // a half-cube-offset grid.
    var mix_1: f32 = mod(dot(origin, v_perpendicular_1) - 0.5, 1.0);
    var mix_2: f32 = mod(dot(origin, v_perpendicular_2) - 0.5, 1.0);

    // Ensure that mix <= 0.5, i.e. the 'near' side below is the side we are on
    var dir_1: vec3<f32> = v_perpendicular_1;
    var dir_2: vec3<f32> = v_perpendicular_2;
    if (mix_1 > 0.5) {
        dir_1 = dir_1 * -1.0;
        mix_1 = 1.0 - mix_1;
    }
    if (mix_2 > 0.5) {
        dir_2 = dir_2 * -1.0;
        mix_2 = 1.0 - mix_2;
    }

    // Modify interpolation by smoothstep to change the visual impression towards
    // "blurred blocks" and away from the diamond-shaped gradients of linear interpolation
    // which, being so familiar, can give an unfortunate impression of "here is 
    // a closeup of a really low-resolution texture".
    // TODO: disabled because current wgpu doesn't implement smoothstep
    // mix_1 = smoothstep(0.0, 1.0, mix_1);
    // mix_2 = smoothstep(0.0, 1.0, mix_2);

    // Retrieve texels, again using the half-cube-offset grid (this way we won't have edge artifacts).
    let lin_lo = -0.5;
    let lin_hi = 0.5;
    var near12    = light_texture_fetch(origin + lin_lo * dir_1 + lin_lo * dir_2);
    var near1far2 = light_texture_fetch(origin + lin_lo * dir_1 + lin_hi * dir_2);
    var near2far1 = light_texture_fetch(origin + lin_hi * dir_1 + lin_lo * dir_2);
    var far12     = light_texture_fetch(origin + lin_hi * dir_1 + lin_hi * dir_2);
    
    if (!valid_light(near1far2) && !valid_light(near2far1)) {
        // The far corner is on the other side of a diagonal wall, so should be
        // ignored to prevent light leaks.
        far12 = near12;
    }

    // Apply ambient occlusion.
    near12    = ao_fudge(near12);
    near1far2 = ao_fudge(near1far2);
    near2far1 = ao_fudge(near2far1);
    far12     = ao_fudge(far12);

    // Perform bilinear interpolation.
    let v = mix(
        mix(near12,    near1far2, mix_2),
        mix(near2far1, far12,     mix_2),
        mix_1
    );
    // Scale result by sum of valid texels.
    // Because v.a went through the mix, it scales with the proportion of valid texels
    // that were used, so it is always a smooth blend without block edge effects.
    // However, we don't want divide-by-a-small-number effects so we cap the divisor.
    return v.rgb / max(0.1, v.a);
}

// Compute light intensity applying to the fragment.
fn lighting(in: BlockFragmentInput) -> vec3<f32> {
    switch (camera.light_lookup_offset_and_option.w) {
        // LightingOption::None or fallback: no lighting
        default: {
            return vec3<f32>(1.0);
        }
        
        // LightingOption::Flat
        case 1: {
            let origin = in.cube + in.normal + vec3<f32>(0.5);
            let local_light = light_texture_fetch(origin).rgb;
            return fixed_directional_lighting(in.normal) * local_light;
        }

        // LightingOption::Smooth
        case 2: {
            return fixed_directional_lighting(in.normal) * interpolated_space_light(in);
        }
    }
}

// Get the vertex color or texel value to display
fn get_diffuse_color(in: BlockFragmentInput) -> vec4<f32> {
    if (in.color_or_texture[3] < -0.5) {
        // Texture coordinates.
        let texcoord: vec3<f32> =
            clamp(in.color_or_texture.xyz, in.clamp_min, in.clamp_max);
        return textureSampleLevel(block_texture, block_sampler, texcoord, 0.0);

        // TODO: implement DEBUG_TEXTURE_EDGE
    } else {
        // Solid color.
        return in.color_or_texture;
    }
}

[[stage(fragment)]]
fn block_fragment_opaque(in: BlockFragmentInput) -> [[location(0)]] vec4<f32> {
    let diffuse_color: vec4<f32> = get_diffuse_color(in);
    
    // Lighting
    let lit_color = diffuse_color * vec4<f32>(lighting(in), 1.0);

    // Fog
    let fogged_color = vec4<f32>(mix(lit_color.rgb, camera.fog_color, in.fog_mix), lit_color.a);

    // Exposure/eye adaptation
    let exposed_color = vec4<f32>(fogged_color.rgb * camera.exposure, fogged_color.a);

    return exposed_color;
}

[[stage(fragment)]]
fn block_fragment_transparent(in: BlockFragmentInput) -> [[location(0)]] vec4<f32> {
    let diffuse_color: vec4<f32> = get_diffuse_color(in);
    
    // Lighting
    let lit_color = diffuse_color * vec4<f32>(lighting(in), 1.0);

    // Fog
    let fogged_color = vec4<f32>(mix(lit_color.rgb, camera.fog_color, in.fog_mix), lit_color.a);

    // Exposure/eye adaptation
    let exposed_color = vec4<f32>(fogged_color.rgb * camera.exposure, fogged_color.a);

    // Multiply color channels by alpha because our blend function choice is premultiplied alpha.
    return vec4<f32>(exposed_color.rgb * exposed_color.a, exposed_color.a);
}

// --- Lines shader ------------------------------------------------------------
//
// This is in the same shader source file as the block shader so that it can share
// the camera struct and uniform.

// Vertex-to-fragment data for lines
struct LinesFragmentInput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
    [[location(1)]] fog_mix: f32;
};

[[stage(vertex)]]
fn lines_vertex(
    input: WgpuLinesVertex,
) -> LinesFragmentInput {
    return LinesFragmentInput(
        camera.projection * camera.view_matrix * vec4<f32>(input.position, 1.0),
        input.color,
        compute_fog(input.position),
    );
}

[[stage(fragment)]]
fn lines_fragment(input: LinesFragmentInput) -> [[location(0)]] vec4<f32> {
    let color = input.color;
    
    // Fog
    let fogged_color = vec4<f32>(mix(color.rgb, camera.fog_color, input.fog_mix), color.a);

    return fogged_color;
}