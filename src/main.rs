mod editor;

use bevy::prelude::{shape::Cube, *};

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

#[derive(Component)]
struct Tag(String);

fn setup_app(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_translation(Vec3::ZERO),
        ..Default::default()
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            ..Default::default()
        },
        Tag("Cube".to_string()),
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn print_tag_system(query: Query<&Tag>) {
    for tag in query.iter() {
        println!("Tag: {}", tag.0);
    }
}
