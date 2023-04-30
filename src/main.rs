mod editor;

use bevy::{prelude::{shape::Cube, *}, pbr::CascadeShadowConfigBuilder};

use editor::{camera_control::camera_control_system, paint::paint_system};

fn main() {
    println!("Hello, studio!");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_app)
        // .add_system(print_tag)
        .add_system(camera_control_system)
        .add_system(paint_system)
        .run();
}

#[derive(Component, Debug)]
pub struct Tag(String);

fn setup_app(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Floor
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_translation(Vec3::ZERO),
        ..Default::default()
    });

    // Model
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/Monkey.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..Default::default()
        },
        Tag("model".to_string()),
    ));
    // commands.spawn((
    //     PbrBundle {
    //         mesh: meshes.add(Mesh::from(Cube { size: 1.0 })),
    //         material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //         transform: Transform::from_xyz(0.0, 1.5, 0.0),
    //         ..Default::default()
    //     },
    //     Tag("Cube".to_string()),
    // ));

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // commands.spawn(DirectionalLightBundle {
    //     directional_light: DirectionalLight {
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     // This is a relatively small scene, so use tighter shadow
    //     // cascade bounds than the default for better quality.
    //     // We also adjusted the shadow map to be larger since we're
    //     // only using a single cascade.
    //     cascade_shadow_config: CascadeShadowConfigBuilder {
    //         num_cascades: 1,
    //         maximum_distance: 1.6,
    //         ..default()
    //     }
    //     .into(),
    //     ..default()
    // });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
