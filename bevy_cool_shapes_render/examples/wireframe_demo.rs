use std::f32::consts::TAU as TAU32;
use std::f64::consts::TAU;

use bevy::prelude::*;
use bevy_cool_shapes_render::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(RenderableShapesPlugin)
        .add_startup_system(setup)
        .add_system(enable_animation)
        .add_system(run_animation)
        .run();
}

#[derive(Component)]
enum Animated {
    Bob { period: f64, direction: Vec3 },
    Rotate { period: f64, axis: Vec3 },
}
fn bob(period: f64, direction: Vec3) -> Option<Animated> {
    Some(Animated::Bob { period, direction })
}
fn rotate(period: f64, axis: Vec3) -> Option<Animated> {
    Some(Animated::Rotate { period, axis })
}
#[derive(Component)]
struct AnimationState {
    transform: Transform,
}

fn enable_animation(
    animated: Query<(Entity, &Transform), Added<Animated>>,
    mut commands: Commands,
) {
    let mut cmd_buffer = Vec::new();
    for (entity, &transform) in animated.iter() {
        let state = AnimationState { transform };
        cmd_buffer.push((entity, (state,)));
    }
    commands.insert_or_spawn_batch(cmd_buffer);
}

fn run_animation(
    time: Res<Time>,
    mut animated: Query<(&mut Transform, &AnimationState, &Animated)>,
) {
    let time = time.seconds_since_startup();
    for (mut trans, init, anim) in animated.iter_mut() {
        match *anim {
            Animated::Rotate { axis, period } => {
                let angle = time % period / period * TAU;
                let rot = Quat::from_axis_angle(axis, angle as f32);
                trans.rotation = init.transform.rotation * rot;
            }
            Animated::Bob { direction, period } => {
                let anim_offset = time % period / period * TAU;
                let space_offset = direction * (anim_offset as f32).sin();
                trans.translation = init.transform.translation + space_offset;
            }
        }
    }
}

fn star_lines() -> impl Iterator<Item = [usize; 2]> {
    (0..11).map(|i| [i, (i + 1) % 10])
}

fn star_points() -> impl Iterator<Item = Vec2> {
    (0..11).map(|i| {
        let even = i % 2 == 0;
        let angle = TAU32 / 10. * (i as f32);
        let radius = if even { 2.0 } else { 1.0 };
        Vec2::new(angle.sin(), angle.cos()) * radius
    })
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PerspectiveCameraBundle::new_3d())
        .insert(Transform::from_xyz(25., 10., 25.).looking_at(Vec3::ZERO, Vec3::Y));

    // The `I` is to make sure I update all the arrays at the same time.
    const I: usize = 18;

    let [x, y, z] = Vec3::AXES;
    let one = Vec3::ONE;

    let star = Polygon {
        points: star_points().collect(),
        lines: star_lines().collect(),
        tris: vec![],
    };
    #[rustfmt::skip]
    let shapes: [OutlineableShape; I] = [
        Disc { radius: 3. }.into(),
        HalfDisc { radius: 1.0 }.into(),
        QuarterDisc { radius: 1.9 }.into(),
        Rectangle { size: Vec2::ONE * 3.3 }.into(),
        Triangle { a: Vec2::ZERO, b: Vec2::Y, c: Vec2::X }.into(),
        star.clone().into(),
        Pyramid { base: Shape2d::Polygon(star.clone()), height: 3.0 }.into(),
        Extruded { base: Shape2d::Polygon(star), height: 3.0 }.into(),
        Sphere { radius: 2.3 }.into(),
        HalfSphere { radius: 2.0 }.into(),
        Capsule { radius: 1.3, segment_height: 5.0 }.into(),
        Cuboid { size: one * 2.3 }.into(),
        Cone { base_radius: 1.0, height: 2.0 }.into(),
        Tetrahedron { a: Vec3::ZERO, b: x * 2., c: y * 2., d: z * 2. }.into(),
        Tetrahedron { a: Vec3::ZERO, b: x * -2., c: y * -2., d: z * -2. }.into(),
        Lines {
            points: vec![Vec3::ZERO, x, y, z],
            lines: vec![[0,1], [0,2], [0,3]],
        }.into(),
        Cylinder { height: 5., radius: 2. }.into(),
        HeightField {
            size: Vec2::ONE * 30.0,
            heights: vec![
                vec![1., 1., 1., 1., 1., 1., 1., 1., 1., 1.],
                vec![1., 0., 1., 0., 1., 0., 1., 0., 1., 0.],
                vec![1., 0., 1., 0., 1., 0., 1., 0., 1., 0.],
                vec![1., 0., 1., 0., 1., 0., 1., 0., 1., 0.],
                vec![1., 0., 1., 0., 1., 0., 1., 0., 1., 0.],
                vec![1., 0., 1., 0., 1., 0., 1., 0., 1., 0.],
                // vec![1.5, 0.8, 0., 0.],
                // vec![0.8, 0.2, 0., 0.],
                // vec![0., 0., 0.6, 0.],
                // vec![1., 1., 1., 1.],
            ],
        }.into(),
    ];
    let colors: [_; I] = [
        (Color::CYAN, 200.0),
        (Color::GREEN, 9.0),
        (Color::CYAN, 93.0),
        (Color::RED, 13.0),
        (Color::AQUAMARINE, 200.0),
        (Color::OLIVE, 43.0),
        (Color::OLIVE, 43.0),
        (Color::OLIVE, 43.0),
        (Color::GOLD, 143.0),
        (Color::LIME_GREEN, 43.0),
        (Color::GREEN, 13.0),
        (Color::FUCHSIA, 63.0),
        (Color::CRIMSON, 63.0),
        (Color::BLUE, 53.0),
        (Color::RED, 53.0),
        (Color::PINK, 100.0),
        (Color::WHITE, 100.0),
        (Color::YELLOW, 100.0),
    ];
    let xy = Vec3::new(1., 1., 0.).normalize();
    let animations: [_; I] = [
        (bob(9.34, y * 9.), Vec3::new(-5., 5., -5.)),
        (bob(6.34, z * 5.), Vec3::new(5., 5., 5.)),
        (rotate(1.34, z), Vec3::new(-4., 5., 4.)),
        (rotate(3.34, x), Vec3::new(10., 10., -5.)),
        (rotate(9.34, one.normalize()), Vec3::new(10., 10., 21.)),
        (rotate(9.34, x), Vec3::new(10., 9., 18.)),
        (rotate(9.34, y), Vec3::new(15., 9., 18.)),
        (rotate(9.34, z), Vec3::new(20., 9., 18.)),
        (bob(2.14, y * 5.), Vec3::new(0., 5., 0.)),
        (rotate(12.14, y), Vec3::new(20., 5., 15.)),
        (rotate(10., one.normalize()), Vec3::new(0., 7., 15.)),
        (rotate(3.34, y), Vec3::new(10., 5., 15.)),
        (rotate(9.34, x), Vec3::new(15., 10., 15.)),
        (rotate(5., xy), Vec3::new(20., 7., 19.)),
        (rotate(5., xy), Vec3::new(20., 7., 19.)),
        (rotate(-9.34, z), Vec3::new(15., 10., -15.)),
        (rotate(9.34, x), Vec3::new(15., 10., -15.)),
        (None, Vec3::ZERO),
    ];
    let shapes = shapes.into_iter();
    let colors = colors.into_iter();
    let animations = animations.into_iter();
    let iterator = shapes.zip(colors).zip(animations).enumerate();
    for (i, ((shape, (color, width)), (anim, pos))) in iterator {
        let mut entity = commands.spawn_bundle(ShapeOutlineBundle {
            shape: shape.lines(color, width, if i == 0 { 0.4 } else { 0.0 }),
            transform: Transform::from_translation(pos),
            ..default()
        });
        if let Some(animation) = anim {
            entity.insert(animation);
        }
    }
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(shape::Cube::new(10.).into()),
        material: mats.add(Color::WHITE.into()),
        ..default()
    });
}
