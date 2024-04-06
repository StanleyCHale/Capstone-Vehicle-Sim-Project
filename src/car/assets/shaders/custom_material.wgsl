#import bevy_pbr::mesh_functions::{get_model_matrix, mesh_position_local_to_clip}


struct CustomMaterial {
    color: vec4<f32>,
};
@group(1) @binding(0) var<uniform> material: CustomMaterial;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) blend_color: vec4<f32>,
};


@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    var scale = 1.0;
    var newy = sin(vertex.position.x * 4.0) / scale;
    var ymax = 1.0 / scale;

    out.clip_position = mesh_position_local_to_clip(
        get_model_matrix(vertex.instance_index),
        //vec4<f32>(vertex.position.x, newy / 2.0, vertex.position.z, 1.0),
        vec4<f32>(vertex.position.x, vertex.position.y, vertex.position.z, 1.0),
    );
    //out.blend_color = vertex.blend_color;
    out.blend_color = vec4<f32>(0.2, newy / ymax, newy / ymax, 1.0);
    return out;
}

struct FragmentInput {
    @location(0) blend_color: vec4<f32>,
};

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    return material.color * input.blend_color;
}