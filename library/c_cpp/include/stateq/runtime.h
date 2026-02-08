
#ifndef STATEQ_RUNTIME_H
#define STATEQ_RUNTIME_H

#ifdef __cplusplus
  #error "`stateq.h` is only for C, please use `stateq.hpp` in C++"
#endif

#include "qivm/runtime.h"

#include <stddef.h>
#include <stdint.h>
#include <assert.h>
#include <memory.h>
#include <math.h>

static const double pi = M_PI;

typedef union GateArgument
{
    double float_val;
    int64_t int_val;
}
GateArgument;

int64_t powi(int64_t base, int64_t exponent);
int64_t mpowi(int64_t base, int64_t exponent, int64_t mod);
double log2i(int64_t value);

typedef struct StateqBits
{
    size_t data_size;
    uint32_t* data;
}
StateqBits;

int uint32_count_ones(uint32_t value);
int stateq_bits_count_ones(StateqBits* bitset);
size_t stateq_get_size_of_bits(StateqBits bits);

typedef struct StateqBitsIterator
{
    StateqBits* bits;
    int block_offset;
    int bit_offset;
    int64_t next;
}
StateqBitsIterator;

StateqBitsIterator stateq_bits_iterator(StateqBits* bits);
bool stateq_bits_iterator_has_next_bit(StateqBitsIterator* iterator);
bool stateq_bits_iterator_next_bit(StateqBitsIterator* iterator);
bool stateq_bits_iterator_has_next(StateqBitsIterator* iterator);
int64_t stateq_bits_iterator_next(StateqBitsIterator* iterator);
int64_t stateq_get_index_of_bits(StateqBits bits, int index);

typedef struct StateqList
{
    size_t size;
    void* data;
}
StateqList;

#define stateq_get_list_item(Type, list, index) ( \
    assert(index < list->size),                     \
    ((Type*) list->data)[index]                     \
)

//StateqList stateq_list_new(size_t n, const size_t elem_size, ...);

RawMeasurementResult stateq_program_get_result_and_destroy(QuantumProgramContext* ctx);

#endif // STATEQ_RUNTIME_H
