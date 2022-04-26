use std::f32::consts::TAU;
use std::iter;

use bevy::math::{Mat2, Vec2, Vec3, Vec3Swizzles};
use bevy_cool_shapes::*;

#[derive(Debug, Clone)]
pub struct OutlineableShape {
    density: usize,
    shape: OutlineableShapeEnum,
}
impl Default for OutlineableShape {
    fn default() -> Self {
        OutlineableShape {
            density: 4,
            shape: OutlineableShapeEnum::Sphere(Sphere { radius: 1.0 }),
        }
    }
}

#[derive(Debug, Clone)]
enum OutlineableShapeEnum {
    Shape2d(Shape2d),
    Pyramid(Pyramid),
    Extruded(Extruded),
    Sphere(Sphere),
    HalfSphere(HalfSphere),
    Capsule(Capsule),
    Cuboid(Cuboid),
    Cone(Cone),
    Tetrahedron(Tetrahedron),
    Lines(Lines),
    Cylinder(Cylinder),
    HeightField(HeightField),
}
macro_rules! into_debug_shape {
    ($( $shape:ident ,)*) => (
        $(impl From<$shape> for OutlineableShape{
            fn from(shape: $shape) -> Self {
                Self {
                    shape: OutlineableShapeEnum::$shape(shape),
                    density: 4,
                }
            }
        })*
    )
}
macro_rules! into_debug_shape_2d {
    ($( $shape:ident ,)*) => (
        $(impl From<$shape> for OutlineableShape {
            fn from(shape: $shape) -> Self {
                Self {
                    shape: OutlineableShapeEnum::Shape2d(Shape2d::$shape(shape)),
                    density: 4,
                }
            }
        })*
    )
}
#[rustfmt::skip]
into_debug_shape! {
    Shape2d,    Pyramid,     Extruded, Sphere,
    HalfSphere, Capsule,     Cuboid,   
    Cone,       Tetrahedron, Lines,     Cylinder, HeightField,
}
#[rustfmt::skip]
into_debug_shape_2d! {
    Disc, HalfDisc, QuarterDisc, Rectangle,
     Triangle, Polygon,
}

fn outline_2d(shape: &Shape2d, density: usize) -> Vec<Vec2> {
    let segment_rotation = Mat2::from_angle(TAU / 4.0 / density as f32);
    let circle = |start| iter::successors(Some(start), |v| Some(segment_rotation * *v));
    let half_circle = |radius: f32| circle(-Vec2::X * radius).take(density * 2 + 1);
    let full_circle = |radius: f32| circle(-Vec2::X * radius).take(density * 4 + 1);
    macro_rules! match_2d {
        ( $input:expr, { $( $shape:ident $binding:tt => $arm:expr ,)* }) => (
            match $input { $( Shape2d::$shape($shape $binding ) => $arm, )* }
        )
    }
    match_2d!(*shape, {
        Disc { radius } => full_circle(radius).collect(),
        HalfDisc { radius } => half_circle(-radius).collect(),
        QuarterDisc { radius } => circle(Vec2::Y * radius).take(density + 1).collect(),
        Triangle { a, b, c } => vec![a, b, c, a],
        Polygon { ref points, ref lines, .. } => lines
            .iter()
            .flat_map(|[a, b]| [points[*a], points[*b], Vec2::NAN])
            .collect(),
        Rectangle { size } => {
            let xy = size / 2.0;
            let a = xy;
            let b = xy * Vec2::new(1.0, -1.0);
            vec![a, b, -a, -b, a]
        },
    })
}

impl OutlineableShape {
    pub(crate) fn outline(&self) -> Vec<Vec3> {
        // chain_segments![a,b,c,d] =>
        // a.chain(iter::once(Vec3::NAN))
        //  .chain(b).chain(iter::once(Vec3::NAN))
        //  .chain(c).chain(iter::once(Vec3::NAN))
        //  .chain(d).collect()
        macro_rules! chain_segments {
            ($head_iter:expr, $($iters:expr),* $(,)?) => (
                $head_iter
                    $(.chain(iter::once(Vec3::NAN)).chain($iters))*
                    .collect()
            )
        }
        macro_rules! match_3d {
            ( $input:expr, {
                Shape2d($first_binding:ident) => $first_arm:expr,
                $( $shape:ident $binding:tt => $arm:expr ,)*
            }) => (
                match $input {
                    OutlineableShapeEnum::Shape2d(ref $first_binding) => $first_arm,
                    $( OutlineableShapeEnum::$shape($shape $binding ) => $arm, )*
                }
            )
        }
        let density = self.density;
        let x3d = |v: Vec2| Vec3::new(0.0, v.x, v.y);
        let y3d = |v: Vec2| Vec3::new(v.x, 0.0, v.y);
        let z3d = |v: Vec2| Vec3::new(v.x, v.y, 0.0);
        let segment_rotation = Mat2::from_angle(TAU / 4.0 / density as f32);
        let circle = |start| iter::successors(Some(start), |v| Some(segment_rotation * *v));
        let half_circle = |radius: f32| circle(-Vec2::X * radius).take(density * 2 + 1);
        let full_circle = |radius: f32| circle(-Vec2::X * radius).take(density * 4 + 1);
        match_3d!(self.shape, {
            Shape2d(shape) => outline_2d(shape, density).into_iter().map(z3d).collect(),
            Pyramid { ref base, height } => {
                let half_height = Vec3::Y * height / 2.0;
                let base_shape = outline_2d(base, density);
                let segment_to_top = |v: &Vec2| [y3d(*v) - half_height, half_height, Vec3::NAN];
                chain_segments![
                    base_shape.iter().map(|v| y3d(*v) - half_height),
                    base_shape.iter().flat_map(segment_to_top),
                ]
            },
            Extruded { ref base, height } => {
                let half_height = Vec3::Y * height / 2.0;
                let segment_to_top =
                    |v: &Vec2| [y3d(*v) - half_height, y3d(*v) + half_height, Vec3::NAN];
                let base_shape = outline_2d(base, density);
                chain_segments![
                    base_shape.iter().map(|v| y3d(*v) - half_height),
                    base_shape.iter().map(|v| y3d(*v) + half_height),
                    base_shape.iter().flat_map(segment_to_top),
                ]
            },
            Tetrahedron { a, b, c, d } => vec![a, b, c, a, d, c, Vec3::NAN, d, b],
            Sphere { radius } => chain_segments![
                full_circle(radius).map(y3d),
                full_circle(radius).map(x3d),
                full_circle(radius).map(z3d),
            ],
            HalfSphere { radius } => {
                let y_half_circle = circle(Vec2::Y * -radius).take(density * 2 + 1);
                chain_segments![
                    full_circle(radius).map(y3d),
                    half_circle(-radius).map(z3d),
                    y_half_circle.map(x3d),
                ]
            },
            Capsule { radius, segment_height } => {
                let [x, y, z] = Vec3::AXES;
                let offset = y * segment_height * 0.5;
                let top_joint = |extremum| offset + extremum * radius;
                let bot_joint = |extremum| -offset + extremum * radius;
                let y_half_circle = |r| circle(Vec2::Y * r).take(density * 2 + 1);
                // TODO: could be reduced to 3 Segments (instead of 10) by
                // merging the top & bottom half circles and connecting the
                // joining longitudes. This would only save uploading to GPU 7 Vec3
                chain_segments![
                    // top
                    y_half_circle(-radius).map(|v| x3d(v) + offset),
                    full_circle(radius).map(|v| y3d(v) + offset),
                    half_circle(-radius).map(|v| z3d(v) + offset),
                    // bottom
                    y_half_circle(radius).map(|v| x3d(v) - offset),
                    full_circle(radius).map(|v| y3d(v) - offset),
                    half_circle(radius).map(|v| z3d(v) - offset),
                    // Joins top to bottom
                    [
                        top_joint(x), bot_joint(x), Vec3::NAN,
                        top_joint(-x), bot_joint(-x), Vec3::NAN,
                        top_joint(z), bot_joint(z), Vec3::NAN,
                        top_joint(-z), bot_joint(-z),
                    ]
                ]
            },
            Cuboid { size } => {
                let xy = size.xy() / 2.0;
                let a = xy;
                let b = xy * Vec2::new(1.0, -1.0);
                let c = -a;
                let d = -b;
                let hz = size.z / 2.0; // Half-depth
                let back = |v| z3d(v) - Vec3::Z * hz;
                let front = |v| z3d(v) + Vec3::Z * hz;
                vec![
                    // back
                    back(a), back(b), back(c), back(d), back(a), front(a), Vec3::NAN,
                    // front
                    front(b), front(c), front(d), front(a), front(b), back(b), Vec3::NAN,
                    // Transversal edges (a and b were sneacked in previous lines)
                    back(c), front(c), Vec3::NAN, back(d), front(d),
                ]
            },
            // Shape::RoundedRectangle { size, bevel } => { }
            // Shape::RoundedCuboid { size, bevel } => { }
            Cone { height, base_radius } => {
                let half_height = Vec3::Y * height / 2.0;
                let start = -Vec2::X * base_radius;
                let circle = full_circle(base_radius);
                let segment_rotation = Mat2::from_angle(TAU / (density + 1) as f32);
                let low_res_circle = iter::successors(Some(start), |v| Some(segment_rotation * *v));
                let segment_to_top = |v| [y3d(v) - half_height, half_height, Vec3::NAN];
                chain_segments![
                    circle.map(|v| y3d(v) - half_height),
                    low_res_circle.take(density + 1).flat_map(segment_to_top),
                ]
            },
            Lines { ref points, ref lines } => lines
                .iter()
                .flat_map(|[a, b]| [points[*a], points[*b], Vec3::NAN])
                .collect(),
            Cylinder { height, radius } => {
                let half_height = Vec3::Y * height / 2.0;
                let start = -Vec2::X * radius;
                let segment_rotation = Mat2::from_angle(TAU / (density + 1) as f32);
                let low_res_circle = iter::successors(Some(start), |v| Some(segment_rotation * *v));
                let segment_to_top = |v| [y3d(v) - half_height, y3d(v) + half_height, Vec3::NAN];
                chain_segments![
                    // base
                    full_circle(radius).map(|v| y3d(v) - half_height),
                    // top
                    full_circle(radius).map(|v| y3d(v) + half_height),
                    // transversals
                    low_res_circle.take(density + 1).flat_map(segment_to_top),
                ]
            },
            HeightField { ref heights, size } => {
                let x_len = heights.len();
                let y_len = heights[0].len();
                let x_coord = |x| size.x / ((x_len - 1) as f32) * (x as f32) - size.x / 2.0;
                let y_coord = |y| size.y / ((y_len - 1) as f32) * (y as f32) - size.y / 2.0;
                let heights = |x: usize| &heights[x];
                let point_at = |x, y| Vec3::new(x_coord(x), heights(x)[y], y_coord(y));

                // builds two segments, one containing the "zigzag" pattern
                // between each columns, going upward then downward, up, down etc.
                // This segment contains all the horizontal and diagonal lines
                let mut zig_zags = Vec::with_capacity((x_len - 1) * (2 * y_len + 1));
                // And one that contains the columns. In a single segment
                // thanks to going all the way up, then going one step to the
                // right then all the way down, repeating.
                let mut y_axises =
                    Vec::with_capacity((1 + y_len) * x_len + zig_zags.capacity() + 1);
                let mut x = 0;
                let mut y = 0;
                // This loop increments y until it reaches the upper bound,
                // then increments x, then decrements y until it reaches the
                // lower bound, then increment x, then increment y until etc.
                loop {
                    let y_len = y_len as isize;
                    if x != x_len - 1 {
                        zig_zags.push(point_at(x, y));
                        zig_zags.push(point_at(x + 1, y));
                    }
                    y_axises.push(point_at(x, y));

                    let y_direction = if x % 2 == 0 { 1 } else { -1 };
                    let iy = y as isize + y_direction;
                    let y_out_of_bound = iy >= y_len || iy < 0;

                    if y_out_of_bound && x >= x_len - 1 {
                        break;
                    } else if y_out_of_bound {
                        y_axises.push(Vec3::NAN);
                        x += 1;
                    } else {
                        y = iy as usize;
                    }
                }
                y_axises.push(Vec3::NAN);
                y_axises.extend(zig_zags);
                y_axises
            },
        })
    }
}
