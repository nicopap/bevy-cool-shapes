mod bundles;
mod outline;
mod render;

pub mod prelude {
    pub use crate::bundles::DebugShapeBundle;
    pub use crate::outline::DebugShape;
    pub use crate::render::{DebugShapeOutline, IntoRenderableShape};
    pub use crate::DebugShapesPlugin;
    pub use bevy_cool_shapes::*;
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
