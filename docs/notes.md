# Notes to self

- Finished book 1
- Finished book 2

## How to profile

- Use [samply](https://github.com/mstange/samply)
- Run with 'profiling' build

```
cargo build --profile profiling
samply record ./target/profiling/rs_ray_tracer.exe
```

## Performance

### Removing necessity for hittables to be clone-able

- Low:
  - Scene build:  0m 10s (10s)
  - Render: 0m 26s (26s)
  - Total: 0m 37s (37s)

### Removing dyn traits

Time to render at different resolutions BEFORE OPTIMIZATIONS,
generate_final_scene (with dragon), release mode, no debug.

- Low:
  - Scene build: 0m 11s (11s)
  - Render: 0m 31s (31s)
  - Total: 0m 42s (42s)
- Med:
  - Scene build: 0m 11s (11s)
  - Render: 2m 4s (124s)
  - Total: 2m 16s (136s)

Time to render at different resolutions AFTER REMOVING DYN TRAITS,
generate_final_scene (with dragon), release mode, no debug.

- Low:
  - Scene build: 0m 12s (12s)
  - Render: 0m 30s (30s)
  - Total: 0m 42s (42s)
- Med:
  - Scene build: 0m 11s (11s)
  - Render: 2m 5s (125s)
  - Total: 2m 17s (137s)

Time to render at different resolutions AFTER SWITCH TO ARC,
generate_final_scene (with dragon), release mode, no debug.

- Low:
  - Scene build: 0m 9s (9s)
  - Render: 0m 32s (32s)
  - Total: 0m 42s (42s)

Time to render at different resolutions AFTER BVHNode,
generate_final_scene (with dragon), release mode, no debug.

- Low:
  - Scene build: 0m 9s (9s)
  - Render: 0m 27s (27s)
  - Total: 0m 37s (37s)
- Med:
  - Scene build: 0m 9s (9s)
  - Render: 1m 51s (111s)
  - Total: 2m 0s (120s)

### BVH optimizations

Time to render at different resolutions BEFORE OPTIMIZATIONS,
generate_final_scene (with pyramid), release mode, no debug.

- Low:
  - BVH build: 0m 0s (0s)
  - Render: 0m 26s (26s)
  - Total: 0m 26s (26s)
- Med:
  - BVH build: 0m 0s (0s)
  - Render: 1m 43s (103s)
  - Total: 1m 43s (103s)
- High:
  - BVH build: 0m 0s (0s)
  - Render: 68m 35s (4115s)
  - Total: 68m 35s (4115s)

Time to render at different resolutions with BVH HIT and BOUNDING_BOX HIT OPTIMIZATIONS,
generate_final_scene (with pyramid), release mode, no debug.

- Low:
  - BVH build: 0m 0s (0s)
  - Render: 0m 23s (23s)
  - Total: 0m 23s (23s)

Time to render at different resolutions with BVH LONGEST AXIS OPTIMIZATIONS,
generate_final_scene (with pyramid), release mode, no debug.

- Low:
  - BVH build: 0m 0s (0s)
  - Render: 0m 24s (24s)
  - Total: 0m 24s (24s)
- Med:
  - BVH build: 0m 0s (0s)
  - Render: 1m 41s (101s)
  - Total: 1m 41s (101s)

### BVH

Time to render at different resolutions BEFORE BVH,
generate_random_complex_scene_moving_spheres, release mode, no debug.

- Low: 0m 19s (19s)
- Med: 6m 44s (404s)
- High: 23m 13s (1393s)

Time to render at different resolutions AFTER BVH,
generate_random_complex_scene_moving_spheres, release mode, no debug.

- Low:
  - BVH build: 0m 0s (0s)
  - Render: 0m 2s (2s)
  - Total: 0m 2s (2s)
- Med:
  - BVH build: 0m 0s (0s)
  - Render: 0m 51s (51s)
  - Total: 0m 51s (51s)
- High:
  - BVH build: 0m 0s (0s)
  - Render: 3m 8s (188s)
  - Total: 3m 8s (188s)
