use std::f32::consts::PI;

use bevy::{
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap, ExtendedMaterial}, 
    prelude::*
};

use grid_terrain::{
    //examples::{perlin_plane, steps, table_top, wave}, 
    examples::perlin_plane,
    MyExtension,
    GridTerrain,
};

use grid_terrain::PLANESIZE;

pub fn build_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, MyExtension>>>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.9, 0.9, 1.0),
        brightness: 0.4,
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 10000.0, // lux
            shadow_depth_bias: 0.3,
            shadow_normal_bias: 1.0,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 10.0),
            rotation: Quat::from_rotation_x(-PI / 4.) * Quat::from_rotation_y(-PI / 4.),

            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 4,
            minimum_distance: 1.,
            maximum_distance: 300.0,
            first_cascade_far_bound: 5.0,
            overlap_proportion: 0.3,
        }
        .into(),

        ..default()
    });

    commands.insert_resource(DirectionalLightShadowMap { size: 4 * 1024 });
    
    // must be the same for all grid elements
    let size = PLANESIZE; 

    //let height = 2.;
    //let table_elements = table_top(size as f64, height);
    //let height = 0.3;
    //let wave_length = 4.;
    //let wave_elements = wave(size, height, wave_length);
    //let step_elements = steps(size, vec![0.2, 0.4, 0.6]);

    let perlin_elements = perlin_plane(size as f64, 1024.0);

    // merge the two grid terrains    
    // Change to below if more elements are being added
    // let mut elements
    let elements =  perlin_elements; 
    //elements.extend(wave_elements);

    let grid_terrain = GridTerrain::new(elements, [size as f64, size as f64]);
    let empty_parent = commands.spawn(SpatialBundle::default()).id();

    grid_terrain.build_meshes(&mut commands, &mut meshes, &mut materials, empty_parent);
    commands.insert_resource(grid_terrain);
}
