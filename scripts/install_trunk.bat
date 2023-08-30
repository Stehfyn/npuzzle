@echo off
setlocal

set root="%~dp0..\"
pushd %root%

set toolCache=external\cache
set trunkTarget=https://github.com/thedodd/trunk/releases/download/v0.17.5/trunk-x86_64-pc-windows-msvc.zip
set trunkZip=external\cache\trunk-x86_64-pc-windows-msvc.zip
set trunkRoot=external\trunk-x86_64-pc-windows-msvc
set trunkExe=%trunkRoot%\trunk.exe
set zipExe=external\7z2201-x64\Files\7-Zip\7z.exe

rem make tool cache
if not exist %toolCache% (
    mkdir %toolCache%
)

rem Download and Install 7z
if not exist %trunkZip% (
    curl -L --show-error %trunkTarget% -o %trunkZip%
)

if not exist %trunkExe% (
    if not exist %trunkRoot% (
        mkdir %trunkRoot%
    )

   "%zipExe%" e "%trunkZip%" -o"%trunkRoot%" -y
)

popd
endlocal