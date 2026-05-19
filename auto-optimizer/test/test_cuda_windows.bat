@echo off
for /f %%a in ('echo prompt $E^| cmd') do set "ESC=%%a"

set /a "new_duration=%~2 * 5"

..\target\release\cli-stressor-cuda-rs.exe --precisions fp16 --matrix-sizes 2048,4096,8192 --duration %new_duration%
