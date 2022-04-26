use bevy::prelude::*;

use crate::render::DebugShapeOutline;

#[derive(Bundle, Default)]
pub struct DebugShapeBundle {
    pub shape: DebugShapeOutline,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}
