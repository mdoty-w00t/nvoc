# Native Python Binding for `nvoc-core`

## Current Progress

This branch adds a new Rust workspace crate, `nvoc-python`, that builds the `pynvoc` Python package
with PyO3 and maturin. The package exposes a native extension module at `pynvoc._native` and a thin
Python package wrapper at `pynvoc`.

The first implementation focuses on direct `nvoc-core` access for GPU discovery, read-only queries,
simple set/reset operations, and static VFP curve import/export support. TUI migration work in #144
is the reference for moving UI actions from CLI subprocess calls to the `pynvoc` package. GUI
migration should follow the same native-service pattern for every non-autoscan path.

For now, GUI auto-optimize workflows remain CLI-backed. That includes the Autoscan tab, scan and
fix-result commands, helper import/export/reset actions in that workflow, streamed logs,
cancellation, stress-test orchestration, and CLI path selection.

## Implemented Package Shape

- Distribution name: `pynvoc`
- Python import name: `pynvoc`
- Native module name: `pynvoc._native`
- Rust crate/package: `pynvoc` in `nvoc-python/`
- Build backend: `maturin`
- Rust binding strategy: keep PyO3 isolated in `nvoc-python` so `nvoc-core` remains a Rust-first
  library without Python-specific dependencies.

## Implemented Python API

`pynvoc` promotes every registered `pynvoc._native` function into the top-level package API.
The exported bindings are:

- Discovery and general queries:
  - `discover_gpus`
  - `query_info`
  - `query_status`
  - `query_settings`
  - `query_supported_applications_clocks`
  - `query_clock_offset`
  - `query_domain_vfp_points`
  - `query_vfp_point_voltage`
  - `query_legacy_p0_core_max_voltage_delta`
  - `query_tdp_temp_limits`
  - `probe_voltage_limits`
  - `check_voltage_frequency`
- Clock, power, thermal, and fan controls:
  - `set_clock_offset`
  - `set_power_limit`
  - `set_thermal_limit`
  - `set_applications_clocks`
  - `reset_applications_clocks`
  - `set_locked_clocks`
  - `reset_locked_clocks`
  - `set_fan`
  - `reset_fan_speed`
  - `set_cooler_levels`
  - `reset_cooler_levels`
  - `set_legacy_clocks`
- P-state and VFP controls:
  - `set_pstate_base_voltage`
  - `reset_pstate_base_voltages`
  - `set_pstate_clock_offset`
  - `reset_pstate_clock_offsets`
  - `set_vfp_frequency_lock`
  - `reset_vfp_frequency_lock`
  - `set_vfp_voltage_lock`
  - `reset_vfp_lock`
  - `reset_vfp_deltas`
  - `set_vfp_point_delta`
  - `set_vfp_range_delta`
  - `set_domain_vfp_deltas`
- NVAPI/NVML-specific controls:
  - `set_nvapi_power_limits`
  - `reset_nvapi_power_limits`
  - `set_nvapi_sensor_limits`
  - `reset_nvapi_sensor_limits`
  - `set_nvapi_pstate_lock`
  - `set_nvml_pstate_lock`
  - `set_voltage_boost`
  - `set_legacy_voltage_delta`
- Convenience resets:
  - `reset_core_clocks`
  - `reset_mem_clocks`
  - `reset_all`

The package wrapper keeps `pynvoc.__all__` in parity with `pynvoc._native` so Python callers do not
need to import the private native module directly. Return values are normalized Python
dictionaries/lists converted from `serde_json::Value`.

## Validation and Alias Policy

User-facing validation should list every accepted alias. Current aliases include:

- Backend sets: `both`, `all`, `nvapi`, `nvml`
- Action backends: `nvapi`, `nvml`, `nvapi-cooler`, `nvml-cooler`
- Clock domains: `graphics`, `core`, `gpu`, `memory`, `mem`

Non-hardware validation tests cover these parsing paths so invalid values fail before GPU access.

## CI and Tests

The CI job for `nvoc-python` builds and tests the Rust package with:

- `cargo test --package pynvoc --no-default-features`
- `maturin develop --release`
- `pytest tests/`

The Python tests cover import/export contract, validation behavior that does not require GPU
hardware, and GPU smoke tests that should skip when no supported GPU is available.

## Remaining Work

- After #144 is merged, add a GUI native adapter/service modeled after the TUI `NativeService`: lazy
  import `pynvoc`, run short native actions off the Tk main thread, and report native errors to the
  GUI console.
- Add `pynvoc` as a GUI workspace dependency and route GUI GPU discovery, dashboard queries,
  info/status/settings refreshes, Overclock actions, Fan Control actions, and VF Curve
  non-autoscan actions through the native adapter.
- Keep the whole GUI Autoscan tab on the CLI for now, including scan/fix-result, workflow helper
  import/export/reset buttons, streamed output, cancellation, and CLI executable configuration.
- If #144 has not already provided it, add `query_domain_vfp_points` to `pynvoc` before migrating
  GUI VF Curve refresh/export logic away from CSV subprocess exports.
- Add GUI adapter tests with fake native bindings for discovery/query, fan, overclock, and VF Curve
  native success/error paths. Keep Autoscan tests CLI-oriented.
- On NVIDIA hardware, smoke-test discovery, info/status/settings queries, and one supervised
  read/write operation per backend.

## Assumptions

- The native binding remains additive during the initial migration.
- Non-autoscan GUI flows should surface native import/action failures as console errors rather than
  silently falling back to CLI behavior.
- CLI behavior remains available for auto-optimize workflows until a native progress/cancellation API
  exists.
- Hardware-mutating tests remain manual, ignored, or hardware-gated.
