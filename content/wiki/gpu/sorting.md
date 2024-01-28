+++
title = "Sorting"
+++

Many rendering algorithms (including a proposed sparse strip technique for path rendering, and also Gaussian Splatting) rely on sorting.
Because the GPU has a different architecture to the CPU, programs running on the GPU have different performance characteristics, and this changes which sorting algorithms are optimal for a particular context.
In particular, sorting algorithms that exploit parallelism tend to be more suited to the GPU.
The literature on parallel sorting algorithms is extremely well developed, and there are many excellent implementations for CUDA.
In WebGPU, the situation is still evolving.
This page has pointers to the potentially useful resources for understanding existing implementations and developing new ones.

## Classes of sorting algorithms

The most promising sorting algorithms for GPU are merge sort and radix sort.
Within radix sort there are further distinctions, most notably least significant digit (LSD) vs most significant digit.

## Key size and segmentation

GPU sorting works best when the key is small and fixed-size.
For many rendering applications, this is a reasonable assumption.

In some cases (especially path rendering), a *segmented* sort may be a better choice than one that sorts the entire buffer.
An especially good writeup is [segmented sort] from Modern GPU, which builds on their [mergesort].

## LSD implementations

The LSD sort algorithm with the strongest claims to being fastest is [Onesweep].
This has an authoritative implementation in [CUB], for which a good starting point is [agent_radix_sort_onesweep].

A very popular sorting algorithm in gamedev circles is [FidelityFX sort] from AMD, for which the original source is HLSL.
This is has enough similarity to Onesweep that it is worth considering the differences.

* Onesweep uses a single pass scan for the digit histograms, while FidelityFX uses a traditional multi-dispatch tree reduction approach.
* Onesweep uses 8 bit digits (so 4 passes for a 32 bit key), while FidelityFX uses 4.
* Onesweep uses [warp-level multi-split] for ranking, while FidelityFX uses two 2-bit LSD passes.

Both original code bases use subgroups extensively.
The FidelityFX implementation works with a subgroup size of 16 or greater, and will produce incorrect results if deployed for a smaller subgroup size.
Onesweep depends on a hardcoded subgroup (warp) size of 32 and is difficult to make portable to GPUs with other subgroup sizes.

### WebGPU experiment

Raph did an [experiment](https://github.com/googlefonts/compute-shader-101/pull/31) of a hybrid algorithm largely based on FidelityFX, but adapted to WebGPU, and with a version of warp-level multi-split.
It achieves approximately 1G elements/s on M1 Max.
For radix sorts, element/s on large arrays (16M elements) is meaningful, as the algorithm scales linearly with the array size and is usually insensitive to the distribution of keys.

Like FidelityFX, it uses 4 bit digits and a tree reduction approach to histogram scan.
It's also basically patterned after FidelityFX in the structure of dispatches.

Because WebGPU doesn't have subgroups (yet), the warp-level multi-split algorithm fakes it using workgroup shared memory.
The terms "warp" and "lane" are borrowed from CUDA but in this case represent a more or less arbitrary partitioning.
A "warp" size of 16 was found to outperform 32, which is not surprising because the fake "ballot" operation to determine which neighbors in the warp hold the same key does linear iteration proportional to warp size.

Raph has also done experiments (code not yet published) porting this to Metal and using actual subgroup ballot operations.
That results in ~3G el/s, though some of the speedup is vanilla wgpu inefficiency.
Also note that to implement this algorithm correctly requires a subgroup barrier (`simdgroup_barrier` in Metal Shading Language), and that this primitive is not in the WebGPU subgroup proposal (see [gpuweb#4437](https://github.com/gpuweb/gpuweb/issues/4437)).
However, on M1 Max, placing a stronger threadgroup barrier is only a 2% slowdown, so this matter is not urgent.

Attempts to push this experiment to 8 bits per digit have not yet yielded sustainable performance improvement.

Several people have pointed out the Onesweep inspired sort from Lichtso, part of the [splatter] Gaussian Splat implementation.
However, as discussed in [splatter#2], it is an approximate sort only, and may rely on luck that the particular GPU will process atomics within a subgroup in order.
In addition, because it is one-pass, on GPUs without a forward progress guarantee (of which Apple Silicon is especially noticeable), the algorithm may deadlock or experience extended stalls.
The experiment mentioned above has neither of these shortcomings.
Note that it *does* achieve 8 bits per pass; it is entirely likely that a high performance implementation could draw inspiration from it, more so after subgroups land in WebGPU and thus real warp-level multi-split is possible.

### DeviceRadixSort

Aras Pranckevičius has been doing lots of experiments with sorting in his [UnityGaussianSplatting] implementation, all written in the Unity flavor of HLSL.
[UnityGaussianSplatting#82] adds something called DeviceRadixSort which shows a modest performance improvement (the discussion thread also speaks to the difficulty of implementing such things portably).
It uses 8 bit digits, and a subgroup implementation of warp-local multi-split.
This implementation is attempting to be portable to a range of subgroup sizes, but is untested on wave sizes other than 32 and is still under development.

The author, [Thomas Smith], has done experiments comparing reduce-then-scan against a chained single-pass approach for the global histogram scan, which suggest that the latter has approximately a 10% performance gain on Nvidia hardware, but pose considerable challenges for portability.
Single-pass approaches will not perform well on devices without a forward progress guarantee.

## Hybrid Radix Sort

[Hybrid Radix sort] is another strong contender, fundamentally based on an MSD approach but with additional mechanisms to reduce memory bandwidth.
It has better performance in some cases than Onesweep.
Like Onesweep, it manages 8 bits per pass.
That said, it is considerably more complicated, and it's also not clear how easily it would translate to WebGPU.

## Merge sort

Modern GPU has a well documented [mergesort], also readily adapted to [segmented sort].
It is older, and performance numbers from newer papers suggest that it has been surpassed.
That said, the segmented sort capability may be valuable.

[Forma] has a sorting implementation called [conveyor_sort].
This is a merge sort and is in vanilla WebGPU.
Performance has not been characterized yet.

## Bitonic sort

[Bitonic sort] is often proposed as it is conceptually fairly simple and the parallelism is easy to exploit, but when applied to large problems it is clear that the number of passes is unacceptably large; typically in the dozens where a radix sort would have 4 or 8.

## Discussion

There is active discussion of this issue in the [Sorting revisited](https://xi.zulipchat.com/#narrow/stream/197075-gpu/topic/Sorting.20revisited) Zulip thread.
Resources and measurements from implementations can be discussed there, then hopefully added to this wiki page.

[segmented sort]: https://moderngpu.github.io/segsort.html
[mergesort]: https://moderngpu.github.io/mergesort.html
[Onesweep]: https://arxiv.org/abs/2206.01784
[CUB]: https://nvlabs.github.io/cub/
[agent_radix_sort_onesweep]: https://github.com/NVIDIA/cub/blob/0fc3c3701632a4be906765b73be20a9ad0da603d/cub/agent/agent_radix_sort_onesweep.cuh
[FidelityFX sort]: https://gpuopen.com/fidelityfx-parallel-sort/
[warp-level multi-split]: https://arxiv.org/pdf/1701.01189.pdf
[splatter]: https://github.com/Lichtso/splatter
[splatter#2]: https://github.com/Lichtso/splatter/issues/2
[UnityGaussianSplatting]: https://github.com/aras-p/UnityGaussianSplatting
[UnityGaussianSplatting#82]: https://github.com/aras-p/UnityGaussianSplatting/pull/82
[Thomas Smith]: https://github.com/b0nes164/
[Hybrid Radix sort]: https://arxiv.org/abs/1611.01137
[Forma]: https://github.com/google/forma
[conveyor_sort]: https://github.com/google/forma/tree/main/forma/src/gpu/conveyor_sort
[Bitonic sort]: https://en.wikipedia.org/wiki/Bitonic_sorter
