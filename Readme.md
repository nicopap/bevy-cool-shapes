# Bevy Cool Shapes

Bevy cool shapes is a collection of crates that provides shape primitives
mostly intended for debugging.

Currently, Bevy Cool Shapes includes two crates:
* `bevy_cool_shapes`: Defines 2d and 3d shape structs, as simple abstract things.
* `bevy_cool_debug_shapes`:
  * Extends the `bevy_cool_shapes` shapes with more concrete shapes with specified line
    counts, etc. And defines ways to easily define those shapes from the abstract
    `bevy_cool_shapes`.
  * The debug shapes can be rendered in the bevy world stuff.

## License

All crates in this repository are licensed under the terms of the Apache 2.0 license.
