#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

struct MyExtendedMaterial {
    zmax: f32,
}

@group(1) @binding(100)
var<uniform> my_extended_material: MyExtendedMaterial;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    // Change the color to be based off of height
    let height_percentage = (in.world_position.z + my_extended_material.zmax) / (my_extended_material.zmax * 2.0);

    let color_deep = vec4<f32>(0.27, 0.2, 0.8, 1.0); // Blue for deep heights
    let color_low = vec4<f32>(0.55, 0.75, 0.8, 1.0); // LightBlue for lower heights
    let color_mid = vec4<f32>(0.15, 0.7, 0.13, 1.0); // Green for mid heights
    let color_highmid = vec4<f32>(0.6, 0.65, 0.35, 1.0); // YellowGreen for highmid heights
    let color_high = vec4<f32>(0.93, 0.92, 0.97, 1.0); // White mountain tops

    if (height_percentage < 0.1) { 
        pbr_input.material.base_color = color_deep;
    } 
    else if (height_percentage < 0.3) { 
        pbr_input.material.base_color = mix(color_deep, color_low, (height_percentage - 0.1) / 0.2);
    } 
    else if (height_percentage < 0.5) {
        pbr_input.material.base_color = mix(color_low, color_mid, (height_percentage - 0.3) / 0.2);
    } 
    else if (height_percentage < 0.85) {
        pbr_input.material.base_color = mix(color_mid, color_highmid, (height_percentage - 0.5) / 0.35);
    } 
    else if (height_percentage < 0.95) {
        pbr_input.material.base_color = mix(color_highmid, color_high, (height_percentage - 0.85) / 0.1);
    } 
    else {
        pbr_input.material.base_color = color_high;
    }

    // alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE
    // in deferred mode we can't modify anything after that, as lighting is run in a separate fullscreen shader.
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    // apply lighting
    out.color = apply_pbr_lighting(pbr_input);

#endif

    return out;
}