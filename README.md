Rust real-time raytracer
========================
[![Build Status](https://travis-ci.org/alex-eg/rt2.svg?branch=master)](https://travis-ci.org/alex-eg/rt2)

![Fancy!](https://raw.githubusercontent.com/alex-eg/rt2/master/doc/images/rt.png)

Build and run
-------------

Make sure you have [**SDL2**](https://www.libsdl.org/) library installed.

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
