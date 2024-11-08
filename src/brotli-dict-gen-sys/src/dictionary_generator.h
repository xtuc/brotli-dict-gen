#include "stdint.h"
#include "stddef.h"

const char* generate(
    const uint8_t* sample_data,
    const size_t* sample_sizes_ptr,
    size_t sample_sizes_len);

void free_result(void* ptr);
