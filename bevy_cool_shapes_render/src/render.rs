use bevy::prelude::*;

use crate::outline::OutlineableShape;
use bevy_polyline::prelude::*;

#[derive(Component)]
pub struct ShapeOutline {
    shape: OutlineableShape,
    color: Color,
    // TODO: actually update this when this is merged:
    // https://github.com/ForesightMiningSoftwareCorporation/bevy_polyline/pull/26
    #[allow(unused)]
    depth_bias: f32,
    width: f32,
}
impl Default for ShapeOutline {
    fn default() -> Self {
        ShapeOutline {
            shape: OutlineableShape::default(),
            color: Color::YELLOW,
            depth_bias: 0.0,
            width: 1.0,
        }
    }
}

/// Easy extension methods on [`Shape`] to quickly create a [`DebugShapeOutline`].
pub trait IntoOutline {
    fn lines(self, color: Color, width: f32, bias: f32) -> ShapeOutline;
}
impl IntoOutline for OutlineableShape {
    fn lines(self, color: Color, width: f32, bias: f32) -> ShapeOutline {
        ShapeOutline { shape: self, color, width, depth_bias: bias }
    }
}

/// Marks entites spawned by [`insert_debug_shapes`].
///
/// This let us recognize which child we care about and need to update.
#[derive(Component)]
pub(crate) struct LineMesh;

// How this works: Create many children to the Entity with a DebugShapeOutline
// component, each one a Polyline or a simple StandardMaterial with opacity
pub(crate) fn insert_outline(
    mut cmds: Commands,
    query: Query<(Entity, &ShapeOutline), Added<ShapeOutline>>,
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
pub(crate) fn update_outline(
    mut lines: Query<(&mut Handle<Polyline>, &mut Handle<PolylineMaterial>), With<LineMesh>>,
    mut poly_mats: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
    shapes: Query<(&Children, &ShapeOutline), Changed<ShapeOutline>>,
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

type OutlinesWithChangedVisibility = (Without<LineMesh>, With<ShapeOutline>, Changed<Visibility>);
pub(crate) fn update_outlines_visibility(
    mut lines: Query<&mut Visibility, With<LineMesh>>,
    visibilities: Query<(&Children, &Visibility), OutlinesWithChangedVisibility>,
) {
    for (children, new_vis) in visibilities.iter() {
        for child in children.iter() {
            if let Ok(mut visibility) = lines.get_mut(*child) {
                visibility.is_visible = new_vis.is_visible;
            }
        }
    }
}
pub(crate) fn remove_outline(
    mut cmds: Commands,
    lines: Query<(), With<LineMesh>>,
    children: Query<&Children>,
    removed: RemovedComponents<ShapeOutline>,
) {
    for parent in removed.iter() {
        for child in children.get(parent).into_iter().flat_map(|p| &**p) {
            if lines.get(*child).is_ok() {
                cmds.entity(*child).despawn();
            }
        }
    }
}
