@echo off
setlocal

set root="%~dp0..\"
pushd %root%

set trunkRoot=external\trunk-x86_64-pc-windows-msvc
set trunkExe=%trunkRoot%\trunk.exe

%trunkExe% serve

popd
endlocal