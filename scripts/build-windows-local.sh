#!/usr/bin/env bash
# Local Windows .exe cross-build for Arch/Omarchy.
# Prerequisites (run once):
#   sudo pacman -S --needed mingw-w64-gcc mingw-w64-crt mingw-w64-headers mingw-w64-winpthreads
#   rustup target add x86_64-pc-windows-gnu
# A prebuilt shared FFmpeg (headers + import libs + DLLs) is downloaded below, so
# no AUR ffmpeg build is needed.
set -euo pipefail
cd "$(dirname "$0")/.."

source "$HOME/.cargo/env"
export PATH="$HOME/.local/bin:$PATH"

FFMPEG_ROOT="$HOME/.local/ffmpeg-windows"
if [ ! -d "$FFMPEG_ROOT/include/libavcodec" ]; then
  echo "Baixando FFmpeg compartilhado para Windows..."
  mkdir -p "$HOME/.local"
  curl -fL --retry 3 -o /tmp/ffmpeg-win64.zip \
    https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl-shared.zip
  rm -rf "$FFMPEG_ROOT"
  mkdir -p "$FFMPEG_ROOT"
  unzip -q /tmp/ffmpeg-win64.zip -d /tmp/ffmpeg-win-extract
  # O zip tem uma pasta ffmpeg-*/ dentro: copie o CONTEÚDO, não a pasta,
  # senão FFMPEG_DIR fica apontando para um nível acima dos headers.
  inner=$(find /tmp/ffmpeg-win-extract -mindepth 1 -maxdepth 1 -type d | head -1)
  if [ -n "$inner" ]; then
    cp -a "$inner"/. "$FFMPEG_ROOT"/
  else
    cp -a /tmp/ffmpeg-win-extract/. "$FFMPEG_ROOT"/
  fi
  rm -rf /tmp/ffmpeg-win-extract /tmp/ffmpeg-win64.zip
fi

# ffmpeg-sys-next reads FFMPEG_DIR (include/ + lib/) directly.
export FFMPEG_DIR="$FFMPEG_ROOT"
export CC_x86_64_pc_windows_gnu=x86_64-w64-mingw32-gcc
export CXX_x86_64_pc_windows_gnu=x86_64-w64-mingw32-g++

# bindgen needs the manually installed clang resource dir on this machine.
export LIBCLANG_PATH="$HOME/.local/libclang/libclang-18.1.1.data/platlib/clang/native"
# O wrapper do ffmpeg-sys-next suprime <cuda.h> mas o FFmpeg recente usa
# CUarray/CUDA_ARRAY3D_DESCRIPTOR; um stub local evita exigir o CUDA Toolkit.
# __AMXINTRIN_H evita que o immintrin.h do resource dir injete
# __tile1024i_str, que o bindgen 0.73 gera com repr packed+align conflitantes.
export BINDGEN_EXTRA_CLANG_ARGS="-resource-dir=$HOME/.local/clang-resource -D_GCC_LIMITS_H_ -D__AMX_FP16INTRIN_H -D__AMXINTRIN_H -D__AMX_COMPLEXINTRIN_H -include $PWD/scripts/cuda-bindgen-stub.h"

cargo build --locked --release -p xcloud-app --all-features --target x86_64-pc-windows-gnu

target_dir=$(cargo metadata --locked --no-deps --format-version 1 | python3 -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])')
exe="$target_dir/x86_64-pc-windows-gnu/release/openxcloud.exe"
rm -rf dist-windows
mkdir -p dist-windows
cp "$exe" dist-windows/openxcloud.exe
cp LICENSE dist-windows/LICENSE.txt
cp THIRD_PARTY_NOTICES.md dist-windows/THIRD_PARTY_NOTICES.md

# mingw runtime DLLs + every FFmpeg DLL the exe links against.
for dll in \
  /usr/x86_64-w64-mingw32/bin/libgcc_s_seh-1.dll \
  /usr/x86_64-w64-mingw32/bin/libstdc++-6.dll \
  /usr/x86_64-w64-mingw32/bin/libwinpthread-1.dll; do
  [ -f "$dll" ] && cp "$dll" dist-windows/
done
find "$FFMPEG_ROOT/bin" -maxdepth 1 -name '*.dll' -exec cp {} dist-windows/ \;

mkdir -p dist
(cd dist-windows && zip -q -r ../dist/NexaCloud-windows-x86_64.zip .)
cp dist/NexaCloud-windows-x86_64.zip "$HOME/Downloads/NexaCloud-windows-x86_64.zip"
echo "Pronto: ~/Downloads/NexaCloud-windows-x86_64.zip"
