# Renderable Bevy Cool shapes

A shapes library based on RFC12.

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
todo!("Write a tinny example sample program and copy it here")
```


[debug-text-overlay]: https://github.com/nicopap/bevy-debug-text-overlay/
