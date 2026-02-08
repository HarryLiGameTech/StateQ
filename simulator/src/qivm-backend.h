
#ifndef QIVM_BACKEND_H
#define QIVM_BACKEND_H

#include <stdint.h>

#ifdef __cplusplus
  extern "C" {
#endif

typedef struct MeasurementResultEntry
{
    uint64_t value;
    uint64_t count;
}
MeasurementResultEntry;

typedef struct MeasurementResult
{
    uint64_t shots;
    uint64_t result_size;
    struct MeasurementResultEntry *measurements;
}
MeasurementResult;

typedef struct ExecuteResult
{
    uint8_t error;
    struct MeasurementResult measurement;
}
ExecuteResult;

uint32_t qivm_available_qubits();
bool qivm_is_gate_available(const char*);
struct ExecuteResult qivm_exec_bytecode(const uint8_t*, uint32_t, uint32_t);

#ifdef __cplusplus
  };
#endif

#endif // QIVM_BACKEND_H
