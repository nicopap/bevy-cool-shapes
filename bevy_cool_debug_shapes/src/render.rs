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
}
impl IntoRenderableShape for DebugShape {
    fn lines(self, color: Color, width: f32, bias: f32) -> DebugShapeOutline {
        DebugShapeOutline { shape: self, color, width, depth_bias: bias }
    }
}

/// Marks entites spawned by [`insert_debug_shapes`].
///
/// This let us recognize which child we care about and need to update.
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
                material,
                ..default()
            };
            cmds.spawn_bundle(bundle).insert(LineMesh);
        });
    }
}
pub(crate) fn update_debug_shapes(
    mut lines: Query<(&mut Handle<Polyline>, &mut Handle<PolylineMaterial>), With<LineMesh>>,
    mut poly_mats: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
    shapes: Query<(&Children, &DebugShapeOutline), Changed<DebugShapeOutline>>,
) {
    for (children, debug) in shapes.iter() {
        for child in children.iter() {
            if let Ok((mut polyline, mut poly_mat)) = lines.get_mut(*child) {
                *poly_mat = poly_mats.add(PolylineMaterial {
                    width: debug.width,
                    color: debug.color,
                    perspective: true,
                    // depth_bias: debug.depth_bias,
                });
                let vertices = debug.shape.outline();
                *polyline = polylines.add(Polyline { vertices });
            }
        }
    }
}
#[allow(clippy::type_complexity)]
pub(crate) fn update_debug_shapes_visibility(
    mut lines: Query<&mut Visibility, With<LineMesh>>,
    visibilities: Query<(&Children, &Visibility), (With<DebugShapeOutline>, Changed<Visibility>)>,
) {
    for (children, new_vis) in visibilities.iter() {
        for child in children.iter() {
            if let Ok(mut visibility) = lines.get_mut(*child) {
                visibility.is_visible = new_vis.is_visible;
            }
        }
    }
}
pub(crate) fn remove_debug_shapes(
    mut cmds: Commands,
    lines: Query<(), With<LineMesh>>,
    children: Query<&Children>,
    removed: RemovedComponents<DebugShapeOutline>,
) {
    for parent in removed.iter() {
        for child in children.get(parent).into_iter().flat_map(|p| &**p) {
            if lines.get(*child).is_ok() {
                cmds.entity(*child).despawn();
            }
        }
    }
}
