# Builds a self-contained Windows ZIP with the NexaCloud .exe.
# Run on windows-latest after installing ffmpeg through vcpkg (see desktop.yml).
$ErrorActionPreference = 'Stop'
Set-Location (Join-Path $PSScriptRoot '..')

cargo build --locked --release -p xcloud-app --all-features

$metadata = cargo metadata --locked --no-deps --format-version 1 | ConvertFrom-Json
$targetDir = $metadata.target_directory
$binary = Join-Path $targetDir 'release/openxcloud.exe'
if (-not (Test-Path $binary)) { throw "Missing binary: $binary" }

$stage = Join-Path $targetDir 'windows-stage'
if (Test-Path $stage) { Remove-Item -Recurse -Force $stage }
New-Item -ItemType Directory -Path $stage | Out-Null

Copy-Item $binary (Join-Path $stage 'openxcloud.exe')
Copy-Item 'LICENSE' (Join-Path $stage 'LICENSE.txt')
Copy-Item 'THIRD_PARTY_NOTICES.md' (Join-Path $stage 'THIRD_PARTY_NOTICES.md')

# vcpkg ffmpeg ships a handful of runtime DLLs; include them next to the exe so
# the ZIP runs without a separate ffmpeg installation.
$ffmpegBin = if ($env:FFMPEG_DIR) { Join-Path $env:FFMPEG_DIR 'bin' } else { Join-Path $env:VCPKG_INSTALLATION_ROOT 'installed/x64-windows/bin' }
if (Test-Path $ffmpegBin) {
  Get-ChildItem -Path $ffmpegBin -Filter '*.dll' | ForEach-Object {
    Copy-Item $_.FullName (Join-Path $stage $_.Name) -Force
  }
}

$zip = Join-Path (Resolve-Path '.') 'dist/NexaCloud-windows-x86_64.zip'
New-Item -ItemType Directory -Force -Path (Split-Path $zip) | Out-Null
if (Test-Path $zip) { Remove-Item -Force $zip }
Compress-Archive -Path (Join-Path $stage '*') -DestinationPath $zip
Write-Host "Created $zip"
