#include "stateq/runtime.h"

int64_t powi(int64_t base, int64_t exponent)
{
    int64_t result = 1;
    while (exponent > 0) {
        if (exponent & 1) { result *= base; }
        exponent >>= 1;
        base *= base;
    }
    return result;
}

int64_t mpowi(int64_t base, int64_t exponent, int64_t mod)
{
    int64_t result = 1;
    while (exponent > 0) {
        if (exponent & 1) { result = (result * base) % mod; }
        exponent >>= 1;
        base = (base * base) % mod;
    }
    return result;
}

double log2i(int64_t value)
{
    return log2((double) value);
}

RawMeasurementResult stateq_program_get_result_and_destroy(QuantumProgramContext* ctx)
{
    RawMeasurementResult result;
    result.result_size = 1024;
    result.measurements = malloc(sizeof(MeasurementResultEntry) * result.result_size);
    qivm_program_assign_result(ctx, &result);
    qivm_destroy_program_ctx(ctx);
    return result;
}

int uint32_count_ones(uint32_t value)
{
    value = value - ((value >> 1) & 0x55555555);
    value = (value & 0x33333333) + ((value >> 2) & 0x33333333);
    value = (value + (value >> 4)) & 0x0F0F0F0F;
    value = value + (value >> 8);
    value = value + (value >> 16);
    return (int) (value & 0x0000003F);
}

int stateq_bits_count_ones(StateqBits* bitset)
{
    int cnt_ones = 0;
    for (int i = 0; i < bitset->data_size; i++) {
        cnt_ones += uint32_count_ones(bitset->data[i]);
    }
    return cnt_ones;
}

size_t stateq_get_size_of_bits(StateqBits bits)
{
    return stateq_bits_count_ones(&bits);
}

StateqBitsIterator stateq_bits_iterator(StateqBits* bits)
{
    StateqBitsIterator iterator =
    {
        .bits = bits,
        .block_offset = 0,
        .bit_offset = 0,
        .next = -1
    };
    return iterator;
}

bool stateq_bits_iterator_has_next_bit(StateqBitsIterator* iterator)
{
    assert(iterator->bit_offset < 32);
    return (
        iterator->block_offset < iterator->bits->data_size - 1 ||
        (
            iterator->block_offset == iterator->bits->data_size - 1 &&
            iterator->bit_offset < 31
        )
    );
}

bool stateq_bits_iterator_next_bit(StateqBitsIterator* iterator)
{
    assert(stateq_bits_iterator_has_next_bit(iterator));
    if (iterator->bit_offset < 31) {
        iterator->block_offset += 1;
    } else // iterator->bit_offset == 31
    {
        iterator->block_offset = 0;
        iterator->block_offset += 1;
    }
    uint32_t block = iterator->bits->data[iterator->block_offset];
    return (block >> iterator->bit_offset) & 1;
}

bool stateq_bits_iterator_has_next(StateqBitsIterator* iterator)
{
    assert(iterator->next >= -2);
    if (iterator->next >= 0) {
        return true;
    }
    if (iterator->next == -2) {
        return false;
    } else if (iterator->next == -1) {
        while (stateq_bits_iterator_has_next_bit(iterator)) {
            if (stateq_bits_iterator_next_bit(iterator)) {
                iterator->next = iterator->block_offset * 32 + iterator->bit_offset;
                return true;
            }
        }
        return false;
    }
    return "unreachable";
}

int64_t stateq_bits_iterator_next(StateqBitsIterator* iterator)
{
    assert(stateq_bits_iterator_has_next(iterator));
    int64_t result = iterator->next;
    iterator->next = -1;
    return result;
}

int64_t stateq_get_index_of_bits(StateqBits bits, int index)
{
    StateqBitsIterator iterator = stateq_bits_iterator(&bits);
    for (int i = 1; i < index; i++) {
        stateq_bits_iterator_next(&iterator);
    }
    return stateq_bits_iterator_next(&iterator);
}

//StateqList stateq_list_new(size_t n, const size_t elem_size, ...)
//{
//    va_list args;
//    va_start(args, elem_size);
//    StateqList list =
//    {
//        .size = n,
//        .data = malloc(n * elem_size),
//    };
//    for (int i = 0; i < n; i++)
//    {
//        void* dest = list.data + i * elem_size;
//        memcpy(dest, va_arg(args, int8_t[elem_size]), elem_size);
//    }
//    return list;
//}
