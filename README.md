# Untitled Rust Raytracer

So I've been teaching myself a hard programming language and thought to myself: "How do I make it _even harder_?" That's when I decided to write this raytracer. And then I did.

It's not perfect by any means, but it's mine, and I like it.

If you want to try it out, here's what you should do.

0. Install Rust.
1. Clone this repo.
1. Run `cargo run --release`.
1. There is no step 3.

Wait a few seconds and you should get a PNG file.

## TODO

-   General code clean up. Eliminate copy pasta and C-isms (return by writing to a parameter).
-   Make `pbrt::prelude` more useful. Buff `Vector` and `Point` with conversions, casting and general.
-   Command-line arguments support: render size, samples per pixel, output file name. Editing the source just to move the camera is silly.
-   Fearless concurrency.
-   More features: emissive materials and lights, more `Shape` types (quadrics, planes, boxes).
-   Non-projective cameras.
-   Transforms and animations.
-   SIMD/GPGPU support, benchmarks.

## Credits

Half of this raytracer was written under inspiration from [_Physically-Based Rendering: From Theory To Implementation_][pbr-book]. It's awesome and very well written, but you have to basically copy all of it to get any pixels on the screen.

Another half was hacked together in a few hours, closely following the steps of [_Ray Tracing in One Weekend_][rtiow]. It is the opposite to _PBR_ book in a way that it's hacky, but you get the result right away.

My raytracer borrows from both of these books, and Rust compiler allows it. (Get it? Because borrow checker!)

[pbr-book]: http://www.pbr-book.org/
[rtiow]: https://raytracing.github.io/
