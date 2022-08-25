#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting

fn bill_point_light(
    world_position: vec3<f32>, light: PointLight, roughness: f32, NdotV: f32, N: vec3<f32>, V: vec3<f32>,
    R: vec3<f32>, F0: vec3<f32>, diffuseColor: vec3<f32>
) -> vec3<f32> {
    let light_to_frag = light.position_radius.xyz - world_position.xyz;
    let distance_square = dot(light_to_frag, light_to_frag);
    let rangeAttenuation = getDistanceAttenuation(distance_square, light.color_inverse_square_range.w);
    return vec3(rangeAttenuation * diffuseColor * 0.01 * light.color_inverse_square_range.rgb);
}

struct CustomMaterial {
    color: vec4<f32>,
};

struct FragmentInput {
    @builtin(position) frag_coord: vec4<f32>,

    // uv in here
    #import bevy_pbr::mesh_vertex_output
}

@group(1) @binding(0)
var<uniform> material: CustomMaterial;
@group(1) @binding(1)
var<uniform> owner: i32;
@group(1) @binding(2)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(3)
var base_color_sampler: sampler;

// Lots of this code is reused from `pbr()` in pbr_functions.wgsl.
@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    let roughness = 0.1;

    let N = vec3<f32>(0.0, 0.0, 1.0);
    let V = vec3<f32>(1.0, 0.0, 0.0);

    // Neubelt and Pettineo 2013, "Crafting a Next-gen Material Pipeline for The Order: 1886"
    let NdotV = max(dot(N, V), 0.0001);
    let R = reflect(-V, N);
    let F0 = vec3(0.5);
    let diffuse_color = vec3<f32>(1.0, 1.0, 1.0);

    var light_accum: vec3<f32> = vec3<f32>(0.00);

    let view_z = dot(vec4<f32>(
        view.inverse_view[0].z,
        view.inverse_view[1].z,
        view.inverse_view[2].z,
        view.inverse_view[3].z
    ), in.world_position);
    let cluster_index = fragment_cluster_index(in.frag_coord.xy, view_z, false);
    let offset_and_counts = unpack_offset_and_counts(cluster_index);

    for (var i: u32 = offset_and_counts[0]; i < offset_and_counts[0] + offset_and_counts[1]; i = i + 1u) {
        let light_id = get_light_id(i);
        let light = point_lights.data[light_id];
        let light_contrib = bill_point_light(in.world_position.xyz, light, roughness, NdotV, N, V, R, F0, diffuse_color);
        light_accum = light_accum + light_contrib;
    }

    var light_colour = vec4<f32>(light_accum, 1.0) + vec4(lights.ambient_color.rgb, 1.0);

    var sampled = textureSample(base_color_texture, base_color_sampler, in.uv);

    // If the colour is #ff0000 we change it to the team colour.
    if (sampled.r == 1.0 && sampled.g == 0.0 && sampled.b == 0.0) {
        if (owner == 0) {

            sampled = vec4<f32>(0.956862745, 0.850980392, 0.552941176, sampled.a);
        } else if (owner == 1) {
            sampled = vec4<f32>(0.0, 0.0, 1.0, sampled.a);
        }
    }

    return light_colour * sampled;
}

