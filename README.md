## Usage

Sidenote: This project inclues a `flake.nix` file to exactly reproduce my 
dev environment using the [Nix Package Manager](https://nixos.org/download).
This is an optional convenience, however it's use also effects the 
reproducability of the error in leptos-rs/leptos#3288 and may therefor shed
some light on the issue (see later).


```sh
git clone git@github.com:marcuswhybrow/leptos-issue-3288
cd leptos-issue-3288

# Optional Nix Development Shell
direnv allow # option 1: direnv autoloading of Nix devshell
nix develop  # option 2: manually enter Nix devesh (Ctrl+D to exit)

rustc --version # rustc 1.85.0-nightly (a47555110 2024-11-22)
cargo --version # cargo 1.85.0-nightly (66221abde 2024-11-19)
cargo-leptos serve --hot-reload
```

Then open http://localhost:3000 

The DOM will render...

```
VIDEO_COUNT: 1000
Loading
```

The Console will output...

```
Uncaught (in promise) ReferenceError: __RESOLVED_RESOURCES is not defined
    __wbg_static_accessor___RESOLVED_RESOURCES_f5c30e0828639f84 http://localhost:3000/pkg/my_app.js:832
    hydrate http://localhost:3000/pkg/my_app.js:223
    <anonymous> http://localhost:3000/:5
    promise callback* http://localhost:3000/:4
    promise callback* http://localhost:3000/:3
    <anonymous> http://localhost:3000/:9
my_app.js:832:13
```

and the server terminal will output...

```
thread 'tokio-runtime-worker' has overflowed its stack
fatal runtime error: stack overflow 
```

Change `./src/app.rs` line `42` `VIDEO_COUNT` from `1000` to `100` and the error goes away.

### Note: Building With Nix Increases VIDEO_COUNT Limit Before Error

You may find it useful to know that running the project via the aforementioned 
Nix Package Manager using it's `nix run` command, inreases the threshold at 
which VIDEO_COUNT cause a stack overflow.

I was able to increase the `VIDEO_COUNT` to `3000` without errors. `cargo run`
has the same error threshold as `cargo-leptos serve --hot-reload`. Why this is 
is not immediately obvious to me, but it suggests that the build process is 
effecting the error.
