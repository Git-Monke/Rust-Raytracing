![alt text](https://github.com/Git-Monke/Rust-Raytracing/raw/master/raytrace.png "Final product image")

## Rust Raytracing

This is a project I did over the weekend to teach myself Rust! I used [Raytracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) as my guide, and translated the C++ into Rust as I followed along. The image above is the final render after about 2 days of work. 

I didn't copy straight from Ray Tracing in One Weekend. In all portions of the math, like calculating ray intersections with the sphere or how light should bounce off, I tried to figure out how to code on my own given only the math. Rust has many quirks, so everything couldn't be translated 1-1 (for example, operator overloading). I also figured out how to do multithreading on my own to speed things up, but never GPU rendering.

Very fun project overall!
