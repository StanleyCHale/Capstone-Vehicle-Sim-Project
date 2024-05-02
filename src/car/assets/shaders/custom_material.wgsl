#import bevy_pbr::{
    mesh_view_bindings::globals,
    mesh_view_bindings as view_bindings,
    mesh_functions::{get_model_matrix, mesh_position_local_to_clip},
    prepass_utils
}


struct CustomMaterial {
    color: vec4<f32>,
};
@group(1) @binding(0) var<uniform> material: CustomMaterial;
@group(1) @binding(1) var<uniform> zmax: f32;

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

    out.clip_position = mesh_position_local_to_clip(
        get_model_matrix(vertex.instance_index),
        vec4<f32>(vertex.position.x, vertex.position.y, vertex.position.z, 1.0),
    );
    
    //out.blend_color = vec4<f32>(1.0, vertex.position.z / zmax, -vertex.position.z / zmax, 1.0);

    let height_percentage = (vertex.position.z + zmax) / (zmax * 2.0);

    let color_deep = vec4<f32>(0.27, 0.2, 0.8, 1.0); // Blue for deep heights
    let color_low = vec4<f32>(0.55, 0.75, 0.8, 1.0); // LightBlue for lower heights
    let color_mid = vec4<f32>(0.15, 0.7, 0.13, 1.0); // Green for mid heights
    let color_highmid = vec4<f32>(0.6, 0.65, 0.35, 1.0); // YellowGreen for highmid heights
    let color_high = vec4<f32>(0.93, 0.92, 0.97, 1.0); // White mountain tops

    // Blend colors based on height
    // Low to High
    if (height_percentage < 0.1) { 
        out.blend_color = color_deep;
    } 
    else if (height_percentage < 0.3) { 
        out.blend_color = mix(color_deep, color_low, (height_percentage - 0.1) / 0.2);
    } 
    else if (height_percentage < 0.5) {
        out.blend_color = mix(color_low, color_mid, (height_percentage - 0.3) / 0.2);
    } 
    else if (height_percentage < 0.85) {
        out.blend_color = mix(color_mid, color_highmid, (height_percentage - 0.5) / 0.35);
    } 
    else if (height_percentage < 0.95) {
        out.blend_color = mix(color_highmid, color_high, (height_percentage - 0.85) / 0.1);
    } 
    else {
        out.blend_color = color_high;
    }

    return out;
}



struct FragmentInput {
    @location(0) blend_color: vec4<f32>,
};

@fragment
fn fragment( 
    input: FragmentInput
    ) -> @location(0) vec4<f32> {
        
    return material.color * input.blend_color;
}
