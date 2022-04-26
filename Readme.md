# Bevy Cool Shapes

Bevy cool shapes is a collection of crates that provides shape primitives
mostly intended for debugging.

Currently, Bevy Cool Shapes includes two crates:
* [`bevy_cool_shapes`][bevy-cool-shapes]: Defines 2d and 3d shape structs, as simple abstract things.
* [`bevy_cool_shapes_render`][bevy-cool-shapes-render]:
  * Extends the `bevy_cool_shapes` shapes with more concrete shapes with specified line
    counts, etc. And defines ways to easily define those shapes from the abstract
    `bevy_cool_shapes`.
  * Currently supports rendering shapes as collections of line segments.
  
Please check the readme of those directories for detailed instructions and examples
on how to use them.

FYI you can just clone this repository and run `cargo run --example wireframe_demo` in
the root directory of the repository.

[bevy-cool-shapes]: ./bevy_cool_shapes
[bevy-cool-shapes-render]: ./bevy_cool_shapes_render

## License

All crates in this repository are Copyright (c) Nicola Papale 2022, and licensed under
the terms of the Apache 2.0 license.
