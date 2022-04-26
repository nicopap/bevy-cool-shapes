# Bevy Cool debug shapes

A shapes library based on RFC12.

Unlike other debugging library, this uses a retained mod API. In my experience, a retained mod API is
much easier to use for defining coherant shapes to place in the world than a direct mode API.

I might add later an immediate-mode API wrapper on top of the retained mod one, similar to how I did
it in [bevy-debug-text-overlay][debug-text-overlay].

[debug-text-overlay]: https://github.com/nicopap/bevy-debug-text-overlay/
