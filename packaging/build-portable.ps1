# Empaqueta la version portable de MDView (RF-22.1): el mismo ejecutable de
# `cargo build --release`, comprimido, sin ningun paso de instalacion.
$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
$exe = Join-Path $root "target\release\mdview.exe"
if (-not (Test-Path $exe)) {
    throw "No existe $exe. Ejecuta 'cargo build --release' primero."
}
$outDir = Join-Path $root "target\portable"
New-Item -ItemType Directory -Force -Path $outDir | Out-Null
$zip = Join-Path $outDir "mdview-portable.zip"
if (Test-Path $zip) { Remove-Item $zip }
Compress-Archive -Path $exe -DestinationPath $zip
Write-Host "Portable: $zip"
