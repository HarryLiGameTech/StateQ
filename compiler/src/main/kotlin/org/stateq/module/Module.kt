package org.stateq.module

import org.stateq.intermediate.IntermediateFunctionBase
import org.stateq.intermediate.IntermediateModule
import org.stateq.module.classical.ConstantDef
import org.stateq.util.Locatable

interface Definition : Locatable
interface Scope

interface FunctionLikeDefinition : Definition {
    fun transpile(): IntermediateFunctionBase
}

class Module(val definitions: List<Definition>) {
    fun transpile(): IntermediateModule {
        val functions = definitions.filterIsInstance<FunctionLikeDefinition>().map { it.transpile() }
        val constants = definitions.filterIsInstance<ConstantDef>()
        return IntermediateModule(functions, constants)
    }
}
