package org.stateq.intermediate.decomposed

import org.stateq.compiler.CodeGenerator
import org.stateq.intermediate.DecomposedInstruction
import org.stateq.util.Location

class BeginDaggerInstruction(override val location: Location?) : DecomposedInstruction {
    override fun emit(codegen: CodeGenerator) {
        codegen.beginDagger()
    }
}

class EndDaggerInstruction(override val location: Location?) : DecomposedInstruction {
    override fun emit(codegen: CodeGenerator) {
        codegen.endDagger()
    }
}
