#![allow(dead_code)]
use bevy::prelude::*;
use rand::Rng;

//Flo Curves crate is used for the definition and creation of bezier curves for audio playback
use flo_curves::bezier;
use flo_curves::bezier::Curve;
use flo_curves::*;

use cameras::control::CameraParentList;
use rigid_body::{
    definitions::{MeshDef, MeshTypeDef, TransformDef}, 
    joint::{Base, Joint}, 
    sva::{Inertia, Matrix, Motion, Vector, Xform},
    plugin::CarState, 
};

use crate::{
    control::{CarControl, ControlType}, physics::{
        BrakeWheel, DriveType, DrivenWheelLookup, SteeringCurvature, SteeringType,
        SuspensionComponent,
    }, preferences::CarPreferences, tire::PointTire
};

#[derive(Resource)]
pub struct CarDefinition {
    chassis: Chassis,
    suspension: Vec<Suspension>,
    wheel: Wheel,
    drives: Vec<DriveType>,
    brake: Brake,
    pub carcontrol: CarControl,
    pub id: i32,
} 

/*
 * Struct CarList
 * Contains the list of car that are currently a part of this game session
 */
#[derive(Resource, Default)]
pub struct CarList {
    pub cars: Vec<CarDefinition>,
}

#[derive(Component)]
pub struct Engine {
    speed: f32,
    curve: Curve<Coord2>,
}

const SUSPENSION_MASS: f64 = 20.;
const GRAVITY: f64 = 9.81;

/*
 * Inputs: Queries for Brake joints, Players, and Engine Components.
 * Outputs: None
 * Description: This function updates the engine speed of the car by calculating the 
 * speed of the car using the qd of the driven wheel and the radius of the wheel.
 */
pub fn update_engine_speed(
    joints: Query<(&Joint, &BrakeWheel)>,
    mut players: ResMut<CarList>,
    mut engine_q: Query<&mut Engine>,
) {

    let playerlist = &mut players.cars;

    let joint_list: Vec<(&Joint, &BrakeWheel)> = joints.iter().collect();

    let mut count = 0;
    for mut engine in engine_q.iter_mut() {
        let car_joint = count * 2;
        let qd = joint_list[car_joint].0.qd.abs();
        let radius = playerlist[count].wheel.radius;

        //Update the speed
        engine.speed = (qd * radius) as f32; 
        count += 1;
    } 
}

/*
 * Inputs: Queries for the SpatialAudioSink and Engine Components.
 * Outputs: None
 * Description: This function updates the playback speed of the engine audio sink by 
 * calculating the speed of the car using the qd of the driven wheel and the radius of the wheel.
 */
pub fn update_engine_audio(
    music_controller: Query<&SpatialAudioSink, With<Engine>>, 
    engine_q: Query<&Engine>,
    car_preferences: Res<CarPreferences>
) {
    let music_controller: Vec<&SpatialAudioSink> = music_controller.iter().collect();

    let engine_list: Vec<&Engine> = engine_q.iter().collect();

    //For loop for the length of the music_controller
    for i in 0..music_controller.len() {
        //Grab our value from bezier curve using our modified speed value (15% of current speed, always between [0.0, 1.0])
        let mut speed_curve = 
            engine_list[i].curve.point_at_pos(                                          //Get the position from the bezier curve
                (( (engine_list[i].speed * 0.05) % 1.0)).into()                         //Modulate the current speed by 1.0, so it always stays between [0.0, 1.0]
            ).y()                                                                       //Grab the Y-value of from this position on the bezier curve
            as f32;                                                                     //Cast this value to a f32
                
        //Calculate the offset
        let offset = engine_list[i].speed * 0.030;

        //Make the value smaller and apply an offset
        speed_curve = speed_curve + offset;

        //Set the playback speed to our calculated speed_curve of this specific engine audio sink
        music_controller[i].set_speed(speed_curve);
        music_controller[i].set_volume(car_preferences.volume as f32);
    }
}

/*
 * Inputs: none
 * Outputs: CarDefinition - The struct containing the car's specifications
 * Description: Defines a car's specifications to later be built by car_startup_system().
 */
pub fn build_car(
    startposition: [f64; 3], 
    control_type: ControlType, 
    id: i32,
    max_speed: f64,
    chassis_mass: f64,
    max_torque: f64,
    fricion_coefficient: f64,
) -> CarDefinition {
    // Separate the start position into x, y, z coordinates
    let xpos = startposition[0];
    let ypos = startposition[1];
    let zpos = startposition[2];
    
    // Chassis
    let mass = 1000.;
    let dimensions = [3.0_f64, 1.2, 0.4]; // shape of rectangular chassis
    let moi = [
        dimensions[1].powi(2) + dimensions[2].powi(2),
        dimensions[2].powi(2) + dimensions[0].powi(2),
        dimensions[0].powi(2) + dimensions[1].powi(2),
    ]
    .map(|x| mass * (1. / 12.) * x);

    let chassis = Chassis {
        //Get our mass
        mass,
        cg_position: [0., 0., 0.],
        moi,
        dimensions,
        position: startposition, // position: [0., 0., 0.],
        initial_position: [-5. + xpos, 20. + ypos, 0.3 + 0.25 + zpos], // initial_position: [-5., 20., 0.3 + 0.25],
        initial_orientation: [0., 0., 1.57],
        mesh_file: Some("models/vehicle/chassis/car_chassisV2.glb#Scene0".to_string()),
        index: id,
    };

    // Suspension
    let suspension_mass = 20.;
    let suspension_size = 0.025_f64;
    let suspension_stiffness = mass * (GRAVITY / 4.) / 0.1;
    let suspension_damping = 0.25 * 2. * (suspension_stiffness * (1000. / 4.) as f64).sqrt();
    let suspension_preload = mass * (GRAVITY / 4.);
    let suspension_moi = (2. / 3.) * suspension_mass * suspension_size.powi(2);

    let suspension_names = ["fl", "fr", "rl", "rr"].map(|name| name.to_string());
    let suspension_locations = [
        [1.57, 0.75, -0.2],
        [1.57, -0.75, -0.2],
        [-1.31, 0.75, -0.2],
        [-1.31, -0.75, -0.2],
    ];

    let suspension: Vec<Suspension> = suspension_locations
        .iter()
        .zip(suspension_names.clone())
        .enumerate()
        .map(|(ind, (location, name))| {
            let steering = if ind < 2 {
                // SteeringType::Angle(Steering {
                //     max_angle: 30.0_f64.to_radians(),
                // })
                SteeringType::Curvature(SteeringCurvature {
                    x: suspension_locations[ind][0] - suspension_locations[ind + 2][0],
                    y: suspension_locations[ind][1],
                    max_curvature: 1. / 5.0,
                })
            } else {
                SteeringType::None
            };
            Suspension {
                name,
                mass: suspension_mass,
                steering,
                stiffness: suspension_stiffness,
                damping: suspension_damping,
                preload: suspension_preload,
                moi: suspension_moi,
                location: *location,
            }
        })
        .collect();

    // Wheel
    let wheel = build_wheel(chassis_mass, fricion_coefficient);

    // Calculate middle speeds
    let lower_speed = max_speed * 0.25;
    let middle_speed = max_speed * 0.5;

    // Drive and Brake Speeds
    let drive_speeds = vec![0., lower_speed, middle_speed, max_speed];

    // Calculate torques
    let middle_torque = max_torque * 0.6;
    let low_torque = max_torque * 0.25;

    // Drive and Brake Torques
    let drive_torques = vec![max_torque, max_torque, middle_torque, low_torque];

    let rear_drive = DriveType::DrivenWheelLookup(DrivenWheelLookup::new(
        "fl".to_string(),
        drive_speeds.clone(),
        drive_torques.clone(),
    ));

    let drives = vec![
        DriveType::None,
        DriveType::None,
        rear_drive.clone(),
        rear_drive.clone(),
    ];

    let brake = Brake {
        front_torque: 800.,
        rear_torque: 400.,
    };

    let carcontrol = CarControl {
        throttle: 0.,
        steering: 0.,
        brake: 0.,
        steer_wheels: Vec::new(),
        brake_wheels: Vec::new(), // Initialize the BrakeWheels vector
        drive_wheels: Vec::new(),
        control_type,
    };

    CarDefinition {
        chassis,
        suspension,
        wheel,
        drives,
        brake,
        carcontrol,
        id,
    }
}

pub fn build_wheel(chassis_mass: f64, fricion_coefficient: f64) -> Wheel {
    let wheel_mass = 20.;
    let wheel_radius = 0.325_f64;
    let wheel_moi_y = wheel_mass * wheel_radius.powi(2);
    let wheel_moi_xz = 1. / 12. * 10. * (3. * wheel_radius.powi(2));
    let corner_mass = chassis_mass / 4. + SUSPENSION_MASS + wheel_mass;
    let wheel_stiffness = corner_mass * GRAVITY / 0.005;
    let wheel_damping = 0.01 * 2. * (wheel_stiffness * wheel_mass).sqrt();
    Wheel {
        mass: wheel_mass,
        radius: wheel_radius,
        width: 0.2_f64,
        moi_y: wheel_moi_y,
        moi_xz: wheel_moi_xz,
        stiffness: [wheel_stiffness, 0.],
        damping: wheel_damping,
        coefficient_of_friction: fricion_coefficient,
        rolling_radius: 0.315,
        low_speed: 1.0,
        normalized_slip_stiffness: 20.0,
        filter_time: 0.005,
    }
}

pub fn car_startup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut players: ResMut<CarList>,
    mut car_state: ResMut<NextState<CarState>>,
    car_preferences: Res<CarPreferences>,
) {
    //Motion here is for gravity   (9.81 m/s by default)
    let base = Joint::base(Motion::new([0., 0., car_preferences.gravity], [0., 0., 0.]));
    let base_id = commands.spawn((base, Base)).id();

    let mut camera_parent_list = Vec::new();
    camera_parent_list.push(base_id);
    for car in &mut players.cars {
        let control = CarControl::default();
        let control_id = commands
            .spawn((control,))
            .insert(TransformBundle::from(Transform::from_xyz(5.0, 5.0, 0.0)))
            .id();

        let mut rng = rand::thread_rng();

        // Chassis
        let chassis_ids = car.chassis.build(
            &mut commands,
            Color::rgb(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()),
            base_id,
            &asset_server,
        );
        let chassis_id = chassis_ids[3]; // ids are not ordered by parent child order!!! "3" is rx, the last joint in the chain

        camera_parent_list.push(chassis_ids[5]);

        let mut brake_wheel_ids = Vec::new(); // fill this with ids and set car.carcontrol.brake_wheels
        let mut steer_wheel_ids = Vec::new(); // fill this with ids and set car.carcontrol.steer_wheels

        for (ind, susp) in car.suspension.iter().enumerate() {
            let braked_wheel = if ind < 2 {
                Some(BrakeWheel {
                    max_torque: car.brake.front_torque,
                    control: control_id,
                })
            } else {
                Some(BrakeWheel {
                    max_torque: car.brake.rear_torque,
                    control: control_id,
                })
            };
            let (susp_id, maybe_steer_id) = susp.build(&mut commands, chassis_id, &susp.location);
            let wheel_id = car.wheel.build(
                &mut commands,
                &susp.name,
                susp_id,
                car.drives[ind].clone(),
                braked_wheel.clone(),
                0.,
                &asset_server,
                ind,
            );

            // Fill the brake_wheel_ids vector with the ids of the BrakeWheels of this car
            brake_wheel_ids.push(wheel_id);
            if let Some(wheel_id) = maybe_steer_id {
                steer_wheel_ids.push(wheel_id);
            }
        }
        car.carcontrol.brake_wheels = brake_wheel_ids; // update the car
        car.carcontrol.steer_wheels = steer_wheel_ids; // update the car
        commands.spawn(car.carcontrol.clone());
    }

    commands.insert_resource(CameraParentList {
        list: camera_parent_list,
        active: 1, // start with following x, y, z and yaw of chassis
    });

    //Change the car state to Rendered
    car_state.set(CarState::Rendered);

}

#[derive(Clone)]
pub struct Chassis {
    pub mass: f64,
    pub cg_position: [f64; 3], // Center of Gravity Position
    pub moi: [f64; 3],
    pub dimensions: [f64; 3],
    pub position: [f64; 3],
    pub initial_position: [f64; 3],
    pub initial_orientation: [f64; 3],
    pub mesh_file: Option<String>,
    pub index: i32,
}

impl Chassis {
    pub fn build(
        &self,
        commands: &mut Commands,
        color: Color,
        parent_id: Entity,
        asset_server: &Res<AssetServer>,
    ) -> Vec<Entity> {
        // x degree of freedom (absolute coordinate system, not relative to car)
        let mut px = Joint::px("chassis_px".to_string(), Inertia::zero(), Xform::identity());
        px.q = self.initial_position[0];
        let mut px_e = commands.spawn((px, SpatialBundle::default()));
        px_e.set_parent(parent_id);
        let px_id = px_e.id();

        // y degree of freedom (absolute coordinate system, not relative to car)
        let mut py = Joint::py("chassis_py".to_string(), Inertia::zero(), Xform::identity());
        py.q = self.initial_position[1];
        let mut py_e = commands.spawn((py, SpatialBundle::default()));
        py_e.set_parent(px_id);
        let py_id = py_e.id();

        // z degree of freedom (always points "up", relative to absolute coordinate system)
        let mut pz = Joint::pz("chassis_pz".to_string(), Inertia::zero(), Xform::identity());
        pz.q = self.initial_position[2];
        let mut pz_e = commands.spawn((pz, SpatialBundle::default()));
        pz_e.set_parent(py_id);
        let pz_id = pz_e.id();

        // yaw degree of freedom (rotation around z axis)
        let mut rz = Joint::rz("chassis_rz".to_string(), Inertia::zero(), Xform::identity());
        rz.q = self.initial_orientation[2];
        let mut rz_e = commands.spawn((rz, SpatialBundle::default()));
        rz_e.set_parent(pz_id);
        let rz_id = rz_e.id();

        // pitch degree of freedom (rotation around y axis)
        let mut ry = Joint::ry("chassis_ry".to_string(), Inertia::zero(), Xform::identity());
        ry.q = self.initial_orientation[1];
        let mut ry_e = commands.spawn((ry, SpatialBundle::default()));
        ry_e.set_parent(rz_id);
        let ry_id = ry_e.id();

        // roll degree of freedom (rotation around x axis)
        // this is the body of the car!
        let mass = self.mass;
        let cg_position = self.cg_position;
        let moi = self.moi;
        let position = self.position;
        let dimensions = self.dimensions;
        let inertia = Inertia::new(
            mass,
            Vector::new(cg_position[0], cg_position[1], cg_position[2]),
            Matrix::from_diagonal(&Vector::new(moi[0], moi[1], moi[2])),
        );

        let mut rx = Joint::rx("chassis_rx".to_string(), inertia, Xform::identity());
        rx.q = self.initial_orientation[0];
        let mut rx_e = commands.spawn((rx, SpatialBundle::default()));
        rx_e.set_parent(ry_id);
        let rx_id = rx_e.id();

        //Create a bezier curve for curving playback audio (to simulate changing of gears)
        let sound_curve = bezier::Curve::from_points(
            Coord2(0.0, 0.6),
            (Coord2(1.0, 1.4), Coord2(0.84, 0.55)),
            Coord2(1.0, 1.0),
        );

        //Insert the car chassis into the rx roll degree of freedom joint entity.
        if let Some(_chassis_file) = &self.mesh_file {
            rx_e.insert(SceneBundle {
                transform: (&TransformDef::from_position(position)).into(),
                scene: asset_server.load("models/vehicle/chassis/car_chassis.glb#Scene0"),
                ..default()
            });

             //Setup audio emitter for our engine audio and parent it to our chassis
             rx_e.insert((
                AudioBundle {
                    source: asset_server.load("sounds/engine_hum.ogg"),
                    settings: PlaybackSettings::LOOP.with_spatial(true),
                    ..default()
                },
                Engine {
                    speed: 0.0,
                    curve: sound_curve,
                },
            ));
        } else {
            rx_e.insert(MeshDef {
                mesh_type: MeshTypeDef::Box {
                    dimensions: [
                        dimensions[0] as f32,
                        dimensions[1] as f32,
                        dimensions[2] as f32,
                    ],
                },
                transform: TransformDef::from_position(position),
                color,
            });
        }

        let chassis_ids = vec![px_id, py_id, pz_id, rx_id, ry_id, rz_id];
        // return id the last joint in the chain. It will be the parent of the suspension / wheels
        chassis_ids
    }
}

#[derive(Clone)]
pub struct Suspension {
    pub name: String,
    pub mass: f64,
    pub steering: SteeringType,
    pub stiffness: f64,
    pub damping: f64,
    pub preload: f64,
    pub moi: f64,
    pub location: [f64; 3],
}

impl Suspension {
    pub fn build(
        &self,
        commands: &mut Commands,
        mut parent_id: Entity,
        location: &[f64; 3],
    ) -> (Entity, Option<Entity>) {
        // suspension transform
        let mut xt_susp = Xform::new(
            Vector::new(location[0], location[1], location[2]), // location of suspension relative to chassis
            Matrix::identity(),
        );

        // suspension mass
        let inertia = Inertia::new(
            self.mass,
            Vector::new(0., 0., 0.),       // center of mass
            self.moi * Matrix::identity(), // inertia
        );

        let mut steer_id = None;
        match self.steering.clone() {
            SteeringType::None => {}
            SteeringType::Curvature(steering) => {
                let steer_name = ("steer_".to_owned() + &self.name).to_string();
                let steer = Joint::rz(steer_name, Inertia::zero(), xt_susp);
                let mut steer_e = commands.spawn((steer, steering));
                steer_id = Some(steer_e.id());
                steer_e.set_parent(parent_id);

                parent_id = steer_e.id();
                xt_susp = Xform::identity();
            }
            SteeringType::Angle(steering) => {
                // create suspension joint
                let steer_name = ("steer_".to_owned() + &self.name).to_string();
                let steer = Joint::rz(steer_name, Inertia::zero(), xt_susp);
                let mut steer_e = commands.spawn((steer, steering));
                steer_id = Some(steer_e.id());
                steer_e.set_parent(parent_id);

                parent_id = steer_e.id();
                xt_susp = Xform::identity();
            }
        }

        // create suspension joint
        let name = ("susp_".to_owned() + &self.name).to_string();
        let susp = Joint::pz(name, inertia, xt_susp);

        // create suspension entity
        let mut susp_e = commands.spawn((
            susp,
            SpatialBundle::default(),
            SuspensionComponent::new(self.stiffness, self.damping, self.preload),
        ));
        susp_e.set_parent(parent_id);

        (susp_e.id(), steer_id)
    }
}

#[derive(Resource, Clone)]
pub struct Wheel {
    pub mass: f64,
    pub radius: f64,
    pub width: f64,
    pub moi_y: f64,
    pub moi_xz: f64,
    pub stiffness: [f64; 2],
    pub damping: f64,
    pub coefficient_of_friction: f64,
    pub rolling_radius: f64,
    pub low_speed: f64,
    pub normalized_slip_stiffness: f64,
    pub filter_time: f64,
}

impl Wheel {
    pub fn build(
        &self,
        commands: &mut Commands,
        corner_name: &String,
        parent_id: Entity,
        driven_wheel: DriveType,
        braked_wheel: Option<BrakeWheel>,
        initial_speed: f64,
        asset_server: &Res<AssetServer>,
        index: usize,
    ) -> Entity {
        // wheel inertia
        let inertia = Inertia::new(
            self.mass,
            Vector::new(0., 0., 0.),
            Matrix::from_diagonal(&Vector::new(self.moi_xz, self.moi_y, self.moi_xz)),
        );

        // create wheel joint
        let name = ("wheel_".to_owned() + corner_name).to_string();
        let mut ry = Joint::ry(name, inertia, Xform::identity());
        ry.qd = initial_speed;

        let mut wheel_e;
        // Check which side this wheel model should be displayed as depending on index number at setup (Left or Right)
        if index == 1 || index == 3 {
            wheel_e = commands.spawn((
                ry,
                // Assign the mesh of the wheel model
                SceneBundle {
                    transform: (&TransformDef::Identity).into(),
                    scene: asset_server.load("models/vehicle/wheel/wheelR.glb#Scene0"),
                    ..default()
                },
            ));
        } else {
            wheel_e = commands.spawn((
                ry,
                // Assign the mesh of the wheel model
                SceneBundle {
                    transform: (&TransformDef::Identity).into(),
                    scene: asset_server.load("models/vehicle/wheel/wheelL.glb#Scene0"),
                    ..default()
                },
            ));
        }

        // add driven and braked components
        match driven_wheel {
            DriveType::None => {}
            DriveType::DrivenWheelLookup(driven) => {
                wheel_e.insert(driven);
            }
            DriveType::DrivenWheel(driven) => {
                wheel_e.insert(driven);
            }
        }

        if let Some(braked) = braked_wheel {
            wheel_e.insert(braked);
        }

        // set parent
        wheel_e.set_parent(parent_id);
        let wheel_id = wheel_e.id();

        // add tire contact model
        commands.spawn(PointTire::new(
            wheel_id,
            parent_id,
            self.stiffness,
            self.damping,
            self.coefficient_of_friction,
            self.normalized_slip_stiffness,
            self.rolling_radius,
            self.low_speed,
            self.radius,
            self.width,
            self.filter_time,
            5,
            51,
            0.01,
        ));
        wheel_id
    }
}

pub struct Brake {
    front_torque: f64,
    rear_torque: f64,
}
