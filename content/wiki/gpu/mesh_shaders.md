+++
title = "Mesh shaders"
+++

Mesh shaders are a compute-centric approach to producing geometry for rasterization.
They are significantly more powerful than the traditional vertex shader approach, and also simpler than the complex zoo of shader types (hull, domain, tesselation, geometry) that were originally proposed to overcome the limitations of simple vertex shading.

They were originally [introduced][Introduction to Turing Mesh Shaders] by Nvidia in 2018 in the Turing microarchitecture.
In Vulkan, the capability was originally an Nvidia-specific extension, but is now the cross-platform [VK_EXT_mesh_shader] extension.
As of early 2024, representative lower-bound cards include GTX 1650, AMD Radeon RX 6400 (RDNA 2), and Intel Arc A370.
It is added to Metal 3 (2022) and hardware support is new in M3 and A17 (though the example project will run on M1 and A14).
The feature is present in [DirectX 12 Ultimate], released in 2020.
It is not yet available in WebGPU ([gpuweb#3015] is the feature request).

Mesh shaders are a fusion of compute and traditional rasterization.
They bring three new capabilities to the compute side, and of course the ability to drive the rasterization pipeline.

The three new compute-only capabilities are:

* Ability to spawn variable amounts of work
* Communication through two shader stages through a queue
* Ordering of elements processed through the shader stages

These are potentially of interest even with rasterization disabled.
Mainstream usage includes generating primitives (usually triangles) from the second stage.
These are then rasterized (in hardware) and shaded using fragment shaders in exactly the same way as the traditional vertex/fragment pipeline.

The main motivation for mesh shaders is improving the efficiency and flexibility of traditional 3D rasterization, most prominently culling of larger chunks of unneeded geometry, dynamic level-of-detail (LOD) selection, and procedural generation of geometry.
These uses build on the use of compute shaders for these tasks, advocated in Graham Wihlidal's GDC 2016 talk, [Optimizing the Graphics Pipeline with Compute].
Mesh shaders have a number of advantages over using a compute shader to fill a vertex buffer: you don't have to allocate the intermediate vertex buffer (which can fail if the size is not estimated correctly), there's less traffic to device memory as the queues mostly live in shared memory, and there's no pipeline barrier between the compute and rasterization stages.

## Basic computational model

There are two stages, but the first stage is optional.
Following the GPU ecosystem tradition of diversity of terminology, the first stage is called a "task shader" in Vulkan, an "amplification shader" in DirectX, and an "object shader" in Metal.
The second stage is called a mesh shader.


The first stage is dispatched like a compute shader.
There is an output user-defined "payload" structure (anologous to the interface between vertex and fragment shaders, but can be larger; on DirectX 12 and Metal the limit is 16k), and then each workgroup dispatches a variable number of workgroups of the mesh shader.
That number can be zero.
On Metal, it is limited to 1024 workgroups.

The second stage (the mesh shader) accepts the input from the task shader, and can also access the index of the parent task shader workgroup that spawned it.
There are two arrays in workgroup shared memory, one for vertices, the other for vertex indices for primitives.
On output, the vertex and primitive counts are set, up to the maximum allocation of the arrays.
Nvidia recommends 64 vertices and 126 primitives, though (as is typical in the GPU ecosystem) the exact limits vary.
On Metal, the limits are 256 vertices and 512 primitives.

The primitive is usually a triangle, but all implementations support lines, and Vulkan and Metal support points as well.
For triangles, there are 3 vertex indices per primitive.

There is a per-vertex user defined struct (essentially the same as the output of a vertex shader) and also a new per-primitive user-defined struct.

### Execution model

On Nvidia, there is a strong forward progress guarantee, and the workgroups of a mesh shader are launched in order.
Thus, doing a single-pass scan inside the mesh shader is viable (source: launch ordering is discussed at 24:18 and the single-pass scan is at 43:59 of the [Turing Mesh Shaders] video).

TODO: find out ordering guarantees on other hardware.
Presumably it's weak on Apple.

### Performance

The [Vulkan blog][Mesh Shading for Vulkan] warns that portability in performance across hardware is difficult.

The [Mesh shaders: Optimization and best practices] page from AMD contains recommendations and quantitative measurements of how to tune mesh shader usage.
In addition, Timur's blog post [How mesh shaders are implemented in an AMD driver] gives insight into their implementation (at least on AMD).

## Resources

These are authoritative references and particularly helpful documents to explain mesh shaders.

* [Introduction to Turing Mesh Shaders] from Nvidia
   + [Turing Mesh Shaders] at Siggraph 2018 (video)
* [Mesh Shading for Vulkan], Khronos blog
* [Reinventing the Geometry Pipeline]: Mesh Shaders in DirectX 12 (video)
* [Transform your geometry with Metal mesh shaders], WWDC 2022 (video)
* [DirectX Mesh Shader] spec

[Introduction to Turing Mesh Shaders]: https://developer.nvidia.com/blog/introduction-turing-mesh-shaders/
[VK_EXT_mesh_shader]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_mesh_shader.html
[DirectX 12 Ultimate]: https://devblogs.microsoft.com/directx/announcing-directx-12-ultimate/
[gpuweb#3015]: https://github.com/gpuweb/gpuweb/issues/3015
[Optimizing the Graphics Pipeline with Compute]: https://www.gdcvault.com/play/1023109/Optimizing-the-Graphics-Pipeline-With
[Turing Mesh Shaders]: https://www.youtube.com/watch?v=Ge427_2VORo
[Mesh Shading for Vulkan]: https://www.khronos.org/blog/mesh-shading-for-vulkan
[Reinventing the Geometry Pipeline]: https://www.youtube.com/watch?v=CFXKTXtil34
[Mesh shaders: Optimization and best practices]: https://gpuopen.com/learn/mesh_shaders/mesh_shaders-optimization_and_best_practices/
[How mesh shaders are implemented in an AMD driver]: https://timur.hu/blog/2022/how-mesh-shaders-are-implemented
[Transform your geometry with Metal mesh shaders]: https://developer.apple.com/videos/play/wwdc2022/10162/
[DirectX Mesh Shader]: https://microsoft.github.io/DirectX-Specs/d3d/MeshShader.html
