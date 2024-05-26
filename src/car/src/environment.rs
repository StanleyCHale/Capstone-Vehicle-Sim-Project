#![allow(unused_imports)]

use std::f32::consts::PI;

use bevy::{
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap, ExtendedMaterial}, 
    prelude::*
};

use grid_terrain::{
    examples::{perlin_plane, TerrainPreferences}, GridTerrain, MyExtension
};

use grid_terrain::PLANESIZE;

pub fn build_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, MyExtension>>>,
    terrain_preferences: ResMut<TerrainPreferences>,
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
    let size = terrain_preferences.grid_size;

    let perlin_elements = perlin_plane(terrain_preferences);

    // merge the two grid terrains    
    // Change to the below comment if more elements are being added
    let elements = perlin_elements;
    // elements.extend(wave_elements);

    let grid_terrain = GridTerrain::new(elements, [size as f64, size as f64]);
    let empty_parent = commands.spawn(SpatialBundle::default()).id();

    grid_terrain.build_meshes(&mut commands, &mut meshes, &mut materials, empty_parent);
    commands.insert_resource(grid_terrain);
}
