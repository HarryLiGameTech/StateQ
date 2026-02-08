package org.stateq.compiler

interface CompiledModule {
    fun dumpCode(codegen: CodeGenerator): String
}
