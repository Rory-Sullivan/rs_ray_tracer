# Notes to self

- Finished book 1
- Currently on book 2: 8. Instances

## Performance

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
