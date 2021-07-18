# Rust implementation of "Ray Tracing in One Weekend"


[Ray Tracing in One Weekend][rt-book] is a series of books by Peter Shirley showcasing implementation of a simple ray tracer. It's a good introduction and for the most part you can just copy the code. [I've][webfx] written [many real-time][tfx] graphic [projects][anim], but never a ground up offline ray tracer. I admit that I've already knew most of the math behind it, so the implementation was quick and painless.


![Sample image](/src/scenes/scene7.optix.png)
*Scene 7 after using optix denoiser. See below for raw image.*


# Scope

## Book 1

* render the sphere - **DONE**
* surface normals - **DONE**
* antialiasing - **DONE**
* diffuse materials - **DONE**
* metal materials - **DONE**
* dielectrics materials - **DONE**
* camera aperture - **DONE**

## Book 2

* motion blur - nope, I'm fundamentaly opposed to this feature
* Bounding Volume Hierarchies - **DONE**
* textures - **DONE**
* image textures - **DONE**
* lights - **DONE**
* instancing/transforms - **DONE**, actually implemented as matrix transforms
* volumes - **DONE**

## Additional improvements

* parallel execution - using [rayon][rayon], task per pixel
* transform matrices - book only introduces hardcoded single-axis rotations and simple translation


# Sample scenes

More sample scenes are in [src/scenes][more-scenes]. Do not forget to use release build: `cargo build --release`!


### Scene 1 - with camera aperture
![Scene 1](/src/scenes/scene1.png)

### Scene 2 - with materials (metal, glass, lambert diffuse)
![Scene 2](/src/scenes/scene2.png)

### Scene 3 - with BVH tests
![Scene 3](/src/scenes/scene3.png)

### Scene 4 - with textures
![Scene 4](/src/scenes/scene4.png)

### Scene 5 - with lights
![Scene 5](/src/scenes/scene5.png)

### Scene 6 - with simple transform
![Scene 6](/src/scenes/scene6.png)

### Scene 7 - with Cornell box
![Scene 7](/src/scenes/scene7.png)

### Scene 8 - with transforms playground
![Scene 8](/src/scenes/scene8.png)



[rt-book]: https://raytracing.github.io/
[rayon]: https://docs.rs/rayon/1.5.1/rayon/
[webfx]: https://github.com/Scthe/WebFX
[anim]: https://github.com/Scthe/Animation-workshop
[tfx]: https://github.com/Scthe/TressFX-OpenGL
[more-scenes]: src/scenes



