# Config-inator

## Purpose

~~Take over the entire tri-state area~~ Provide an example of how to integrate a Rust library into existing Java and Haskell codebases.

Use a core Rust library to read configuration files and then have thin, language-specific wrappers over top.

## Organization

There is a top-level Cargo workspace, so you can build all of the Rust code with `cargo build` and test with `cargo test`.

### `configinator`

This is the core Rust library. Currently, it matches the configuration defined in the [Lift documentation](https://help.sonatype.com/lift/configuring-lift). Eventually, this will support finding all of the configurations relevant when running in a specific folder, merging configurations, and other core features

### `configinator-jni`

This is the Rust FFI library written with the [`jni-rs`](https://github.com/jni-rs/jni-rs) crate to expose a JNI compatible interface for use in Java.

The intended-use pattern is:

1. Instantiate a configuration by passing in a file path
   * This will return `null` if the configuration cannot be found
   * This will `panic` on other errors (will be refactored to throw a Java exception)
   * This will return the pointer if the configuration is successfully found and parsed (this assumes we are running on an architecture with a 64-bit pointer size)
2. Retrieve configuration values by passing in the pointer to the configuration
   * This will be refactored to better handle being passed a null or invalid pointer
3. When done with the configuration, pass the pointer back to Rust for the memory to be freed

Research should be done on the various potential failure modes and how best to handle them

### `configinator-java`

This is the Java wrapper library that loads the JNI library and wraps it in a nicer interface.

This implements the intended-use pattern described in the previous section through the use of a `private` constructor, `static` from-file construction method, and `close` / `finalize` methods to ensure that memory is cleaned up when the resource goes out of scope or the garbage collector runs.

Currently, the tests define the `LD_LIBRARY_PATH` in [`configinator-java/lib/build.gradle`](configinator-java/lib/build.gradle) to be the `debug` output folder of the Cargo workspace `target`. This requires that `cargo build` is run before doing any Java work. There are a few different ways to approach managing the relationship between the native library and the Java project and each has pros & cons, so further research is required.

## Requirements

* Rust / Cargo ([rustup](https://rustup.rs/) is the recommended way to install & manage Rust)
* Java >= 11 ([sdkman](https://sdkman.io/) is a decent way to install & manage Java)
