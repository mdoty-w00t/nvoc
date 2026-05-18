mod conv;
mod error;
mod gpu;
mod gpu_type;
mod nvapi;
mod nvml;
pub mod operation;
pub mod result;
pub mod target;
mod types;

pub use conv::ConvertEnum;
pub use error::Error;
pub use gpu::GpuSelector;
pub use gpu_type::{GpuOcParams, GpuType, GpuVoltageLimitParams, GpuVoltageLockParams};
pub use operation::{
    CheckVoltageFrequency, GpuOperation, ProbeVoltageLimits, QueryClockOffset, QueryFanInfo,
    QueryPowerLimits, QueryPstates, QuerySupportedApplicationsClocks, QueryTdpTempLimits,
    QueryTemperatureThresholds, QueryVfpPointVoltage, ResetApplicationsClocks, ResetFanSpeed,
    ResetLockedClocks, ResetPstateBaseVoltages, ResetVfpDeltas, ResetVfpFrequencyLock,
    SetApplicationsClocks, SetClockOffset, SetCoolerLevels, SetFanSpeed, SetLegacyClocks,
    SetLockedClocks, SetNvapiPstateLock, SetNvmlPstateLock, SetPowerLimit, SetPstateBaseVoltage,
    SetPstateClockOffset, SetTemperatureLimit, SetVfpFrequencyLock, SetVfpPointDelta,
    SetVfpRangeDelta, SetVfpVoltageLock, detect_gpu_type, fetch_gpu_type, find_matching_vfp_point,
    nvml_pstate_to_index, nvml_pstate_to_str, parse_nvapi_locked_voltage_target,
    parse_nvml_fan_control_policy, parse_nvml_pstate, run, run_many, try_parse_nvml_pstate,
};
pub use result::{
    AppliedValue, BatchReport, ClockOffset, FanInfo, OperationKind, OperationReport,
    OperationWarning, PowerLimits, PstateClockRange, SupportedApplicationClocks, TargetOutcome,
    TdpTempLimits, TemperatureThreshold, VoltageFrequencyCheck, VoltageLimits,
};
pub use target::{BackendSet, GpuId, GpuTarget, TargetInventory, discover_targets, select_targets};
pub use types::{NvapiLockedVoltageTarget, VfpResetDomain};

#[doc(hidden)]
pub mod legacy {
    pub use crate::conv::{
        nvml_pstate_to_index, nvml_pstate_to_str, parse_nvml_pstate, try_parse_nvml_pstate,
    };
    pub use crate::gpu::{
        get_sorted_gpu_ids_nvml, get_sorted_gpus, select_gpu_ids, select_gpus, single_gpu,
    };
    pub use crate::gpu_type::{detect_gpu_type, fetch_gpu_type};
    pub use crate::nvapi::*;
    pub use crate::nvml::*;
}
