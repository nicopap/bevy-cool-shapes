# Renderable Bevy Cool shapes

A shapes library based on [RFC 12][rfc-12].

This crate implements traits and additional shapes that are ready to be rendered. It is based
on the abstract shape definitions available in `bevy-cool-shapes`.

Unlike a debugging library, this uses a retained mod API. In my experience, a retained mod API is
much easier to use for defining coherant shapes to place in the world than a direct mode API.

The library can be used to display shapes, currently you can only display shape outlines. It is
typically useful for debugging.

I might add later an immediate-mode API wrapper on top of the retained mod one, similar to how I did
it in [bevy-debug-text-overlay][debug-text-overlay].

## Usage

Add the following to your `Cargo.toml`:

```toml
bevy_cool_shapes_rendered = "0.1.0"
```

A sample tinny program would be:

```rust
use bevy::prelude::*;
use bevy_cool_shapes_render::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).add_plugin(RenderableShapesPlugin)
        .add_startup_system(setup).run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle::new_3d())
        .insert(Transform::from_xyz(25., 10., 25.).looking_at(Vec3::ZERO, Vec3::Y));
    commands.spawn_bundle(ShapeOutlineBundle {
        shape: OutlineableShape::from(Sphere { radius: 3.0 })
            .lines(Color::YELLOW, 200.0, 0.0),
        ..default()
    });
}
```

This will display a yellow sphere at the center of the screen as follow:

![A yellow sphere at the center of the screen](https://user-images.githubusercontent.com/26321040/165336561-a0915627-c908-4d77-a5a1-da21314ef45f.png)

That's it! 

[debug-text-overlay]: https://github.com/nicopap/bevy-debug-text-overlay/
[rfc-12]: https://github.com/bevyengine/rfcs/pull/12
