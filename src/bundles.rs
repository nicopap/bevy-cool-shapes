use bevy::prelude::*;

use crate::render::DebugShape;

#[derive(Bundle, Default)]
pub struct DebugShapeBundle {
    pub shape: DebugShape,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}
