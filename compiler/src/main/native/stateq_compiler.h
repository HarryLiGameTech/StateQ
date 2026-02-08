
#ifndef STATEQ_COMPILER
#define STATEQ_COMPILER

#ifdef __cplusplus
  extern "C" {
#endif

#include <stdlib.h>
#include <string.h>
#include <stdint.h>

struct KeyValueEntry
{
    char* key;
    char* value;
};

typedef struct KeyValueEntry TKeyValueEntry;

struct KeyValueEntryList
{
    uint32_t size;
    TKeyValueEntry* entries;
};

typedef struct KeyValueEntryList TKeyValueEntryList;

TKeyValueEntry* get_entry_from_list(TKeyValueEntryList* list, uint32_t index)
{
    return index > list->size ? NULL : &list->entries[index];
}

enum CompileErrorType
{
    ERROR = 0,
    WARNING = 1,
    NOTE = 2,
};

struct CompileError
{
    uint32_t err_type;
    char* source;
    int32_t line;
    int32_t column;
    char* message;
};

typedef struct CompileError TCompileError;

struct CompileErrorList
{
    uint32_t size;
    TCompileError* data;
};

typedef struct CompileErrorList TCompileErrorList;

struct CompileTargetList
{
    uint32_t size;
    char** data;
};

typedef struct CompileTargetList TCompileTargetList;

struct CompileResult
{
    TCompileTargetList targets;
    TCompileErrorList errors;
};

typedef struct CompileResult TCompileResult;

TCompileResult* create_compile_result(uint32_t n_targets, uint32_t n_errors)
{
    TCompileResult *compile_result = (TCompileResult*) malloc(sizeof(TCompileResult));
    compile_result->errors.size = n_errors;
    compile_result->errors.data = (TCompileError*) malloc(sizeof(TCompileError) * n_errors);
    compile_result->targets.size = n_targets;
    compile_result->targets.data = (char**) malloc(sizeof(char*) * n_targets);
    return compile_result;
}

void set_compile_error(TCompileResult* compile_result, uint32_t index,
    uint32_t type, char* source, int32_t line, int32_t column, char* message)
{
    TCompileError err = {
        .err_type = type,
        .source = (char*) malloc(strlen(source) + 1),
        .line = line,
        .column = column,
        .message = (char*) malloc(strlen(message) + 1),
    };
    strcpy(err.source, source);
    strcpy(err.message, message);
    compile_result->errors.data[index] = err;
}

void set_compile_target(TCompileResult* compile_result, uint32_t index, char* target)
{
    char* target_str = (char*) malloc(strlen(target) + 1);
    strcpy(target_str, target);
    compile_result->targets.data[index] = target_str;
}

typedef struct CompileResult TCompileResult;

#ifdef __cplusplus
  }
#endif

#endif // STATEQ_COMPILER
