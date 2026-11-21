#!/usr/bin/env bash
# Build Linux de teste (NexaCloud). Configura o bindgen com o clang instalado
# manualmente nesta máquina — o mesmo esquema usado no build Windows.
set -euo pipefail
cd "$(dirname "$0")/.."
source "$HOME/.cargo/env"

export FFMPEG_DIR="${FFMPEG_DIR:-$HOME/Downloads/ffmpeg-master-latest-linux64-gpl-shared}"
export LIBCLANG_PATH="$HOME/.local/libclang/libclang-18.1.1.data/platlib/clang/native"
export BINDGEN_EXTRA_CLANG_ARGS="-resource-dir=$HOME/.local/clang-resource -D_GCC_LIMITS_H_ -D__AMX_FP16INTRIN_H -D__AMXINTRIN_H -D__AMX_COMPLEXINTRIN_H -include $PWD/scripts/cuda-bindgen-stub.h"

cargo build --locked --release -p xcloud-app "$@"
echo "Binário: target/release/openxcloud"
