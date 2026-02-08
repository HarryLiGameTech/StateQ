package org.stateq.intermediate

import org.stateq.compiler.CodeGenerator
import org.stateq.compiler.CompiledModule
import org.stateq.module.classical.ConstantDef

class IntermediateModule(
    private val functions: List<IntermediateFunctionBase>,
    private val constants: List<ConstantDef>,
) : CompiledModule {

    private fun emit(codegen: CodeGenerator) {
        constants.forEach { codegen.defConstant(it) }
        functions.forEach { it.emit(codegen) }
    }

    override fun dumpCode(codegen: CodeGenerator): String {
        codegen.beginFile()
        this.emit(codegen)
        codegen.endFile()
        return codegen.dumpCode()
    }
}
