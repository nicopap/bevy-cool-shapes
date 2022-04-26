use bevy::prelude::*;

use crate::render::ShapeOutline;

#[derive(Bundle, Default)]
pub struct ShapeOutlineBundle {
    pub shape: ShapeOutline,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}
