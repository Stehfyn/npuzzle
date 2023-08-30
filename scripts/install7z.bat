@echo off
setlocal

set root="%~dp0..\"
pushd %root%

rem make tool cache
if not exist external\cache mkdir external\cache

rem Download and Install 7z
if not exist external\cache\7z2201-x64.msi (
    curl -L --show-error https://www.7-zip.org/a/7z2201-x64.msi -o external\cache\7z2201-x64.msi
)
if not exist external\7z2201-x64\Files\7-Zip\7z.exe (
    msiexec /a external\cache\7z2201-x64.msi /qn TARGETDIR="%CD%\external\7z2201-x64" /qb /l*v "install_7z.log"
)

popd
endlocal