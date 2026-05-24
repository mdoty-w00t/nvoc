#!/bin/bash
# Autoscan stressor interface wrapper for cli-stressor-cuda-rs.
# The autoscan calls: <exe> <test_code> <timeout_loops> [--aggressive-recovery] [extras...]
# This wrapper translates to cli-stressor-cuda-rs --duration <seconds>.
DURATION=$(( ${2:-10} * 6 ))
exec /usr/bin/cli-stressor-cuda-rs --duration "$DURATION"
