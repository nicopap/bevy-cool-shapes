use bevy::prelude::*;

use crate::outline::DebugShape;
use bevy_polyline::prelude::*;

#[derive(Component)]
pub struct DebugShapeOutline {
    shape: DebugShape,
    color: Color,
    // TODO: actually update this when this is merged:
    // https://github.com/ForesightMiningSoftwareCorporation/bevy_polyline/pull/26
    #[allow(unused)]
    depth_bias: f32,
    width: f32,
}
impl Default for DebugShapeOutline {
    fn default() -> Self {
        DebugShapeOutline {
            shape: DebugShape::default(),
            color: Color::YELLOW,
            depth_bias: 0.0,
            width: 1.0,
        }
    }
}

/// Easy extension methods on [`Shape`] to quickly create a [`DebugShapeOutline`].
pub trait IntoRenderableShape {
    fn lines(self, color: Color, width: f32, bias: f32) -> DebugShapeOutline;
    // fn lines_and_fill(self, color: Color) -> DebugShapeOutline;
    // fn fill(self, color: Color) -> DebugShapeOutline;
}
impl IntoRenderableShape for DebugShape {
    fn lines(self, color: Color, width: f32, bias: f32) -> DebugShapeOutline {
        DebugShapeOutline { shape: self, color, width, depth_bias: bias }
    }
    // fn lines_and_fill(self, color: Color) -> DebugShapeOutline {
    //     DebugShapeOutline { shape: self, color, display: Display::LinesAndFill }
    // }
    // fn fill(self, color: Color) -> DebugShapeOutline {
    //     DebugShapeOutline { shape: self, color, display: Display::Fill }
    // }
}

#[derive(Component)]
pub(crate) struct LineMesh;
// How this works: Create many children to the Entity with a DebugShapeOutline
// component, each one a Polyline or a simple StandardMaterial with opacity
pub(crate) fn insert_debug_shapes(
    mut cmds: Commands,
    query: Query<(Entity, &DebugShapeOutline), Added<DebugShapeOutline>>,
    mut poly_mats: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    for (entity, debug) in query.iter() {
        let material = poly_mats.add(PolylineMaterial {
            width: debug.width,
            color: debug.color,
            perspective: true,
            // depth_bias: debug.depth_bias,
        });
        let vertices = debug.shape.outline();
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
