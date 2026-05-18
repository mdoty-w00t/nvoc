use super::Error;
use nvapi_hi::Gpu;
use nvml_wrapper::Nvml;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GpuId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendSet {
    Nvapi,
    Nvml,
    Both,
}

#[derive(Clone, Copy)]
pub struct GpuTarget<'a> {
    pub id: GpuId,
    pub index: usize,
    pub nvapi: Option<&'a Gpu>,
    pub nvml: Option<&'a Nvml>,
}

impl<'a> GpuTarget<'a> {
    pub fn nvapi(&self) -> Result<&'a Gpu, Error> {
        self.nvapi
            .ok_or_else(|| Error::Custom(format!("GPU {} has no NvAPI backend", self.id.0)))
    }

    pub fn nvml(&self) -> Result<&'a Nvml, Error> {
        self.nvml
            .ok_or_else(|| Error::Custom(format!("GPU {} has no NVML backend", self.id.0)))
    }
}

pub struct TargetInventory {
    nvml: Option<Nvml>,
    nvapi_gpus: Vec<Gpu>,
    nvml_ids: Vec<u32>,
}

impl TargetInventory {
    pub fn discover(backends: BackendSet) -> Result<Self, Error> {
        let nvapi_gpus = match backends {
            BackendSet::Nvapi | BackendSet::Both => super::gpu::get_sorted_gpus()?,
            BackendSet::Nvml => Vec::new(),
        };

        let nvml = match backends {
            BackendSet::Nvml | BackendSet::Both => Some(
                Nvml::init().map_err(|e| Error::Custom(format!("NVML init failed: {:?}", e)))?,
            ),
            BackendSet::Nvapi => None,
        };

        let nvml_ids = match &nvml {
            Some(nvml) => super::gpu::get_sorted_gpu_ids_nvml(nvml)?,
            None => Vec::new(),
        };

        Ok(Self {
            nvml,
            nvapi_gpus,
            nvml_ids,
        })
    }

    pub fn targets(&self) -> Vec<GpuTarget<'_>> {
        let mut ids = self
            .nvapi_gpus
            .iter()
            .map(|gpu| gpu.id() as u32)
            .chain(self.nvml_ids.iter().copied())
            .collect::<Vec<_>>();
        ids.sort_unstable();
        ids.dedup();

        ids.into_iter()
            .enumerate()
            .map(|(index, id)| GpuTarget {
                id: GpuId(id),
                index,
                nvapi: self.nvapi_gpus.iter().find(|gpu| gpu.id() as u32 == id),
                nvml: self.nvml.as_ref(),
            })
            .collect()
    }
}

pub fn discover_targets(backends: BackendSet) -> Result<TargetInventory, Error> {
    TargetInventory::discover(backends)
}

pub fn select_targets<'a>(
    targets: &'a [GpuTarget<'a>],
    selector: &super::gpu::GpuSelector,
) -> Result<Vec<GpuTarget<'a>>, Error> {
    let ids = targets.iter().map(|target| target.id.0).collect::<Vec<_>>();
    let selected_ids = super::gpu::select_gpu_ids(&ids, selector)?;
    Ok(selected_ids
        .into_iter()
        .filter_map(|id| targets.iter().find(|target| target.id.0 == id).copied())
        .collect())
}
