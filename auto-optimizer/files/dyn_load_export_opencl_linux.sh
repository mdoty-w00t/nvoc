#!/bin/bash
# Dynamic load process for VFP export — applies GPU load so frequencies stabilise
# before the export captures the boost-clock baseline.
exec /usr/bin/cli-stressor-cuda-rs --duration 60
