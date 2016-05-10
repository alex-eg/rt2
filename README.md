Rust real-time raytracer
========================
![Fancy!](https://raw.githubusercontent.com/taptap/rt2/devel/doc/images/rt.png)

Build and run
-------------
```
$ cd rt
$ cargo build --release
$ cargo run --release
```

Runnig without `--release` argument produces debug binaries, which are much slower and much more debuggable.

Controls
--------

* Q - roll CCW
* E - roll CW
* Right/Left arrow - yaw
* Up/Down arrow - pitch
* WASD - move camera
