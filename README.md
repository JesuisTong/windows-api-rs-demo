# `windows-api-rs`

![https://github.com/napi-rs/package-template/actions](https://github.com/napi-rs/package-template/workflows/CI/badge.svg)

# Usage

1. Clone the project.
2. `pnpm i && cargo install && pnpm run build`

# benchmark
```
Running "getProcessExists node.exe" suite...
Progress: 100%

  Native getProcessExists exact node.exe:
    212 ops/s, ±2.08%   | 4.07% slower

  Native getProcessExists not exact node.exe:
    221 ops/s, ±1.05%   | fastest

  cmd getProcessExists node.exe:
    10 ops/s, ±1.00%    | slowest, 95.48% slower

Finished 3 cases!
  Fastest: Native getProcessExists not exact node.exe
  Slowest: cmd getProcessExists node.exe
Running "writeRegistry test" suite...
Progress: 100%

  Native writeRegistry test:
    103 938 ops/s, ±0.59%   | fastest

  cmd writeRegistry test:
    99 ops/s, ±0.72%        | slowest, 99.9% slower

Finished 2 cases!
  Fastest: Native writeRegistry test
  Slowest: cmd writeRegistry test
Running "readRegistry test" suite...

  Native readRegistry test:
    98 226 ops/s, ±0.42%   | fastest

  cmd readRegistry test:
    95 ops/s, ±0.72%       | slowest, 99.9% slower

Finished 2 cases!
  Fastest: Native readRegistry test
  Slowest: cmd readRegistry test
Running "deleteRegistry test" suite...
Progress: 100%

  Native deleteRegistry test:
    46 128 ops/s, ±0.44%   | fastest

  cmd deleteRegistry test:
    98 ops/s, ±0.81%       | slowest, 99.79% slower

Finished 2 cases!
  Fastest: Native deleteRegistry test
  Slowest: cmd deleteRegistry test
```
