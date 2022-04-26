use bevy::prelude::*;
use bevy_cool_shapes_render::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(RenderableShapesPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle::new_3d())
        .insert(Transform::from_xyz(25., 10., 25.).looking_at(Vec3::ZERO, Vec3::Y));
    let shape = Sphere { radius: 3.0 };
    let color = Color::YELLOW;
    let depth_bias = 0.0;
    commands.spawn_bundle(ShapeOutlineBundle {
        shape: OutlineableShape::from(shape).lines(color, 200.0, depth_bias),
        ..default()
    });
}
