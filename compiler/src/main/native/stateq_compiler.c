#include "stateq_compiler.h"
#include "libstateq.h"

TCompileResult* stateq_compile(char* src_path, TKeyValueEntryList* config)
{
    graal_isolate_t* isolate = NULL;
    graal_isolatethread_t* isolatethread = NULL;
    graal_create_isolate(NULL, &isolate, &isolatethread);
    TCompileResult* result = libstateq_compile(isolatethread, src_path, config);
    graal_tear_down_isolate(isolatethread);
    return result;
}
