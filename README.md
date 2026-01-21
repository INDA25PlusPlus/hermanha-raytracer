## Ray Tracer

![Rendered scene](images/high_res.png)

A simple Rust ray tracer from scratch that renders spheres and planes in different materials.

### Build Steps (very brief)

- Made vec3 and camera to shoot rays through each pixel.

![First sphere](images/first_sphere.png)

- Visualized surface normals as RGB for a gradient sphere.

![Normal visualization](images/colored_normal_sphere.png)

- Added point-sampled antialiasing.

![Antialiasing](images/antialized_spheres.png)

- Implemented diffuse shading, then fixed shadow acne.

![Diffuse](images/diffused_image.png)
![Diffuse (no acne)](images/diffuse_without_shadow_acne.png)

- True Lambertian materials.

![Lambertian](images/lambertian.png)

- Metals and planes for the final scene.

![High res](images/high_res.png)