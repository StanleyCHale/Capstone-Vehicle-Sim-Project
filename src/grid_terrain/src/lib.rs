pub mod examples;
pub mod function;
pub mod mirror;
pub mod perlin;
pub mod plane;
pub mod rotate;
pub mod slope;
pub mod step;
pub mod step_slope;

use mirror::Mirror;
use rigid_body::sva::Vector;
use rotate::{Rotate, RotationDirection};

use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension, OpaqueRendererMethod},
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef,},
};


pub const PLANESIZE: f32 = 160.0;


pub struct Interference {
    pub magnitude: f64,
    pub position: Vector,
    pub normal: Vector,
}

impl Interference {
    fn mirror(&mut self, size: f64, mirror: &Mirror) {
        match mirror {
            Mirror::None => {}
            Mirror::XZ => {
                self.position.y = size - self.position.y;
                self.normal.y = -self.normal.y;
            }
            Mirror::YZ => {
                self.position.x = size - self.position.x;
                self.normal.x = -self.normal.x;
            }
        }
    }
    fn rotate(&mut self, size: f64, rotate: &Rotate, direction: RotationDirection) {
        match (rotate, direction) {
            (Rotate::Zero, _) => {}
            (Rotate::Ninety, RotationDirection::Forward)
            | (Rotate::TwoSeventy, RotationDirection::Reverse) => {
                let x = self.position.x;
                let y = self.position.y;
                self.position.x = size - y;
                self.position.y = x;

                let x = self.normal.x;
                let y = self.normal.y;
                self.normal.x = -y;
                self.normal.y = x;
            }
            (Rotate::OneEighty, _) => {
                self.position.x = size - self.position.x;
                self.position.y = size - self.position.y;

                self.normal.x = -self.normal.x;
                self.normal.y = -self.normal.y;
            }
            (Rotate::TwoSeventy, RotationDirection::Forward)
            | (Rotate::Ninety, RotationDirection::Reverse) => {
                let x = self.position.x;
                let y = self.position.y;
                self.position.x = y;
                self.position.y = size - x;

                let x = self.normal.x;
                let y = self.normal.y;
                self.normal.x = y;
                self.normal.y = -x;
            }
        }
    }
}

pub trait GridElement {
    fn interference(&self, point: Vector) -> Option<Interference>;
    fn mesh(&self) -> Mesh;
}

#[derive(Resource)]
pub struct GridTerrain {
    elements: Vec<Vec<Box<dyn GridElement + 'static>>>,
    step: [f64; 2],
}

unsafe impl Sync for GridTerrain {}
unsafe impl Send for GridTerrain {}

impl GridTerrain {
    pub fn new(elements: Vec<Vec<Box<dyn GridElement>>>, step: [f64; 2]) -> Self {
        Self { elements, step }
    }

    pub fn interference(&self, point: Vector) -> Option<Interference> {
        if point.x < 0. || point.y < 0. {
            if point.z < 0. {
                return Some(Interference {
                    magnitude: -point.z,
                    position: Vector::new(point.x, point.y, 0.),
                    normal: Vector::z(),
                });
            }
            return None;
        }

        let x_index = (point.x / self.step[0]) as usize;
        let y_index = (point.y / self.step[1]) as usize;

        let local_offset = Vector::new(
            x_index as f64 * self.step[0],
            y_index as f64 * self.step[1],
            0.,
        );
        let local_point = point - local_offset;
        if let Some(y_elements) = self.elements.get(y_index) {
            if let Some(element) = y_elements.get(x_index) {
                if let Some(mut interference) = element.interference(local_point) {
                    interference.position += local_offset;
                    return Some(interference);
                }
                return None;
            }
        }
        if point.z < 0. {
            return Some(Interference {
                magnitude: -point.z,
                position: Vector::new(point.x, point.y, 0.),
                normal: Vector::z(),
            });
        }
        return None;
    }
    pub fn build_meshes(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ExtendedMaterial<StandardMaterial, MyExtension>>>,
        parent: Entity,
    ) {
        let x_grid_size = self.elements[0].len() as f64 * self.step[0];
        let y_grid_size = self.elements.len() as f64 * self.step[1];
        let extended_size = 500.;

        // add plane meshes outside of the grid specified by the elements
        let x_offsets = vec![-extended_size, 0.0, x_grid_size];
        let y_offsets = vec![-extended_size, 0.0, y_grid_size];
        let x_sizes = vec![extended_size, x_grid_size, extended_size];
        let y_sizes = vec![extended_size, y_grid_size, extended_size];

        for y_ind in 0..3 {
            for x_ind in 0..3 {
                if x_offsets[x_ind] == 0.0 && y_offsets[y_ind] == 0.0 {
                    continue;
                }
                let mut entity = commands.spawn(MaterialMeshBundle { // PbrBundle {
                    mesh: meshes.add(
                        plane::Plane {
                            size: [x_sizes[x_ind], y_sizes[y_ind]],
                            subdivisions: 1,
                        }
                        .mesh(),
                    ),
                    transform: Transform::from_translation(Vec3 {
                        x: x_offsets[x_ind] as f32,
                        y: y_offsets[y_ind] as f32,
                        z: 0.0,
                    }),
                    material: materials.add(ExtendedMaterial {
                        base: StandardMaterial {
                            base_color: Color::RED,
                            perceptual_roughness: 1.0,
                            opaque_render_method: OpaqueRendererMethod::Auto,
                            ..Default::default()
                        },
                        extension: MyExtension { z_max: PLANESIZE * 0.05 },
                    }),
                    ..default()
                });
                entity.set_parent(parent);
            }
        }

        for (y_index, y_elements) in self.elements.iter().enumerate() {
            for (x_index, element) in y_elements.iter().enumerate() {
                let x_offset = x_index as f32 * self.step[0] as f32;
                let y_offset = y_index as f32 * self.step[1] as f32;

                let transform = Transform::from_translation(Vec3 {
                    x: x_offset,
                    y: y_offset,
                    z: 0.,
                });
                let mut entity = commands.spawn(MaterialMeshBundle { // PbrBundle {
                    mesh: meshes.add(element.mesh()),
                    transform,
                    material: materials.add(ExtendedMaterial {
                        base: StandardMaterial {
                            base_color: Color::RED,
                            perceptual_roughness: 1.0,
                            opaque_render_method: OpaqueRendererMethod::Auto,
                            ..Default::default()
                        },
                        extension: MyExtension { z_max: PLANESIZE * 0.05 },
                    }),
                    ..default()
                });
                entity.set_parent(parent);
            }
        }
    }
}


#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct MyExtension {
    #[uniform(100)]
    z_max: f32,
}

impl MaterialExtension for MyExtension {
    fn fragment_shader() -> ShaderRef {
        "shaders/extended_material.wgsl".into()
    }
}