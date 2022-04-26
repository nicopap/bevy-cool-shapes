use bevy_math::{Vec2, Vec3};

#[derive(Debug, Clone)]
pub struct Disc {
    pub radius: f32,
}
/// Upper half of a circle.
#[derive(Debug, Clone)]
pub struct HalfDisc {
    pub radius: f32,
}
/// +x/+y quadrant of a circle.
#[derive(Debug, Clone)]
pub struct QuarterDisc {
    pub radius: f32,
}
#[derive(Debug, Clone)]
pub struct Rectangle {
    pub size: Vec2,
}
// /// Rectangle of size `size` with bevelled edge.
// #[derive(Debug, Clone)]
// pub struct RoundedRectangle {
//     pub size: Vec2,
//     pub bevel: f32,
// }
#[derive(Debug, Clone)]
pub struct Triangle {
    pub a: Vec2,
    pub b: Vec2,
    pub c: Vec2,
}
#[derive(Debug, Clone)]
pub struct Polygon {
    pub points: Vec<Vec2>,
    pub lines: Vec<[usize; 2]>,
    pub tris: Vec<[usize; 3]>,
}

// TODO: This exist only for `Pyramid` and `Extruded`. How would I go about
// making them generic over which shape specifically we accept as base?
// * Create a `Shape2dBase` trait that returns a `Polygon`, store `base: Polygon`
//   in Pyramid, have helper method to build a Pyramid directly from a base
//   shape using that trait.
//   => This wouldn't work with Disc => Joly good, that's why we have Cylinder
//   => Wouldn't work with RoundedRectangle too
//   => Maybe have the `density` parameter to `Shape2dBase`?
// * Make Pyramid generic over the base shape, Only accept base shape that
//   implement a 2dShape trait of sort, worry about concrete implementations
//   only when relevant, bounding over T impl for Pyramid<T>
#[derive(Debug, Clone)]
pub enum Shape2d {
    Disc(Disc),
    HalfDisc(HalfDisc),
    QuarterDisc(QuarterDisc),
    Rectangle(Rectangle),
    // RoundedRectangle(RoundedRectangle),
    Triangle(Triangle),
    Polygon(Polygon),
}

/// A Pyramid with a base of arbitrary shape.
///
/// Note that [`Shape::Cylinder`] is a special case of this, where the
/// number of edges is minimized.
#[derive(Debug, Clone)]
pub struct Pyramid {
    pub base: Shape2d,
    pub height: f32,
}
/// A 2d shape extruded on its perpendicular axis, to form a 3d shape.
///
/// Note: [`Shape::Cuboid`] and [`Shape::Cylinder`] are special cases of a
/// shape extrusion.
#[derive(Debug, Clone)]
pub struct Extruded {
    pub base: Shape2d,
    pub height: f32,
}
#[derive(Debug, Clone)]
pub struct Sphere {
    pub radius: f32,
}
/// Upper half of a sphere.
#[derive(Debug, Clone)]
pub struct HalfSphere {
    pub radius: f32,
}
#[derive(Debug, Clone)]
pub struct Capsule {
    pub radius: f32,
    pub segment_height: f32,
}
/// 3d rectangle specified as the full length of each edge.
#[derive(Debug, Clone)]
pub struct Cuboid {
    pub size: Vec3,
}
// /// Cuboid of size `size` with bevelled edge.
// #[derive(Debug, Clone)]
// pub struct RoundedCuboid {
//     pub size: Vec3,
//     pub bevel: f32,
// }
#[derive(Debug, Clone)]
pub struct Cone {
    pub height: f32,
    pub base_radius: f32,
}
/// A four-cornered 3d shape.
#[derive(Debug, Clone)]
pub struct Tetrahedron {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
    pub d: Vec3,
}
/// A Collection of 3d lines.
#[derive(Debug, Clone)]
pub struct Lines {
    /// Points in 3d space.
    pub points: Vec<Vec3>,
    /// The indexes in `points` of the line vertices.
    pub lines: Vec<[usize; 2]>,
}
#[derive(Debug, Clone)]
pub struct Cylinder {
    pub height: f32,
    pub radius: f32,
}
/// 3d plane with varying height based on a grid.
#[derive(Debug, Clone)]
pub struct HeightField {
    pub heights: Vec<Vec<f32>>,
    pub size: Vec2,
}
