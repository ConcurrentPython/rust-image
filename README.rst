=======================================================
Rust Example: image processing with and without threads
=======================================================

This tiny project provides 2 sample Rust applicatons that unsharpen images:

src/bin/linear
  Processes images in an ``images`` directory sequentially.

src/bin/threaded
  Processes images in an ``images`` directory in separate threads.

Building
========

With a nightly Rust installation::

  cargo build --release

Running
=======

  ./target/release/linear
  ./target/release/threaded
