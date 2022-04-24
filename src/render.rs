use bevy::prelude::*;

use bevy_polyline::prelude::*;

use crate::shapes::Shape;

pub enum Display {
    Lines { line_color: Color, line_width: f32 },
    // LinesAndFill,
    // Fill,
}
#[derive(Component)]
pub struct DebugShape {
    shape: Shape,
    display: Display,
    density: usize,
}
impl DebugShape {
    pub fn new(shape: Shape, display: Display, density: usize) -> Self {
        Self { shape, display, density }
    }
}
impl Default for DebugShape {
    fn default() -> Self {
        let display = Display::Lines { line_color: Color::YELLOW, line_width: 2.0 };
        Self::new(Shape::Sphere { radius: 1.0 }, display, 4)
    }
}

/// Easy extension methods on [`Shape`] to quickly create a [`DebugShape`].
pub trait IntoRenderableShape {
    fn lines(self, line_color: Color, line_width: f32) -> DebugShape;
    // fn lines_and_fill(self, color: Color) -> DebugShape;
    // fn fill(self, color: Color) -> DebugShape;
}
impl IntoRenderableShape for Shape {
    fn lines(self, line_color: Color, line_width: f32) -> DebugShape {
        DebugShape {
            shape: self,
            display: Display::Lines { line_color, line_width },
            density: 4,
        }
    }
    // fn lines_and_fill(self, color: Color) -> DebugShape {
    //     DebugShape { shape: self, color, display: Display::LinesAndFill }
    // }
    // fn fill(self, color: Color) -> DebugShape {
    //     DebugShape { shape: self, color, display: Display::Fill }
    // }
}

#[derive(Component)]
pub(crate) struct LineMesh;
// How this works: Create many children to the Entity with a DebugShape
// component, each one a Polyline or a simple StandardMaterial with opacity
pub(crate) fn insert_debug_shapes(
    mut cmds: Commands,
    query: Query<(Entity, &DebugShape), Added<DebugShape>>,
    mut poly_mats: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    for (entity, debug) in query.iter() {
        match debug.display {
            Display::Lines { line_color, line_width } => {
                let material = poly_mats.add(PolylineMaterial {
                    width: line_width,
                    color: line_color,
                    perspective: true,
                });
                let vertices = debug.shape.segment(debug.density);
                cmds.entity(entity).with_children(|cmds| {
                    let bundle = PolylineBundle {
                        polyline: polylines.add(Polyline { vertices }),
                        material: material.clone(),
                        ..default()
                    };
                    cmds.spawn_bundle(bundle).insert(LineMesh);
                });
            }
        }
    }
}
