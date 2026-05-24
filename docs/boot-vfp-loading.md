# Loading VFP Curves at Boot (OpenRC, Gentoo)

The `auto-optimizer/openrc/` directory contains an OpenRC init script and conf file
that import saved VFP curves for each GPU at boot and reset them cleanly on shutdown.

## Files

| File | Install path | Purpose |
|------|-------------|---------|
| `auto-optimizer/openrc/nvoc-vfp` | `/etc/init.d/nvoc-vfp` | OpenRC init script |
| `auto-optimizer/openrc/nvoc-vfp.confd` | `/etc/conf.d/nvoc-vfp` | Per-GPU configuration |

## Setup (one-time)

### 1. Tune both GPUs first

Complete the autoscan workflow for each GPU and export the final curves:

```bash
sudo mkdir -p /etc/nvoc

# GPU 0 (ID 0x0A00)
nvoc-auto-optimizer --gpu 0x0A00 set vfp export -q /etc/nvoc/vfp-0x0A00.csv

# GPU 1 (ID 0x0B00)
nvoc-auto-optimizer --gpu 0x0B00 set vfp export -q /etc/nvoc/vfp-0x0B00.csv
```

GPU IDs are shown by `nvoc-auto-optimizer info` — look for the `ID:0x...` lines.

### 2. Install the init script

```bash
sudo cp auto-optimizer/openrc/nvoc-vfp /etc/init.d/nvoc-vfp
sudo chmod +x /etc/init.d/nvoc-vfp
sudo cp auto-optimizer/openrc/nvoc-vfp.confd /etc/conf.d/nvoc-vfp
```

### 3. Enable and test

```bash
sudo rc-update add nvoc-vfp default
sudo rc-service nvoc-vfp start
```

Check the output — each GPU logs a line like `GPU 0x0A00: importing VFP curve from ...`.

## Configuration (`/etc/conf.d/nvoc-vfp`)

```bash
NVOC_GPU_CONFIGS="0x0A00:/etc/nvoc/vfp-0x0A00.csv 0x0B00:/etc/nvoc/vfp-0x0B00.csv"
```

Space-separated `GPU_ID:CSV_PATH` pairs. Omit a GPU to leave its curve untouched at boot.
After editing, reload with `sudo rc-service nvoc-vfp restart`.

## Updating curves after re-tuning

Just re-export to the same paths — no reinstall needed:

```bash
nvoc-auto-optimizer --gpu 0x0A00 set vfp export -q /etc/nvoc/vfp-0x0A00.csv
nvoc-auto-optimizer --gpu 0x0B00 set vfp export -q /etc/nvoc/vfp-0x0B00.csv
```

## Behaviour

- **Start**: waits up to 30 s for `/dev/nvidiactl`, then imports each GPU's CSV in turn.
  Missing CSV files are warned and skipped; the service still reports success for the others.
- **Stop**: resets each GPU's VFP curve to factory defaults. This runs on shutdown/reboot,
  so overclocking is only active while the service is running.
- **Ordering**: depends on `udev-settle` so the NVIDIA driver is present before the import
  runs. If you use a display manager (GDM, SDDM, etc.), add `before xdm` to the `depend()`
  block in the init script so the curve loads before any GL context is created.

## Hardware (this machine)

| Slot | NVML index | NvAPI GPU ID | Model |
|------|-----------|-------------|-------|
| 0 | GPU 0 | `0x0A00` (2560) | RTX 5060 Ti (GB206-A) |
| 1 | GPU 1 | `0x0B00` (2816) | RTX 5060 Ti (GB206-A) |

CSV files live in `/etc/nvoc/`.

## Troubleshooting

**Service starts but curves don't apply after reboot**
Run `sudo rc-service nvoc-vfp start` manually and read the output. Check that the CSV
files exist at the paths in `NVOC_GPU_CONFIGS` and that `/usr/bin/nvoc-auto-optimizer`
is present.

**"nvoc-auto-optimizer: command not found" in the init script**
The binary path is hardcoded to `/usr/bin/nvoc-auto-optimizer`. If you installed it
elsewhere, edit `/etc/init.d/nvoc-vfp` and update the path.

**`/dev/nvidiactl` never appears**
The NVIDIA kernel module isn't loading before the service runs. Check `dmesg | grep nvidia`
and ensure the driver is installed and the module is in the autoload list
(`/etc/modules-load.d/` or `RC_NEED` / `modules-load` in OpenRC).
