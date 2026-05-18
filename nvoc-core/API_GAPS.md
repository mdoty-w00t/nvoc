# nvoc-core API gaps

This file tracks public functionality that has not been fully migrated to the
structured `GpuTarget` + `GpuOperation` + `OperationReport` API.

## Marked gaps

- `legacy::query_nvml_power_watts_by_pci`
  - Gap: consumes an ad hoc PCI string instead of a canonical `GpuTarget`.
  - Replacement: `run(target, QueryPowerLimits)`.

- `legacy::set_vfp_range_warn` and `legacy::set_vfp_curve_warn`
  - Gap: warning behavior is emitted through stderr and the functions do not
    return structured warning data.
  - Replacement: migrate callers to `SetVfpRangeDelta`; report warnings through
    `OperationReport::warnings` when per-point warning preservation is needed.

- `legacy::handle_test_voltage_limits`
  - Gap: workflow-shaped name and callback-based printing.
  - Replacement: `run(target, ProbeVoltageLimits)`.

- `legacy::get_gpu_tdp_temp_limit`
  - Gap: returns a tuple alias and prints while querying.
  - Replacement: `run(target, QueryTdpTempLimits)`. The output still mirrors the
    legacy tuple until the thermal throttle curve is represented by a stable
    public struct.

- `legacy::voltage_frequency_check`
  - Gap: callback-based printing and incomplete structured detail.
  - Replacement: `run(target, CheckVoltageFrequency { point })`; `matched_point`
    is currently `None` until the legacy function exposes that detail.

- `legacy::parse_nvml_pstate`
  - Gap: defaults invalid input to P0.
  - Replacement: top-level `parse_nvml_pstate`, which is fallible.

## Temporary compatibility boundary

The `legacy` module exists only so workspace crates can migrate incrementally
without wildcard root exports. New consumers should use the top-level operation
API.
