mod bundles;
mod outline;
mod render;

pub mod prelude {
    pub use crate::bundles::ShapeOutlineBundle;
    pub use crate::outline::OutlineableShape;
    pub use crate::render::{IntoOutline, ShapeOutline};
    pub use crate::RenderableShapesPlugin;
    pub use bevy_cool_shapes::*;
}

use bevy::prelude::*;
use bevy_polyline::PolylinePlugin;

pub struct RenderableShapesPlugin;
impl Plugin for RenderableShapesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PolylinePlugin)
            .add_system(render::insert_outline)
            .add_system(render::update_outlines_visibility)
            .add_system(render::remove_outline)
            .add_system(render::update_outline);
    }
}
