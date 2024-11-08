#include "./dictionary_generator.h"
#include "../brotli/research/durchschlag.h"
#include "cstring"

const char* generate(
    const uint8_t* sample_data,
    const size_t* sample_sizes_ptr,
    size_t sample_sizes_len) {
  size_t targetSize = 16 << 10;
  size_t sliceLen = 16;
  size_t blockSize = 1024;

  std::vector<size_t> sample_sizes(sample_sizes_ptr, sample_sizes_ptr + sample_sizes_len);

  auto ret = durchschlag_generate(
      targetSize, sliceLen, blockSize, sample_sizes, sample_data);

  char* c_str = (char*) malloc(ret.size() + 1);
  std::strcpy(c_str, ret.c_str());

  return c_str;
}

void free_result(void* ptr) {
  free(ptr);
}
