# Capstone-Vehicle-Sim-Project - Driver's Altitude
### **Repository for CS 461 - 463**

## Project Website:
Our webpage covers the features we focused on and developed for this project.
You can follow this link to view it [here](https://stanleychale.github.io/Capstone-Vehicle-Sim-Project-Team3/).

## Project Structure:

.  
├── docs/  
├── src/  
    ├── src/cameras
    ├── src/rigid_body
    ├── src/car
    ├── src/grid_terrain
    ├── src/integrator
├── wip/  
├── .gitignore  
├── LICENSE  
└── README.md  

The docs/ directory contains our supporting team documents for our team deliverables and assignments. Please note that this project's github pages website is hosted from the "astro" branch of this repository.

The `src/` directory contains our codebase for the project.

Within the `src/` directory, there is a series of other folders that contain the actual code for this project.

The `src/cameras/` directory contains the code that controls and updates the camera that is attached to the car.

The src/car/ directory contains the main function and the App struct, which contains the entire ECS and world, and is the "center" of the program. All the other folders and files branch off of this one. 
As a reminder, the program can be run with the following command run inside of the src/ folder: 
```bash
cargo run --example <example_name>
```

The `src/grid_terrain/` directory contains the files that are responsible for terrain
generation, including the bumpy, mountainous terrain simulated by perlin noise.

The `src/integrator/` directory is relatively small, and contains some of the program's 
physics and game state functionality.

The `src/rigid_body/` directory contains a lot of the lower-level physics and collision
functionality, particularly relating to the player-controlled vehicle.

# How to Run This Project:
Download the latest zipped build of the game which can be found [here]().
Alternatively you can clone this repositry and build the project yourself! To do so you can follow this link to the [Project Documentation](src/project-info.md).

# Project Dependency:
This project is based on our **Project Partner, Chris Patton's** original vehicle demo.
The original project repository can be found [here](https://github.com/crispyDyne/bevy_car_demo/tree/main).

# Project Credits:
Team Members:
- Stanley Hale - halesta@oregonstate.edu
  - UI, Engine Audio System, Car Model Implementation
- Ezra McDonald-Mueller - mcdonaez@oregonstate.edu
  - Terrain Generation
- Greggory Hickman​ - hickmang@oregonstate.edu
  - Multiple Cars Functionality, UI
- Benny Xu - xube@oregonstate.edu
  - Terrain Generation, Shaders

Project Partner:
- Christopher Patton - contact@pattondynamics.com
  - Dynamics and Simulation Engineer at Patton Dynamics LLC
  - Chris Patton holds a Ph.D. in Mechanical Engineering from Oregon State University and has worked in Formula 1, LMP1, and the space launch industry. He currently leads Patton Dynamics, specializing in simulation services for the Aerospace, Motorsports, and Wave Energy Generation industries.

## 3rd Party Crates
- `flo_curves` - [Link](https://docs.rs/flo_curves/latest/flo_curves/)
    - Library of routines for inspecting and manipulating curves, with a focus on cubic Bézier curves.
## Assets:
- Vehicle Model: 
    - Diw3D - [Link](https://www.turbosquid.com/FullPreview/2087206)
- Engine Audio: 
    - Mixkit ATV engine motor hum - [Link](https://mixkit.co/free-sound-effects/hum/)