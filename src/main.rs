use bevy::{prelude::*, render::pass::ClearColor};

mod hello_plugin;
mod obj_plugin;

struct RotateAround {
    axis: Vec3,
    angle_per_second: f32,
}

struct RotateAroundLocal {
    axis: Vec3,
    angle_per_second: f32,
}

fn rotate_around_system(
    _time: Res<Time>,
    mut _translation: Mut<Translation>,
    _rotate_around: &RotateAround,
) {
    let rotation = Quat::from_axis_angle(
        _rotate_around.axis,
        _rotate_around.angle_per_second * _time.delta_seconds,
    );

    _translation.0 = rotation.mul_vec3(_translation.0);
}

fn rotate_around_local_system(
    _time: Res<Time>,
    mut _rotation: Mut<Rotation>,
    _rotate_around_local: &RotateAroundLocal,
) {
    let rotation = Quat::from_axis_angle(
        _rotate_around_local.axis,
        _rotate_around_local.angle_per_second * _time.delta_seconds,
    );

    _rotation.0 = _rotation.0.mul_quat(rotation);
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    let monkey_handle: Handle<Mesh> = asset_server.load("assets/obj/suzanne.obj").unwrap();
    let gltf_monkey_handle: Handle<Mesh> = asset_server.load("assets/gltf/Monkey.gltf").unwrap();

    commands
        .spawn(PbrComponents {
            // load a mesh from glTF
            mesh: monkey_handle,
            // create a material for the mesh
            material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
            translation: Translation::new(0.0, 3.0, 0.0),
            ..Default::default()
        })
        .with(RotateAroundLocal {
            axis: Vec3::new(0.0, 1.0, 0.0),
            angle_per_second: 2.0,
        })
        .spawn(PbrComponents {
            mesh: gltf_monkey_handle,
            material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
            translation: Translation::new(-2.5, 3.0, -2.5),
            ..Default::default()
        })
        .with(RotateAroundLocal {
            axis: Vec3::new(0.0, 0.0, 1.0),
            angle_per_second: 2.0,
        })
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            ..Default::default()
        })
        .spawn(LightComponents {
            translation: Translation::new(5.0, 5.0, 0.0),
            ..Default::default()
        })
        .with(RotateAround {
            axis: Vec3::new(0.0, 1.0, 0.0),
            angle_per_second: 2.0,
        })
        .spawn(Camera3dComponents {
            transform: Transform::new_sync_disabled(Mat4::face_toward(
                Vec3::new(-10.0, 10.0, 6.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        });
}

fn main() {
    let mut app = App::build();

    // resources
    app.add_resource(Msaa { samples: 4 });
    // app.add_resource(ClearColor(Color::rgb_u8(96, 88, 64)));

    // plugins
    app.add_default_plugins();
    app.add_plugin(obj_plugin::ObjPlugin);
    // Systems
    app.add_startup_system(setup.system());
    app.add_system(rotate_around_system.system());
    app.add_system(rotate_around_local_system.system());
    // app.add_plugin(hello_plugin::HelloPlugin);

    app.run();
}
