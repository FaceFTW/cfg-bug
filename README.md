# THIS IS NOT A BUG

I originally thought that using `cfg!()` macros in build scripts would provide conditional compilation for the _target triple_. Instead (by design that makes sense) _`cfg!()` in build scripts will apply based on the host machine and not the target_. 

Per this comment from @weihanglo in rust-lang/cargo#14881:
> To run your build script on your host machine, Cargo always compiles it to your host platform, which is x86_64-pc-windows-msvc. cfg! value is determined at "build script compile time" and resolved to x86_64-pc-windows-msvc.
>
> To get the target platform you main package compile to at "build script execution time", you'll need to read either TARGET or CARGO_CFG_TARGET_OS environment variable via std::env::var{_os}, which will be wasm32-unknown-unknown. See Environment variables Cargo sets for build scripts
>
> If you do want to compile your build script to a certain target platform, see the unstable feature artifact dependencies.

[Link to original comment](https://github.com/rust-lang/cargo/issues/14881#issuecomment-2510440862)

Keeping this up and archived as a reference for issue context, nothing more.
