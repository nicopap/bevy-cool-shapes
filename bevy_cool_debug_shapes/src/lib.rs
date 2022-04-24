mod bundles;
mod render;

pub mod prelude {
    pub use crate::bundles::DebugShapeBundle;
    pub use crate::render::DebugShape;
    pub use crate::render::IntoRenderableShape;
    pub use crate::DebugShapesPlugin;
    pub use bevy_cool_shapes::{Shape2d, Shape};
}

use bevy::prelude::*;
use bevy_polyline::PolylinePlugin;

pub struct DebugShapesPlugin;
impl Plugin for DebugShapesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PolylinePlugin)
            .add_system(render::insert_debug_shapes);
    }
}
