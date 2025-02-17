# Notes to self

- Finished book 1
- Finished book 2

## Performance

### Other optimizations

#### TODO

- Change build function of BVH to better pick an axis
- Optimize BVH hit function
- Optimize bounding_box hit function
- Optimize sphere hit function
- Optimize triangle hit function

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
