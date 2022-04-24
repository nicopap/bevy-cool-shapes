use std::f32::consts::TAU;
use std::iter;

use bevy_math::{Mat2, Vec2, Vec3, Vec3Swizzles};

// pub(crate) struct Triangle {
//     pub(crate) a: Vec3,
//     pub(crate) b: Vec3,
//     pub(crate) c: Vec3,
// }

#[derive(Debug, Clone)]
pub enum Shape2d {
    Circle {
        radius: f32,
    },
    /// Upper half of a circle.
    HalfCircle {
        radius: f32,
    },
    /// +x/+y quadrant of a circle.
    QuarterCircle {
        radius: f32,
    },
    Rectangle {
        size: Vec2,
    },
    // /// Rectangle of size `size` with bevelled edge.
    // RoundedRectangle { size: Vec2, bevel: f32 },
    Triangle {
        a: Vec2,
        b: Vec2,
        c: Vec2,
    },
    Polygon {
        points: Vec<Vec2>,
        lines: Vec<[usize; 2]>,
    },
}

#[derive(Debug, Clone)]
pub enum Shape {
    /// A two dimensional shape as defined in [`Shape2d`].
    Shape2d(Shape2d),
    // A Pyramid with a base of arbitrary shape.
    //
    // Note that [`Shape::Cylinder`] is a special case of this, where the
    // number of edges is minimized.
    Pyramid {
        base: Shape2d,
        height: f32,
    },
    /// A 2d shape extruded on its perpendicular axis, to form a 3d shape.
    ///
    /// Note: [`Shape::Cuboid`] and [`Shape::Cylinder`] are special cases of a
    /// shape extrusion.
    Extruded {
        base: Shape2d,
        height: f32,
    },
    Sphere {
        radius: f32,
    },
    /// Upper half of a sphere.
    HalfSphere {
        radius: f32,
    },
    Capsule {
        radius: f32,
        segment_height: f32,
    },
    /// 3d rectangle specified as the full length of each edge.
    Cuboid {
        size: Vec3,
    },
    // /// Cuboid of size `size` with bevelled edge.
    // RoundedCuboid { size: Vec3, bevel: f32 },
    Cone {
        height: f32,
        base_radius: f32,
    },
    /// A four-cornered 3d shape.
    Tetrahedron {
        a: Vec3,
        b: Vec3,
        c: Vec3,
        d: Vec3,
    },
    /// A Collection of 3d lines.
    Mesh {
        /// Points in 3d space.
        points: Vec<Vec3>,
        /// The indexes in `points` of the line vertices.
        lines: Vec<[usize; 2]>,
    },
    Cylinder {
        height: f32,
        radius: f32,
    },
    /// 3d plane with varying height based on a grid.
    HeightField {
        heights: Vec<Vec<f32>>,
        size: Vec2,
    },
}

impl Shape2d {
    pub fn segment(&self, density: usize) -> Vec<Vec2> {
        let segment_rotation = Mat2::from_angle(TAU / 4.0 / density as f32);
        let circle = |start| iter::successors(Some(start), |v| Some(segment_rotation * *v));
        let half_circle = |radius: f32| circle(-Vec2::X * radius).take(density * 2 + 1);
        let full_circle = |radius: f32| circle(-Vec2::X * radius).take(density * 4 + 1);
        match *self {
            Self::Circle { radius } => full_circle(radius).collect(),
            Self::HalfCircle { radius } => half_circle(-radius).collect(),
            Self::QuarterCircle { radius } => circle(Vec2::Y * radius).take(density + 1).collect(),
            Self::Triangle { a, b, c } => vec![a, b, c, a],
            Self::Polygon { ref points, ref lines } => lines
                .iter()
                .flat_map(|[a, b]| [points[*a], points[*b], Vec2::NAN])
                .collect(),
            Self::Rectangle { size } => {
                let xy = size / 2.0;
                let a = xy;
                let b = xy * Vec2::new(1.0, -1.0);
                vec![a, b, -a, -b, a]
            }
        }
    }
}
impl Shape {
    pub fn segment(&self, density: usize) -> Vec<Vec3> {
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
        let x3d = |v: Vec2| Vec3::new(0.0, v.x, v.y);
        let y3d = |v: Vec2| Vec3::new(v.x, 0.0, v.y);
        let z3d = |v: Vec2| Vec3::new(v.x, v.y, 0.0);
        let segment_rotation = Mat2::from_angle(TAU / 4.0 / density as f32);
        let circle = |start| iter::successors(Some(start), |v| Some(segment_rotation * *v));
        let half_circle = |radius: f32| circle(-Vec2::X * radius).take(density * 2 + 1);
        let full_circle = |radius: f32| circle(-Vec2::X * radius).take(density * 4 + 1);
        match *self {
            Self::Shape2d(ref shape) => shape.segment(density).into_iter().map(z3d).collect(),
            Self::Pyramid { ref base, height } => {
                let half_height = Vec3::Y * height / 2.0;
                let base_shape = base.segment(density);
                let segment_to_top = |v: &Vec2| [y3d(*v) - half_height, half_height, Vec3::NAN];
                chain_segments![
                    base_shape.iter().map(|v| y3d(*v) - half_height),
                    base_shape.iter().flat_map(segment_to_top),
                ]
            }
            Self::Extruded { ref base, height } => {
                let half_height = Vec3::Y * height / 2.0;
                let segment_to_top =
                    |v: &Vec2| [y3d(*v) - half_height, y3d(*v) + half_height, Vec3::NAN];
                let base_shape = base.segment(density);
                chain_segments![
                    base_shape.iter().map(|v| y3d(*v) - half_height),
                    base_shape.iter().map(|v| y3d(*v) + half_height),
                    base_shape.iter().flat_map(segment_to_top),
                ]
            }
            Self::Tetrahedron { a, b, c, d } => vec![a, b, c, a, d, c, Vec3::NAN, d, b],
            Self::Sphere { radius } => chain_segments![
                full_circle(radius).map(y3d),
                full_circle(radius).map(x3d),
                full_circle(radius).map(z3d),
            ],
            Self::HalfSphere { radius } => {
                let y_half_circle = circle(Vec2::Y * -radius).take(density * 2 + 1);
                chain_segments![
                    full_circle(radius).map(y3d),
                    half_circle(-radius).map(z3d),
                    y_half_circle.map(x3d),
                ]
            }
            #[rustfmt::skip]
            Self::Capsule { radius, segment_height } => {
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
            }
            #[rustfmt::skip]
            Self::Cuboid { size } => {
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
            }
            // Self::RoundedRectangle { size, bevel } => { }
            // Self::RoundedCuboid { size, bevel } => { }
            Self::Cone { height, base_radius } => {
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
            }
            Self::Mesh { ref points, ref lines } => lines
                .iter()
                .flat_map(|[a, b]| [points[*a], points[*b], Vec3::NAN])
                .collect(),
            Self::Cylinder { height, radius } => {
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
            }
            Self::HeightField { ref heights, size } => {
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
            }
        }
    }
}
