/* Stub mínimo para o bindgen do ffmpeg-sys-next: o hwcontext_wrapper.h do
 * crate define CUDA_VERSION (suprimindo <cuda.h>) mas só declara CUcontext e
 * CUstream. O hwcontext_cuda.h do FFmpeg recente também usa CUarray e
 * CUDA_ARRAY3D_DESCRIPTOR, que então ficavam sem tipo. Não é usado em runtime:
 * apenas satisfaz o bindgen sem exigir o CUDA Toolkit instalado. */
#ifndef OPENXCLOUD_CUDA_BINDGEN_STUB_H
#define OPENXCLOUD_CUDA_BINDGEN_STUB_H

#include <stddef.h>

typedef struct CUarray_st *CUarray;
typedef unsigned int CUarray_format;

typedef struct CUDA_ARRAY3D_DESCRIPTOR_st {
    size_t Width;
    size_t Height;
    size_t Depth;
    CUarray_format Format;
    unsigned int NumChannels;
    unsigned int Flags;
} CUDA_ARRAY3D_DESCRIPTOR;

#endif
