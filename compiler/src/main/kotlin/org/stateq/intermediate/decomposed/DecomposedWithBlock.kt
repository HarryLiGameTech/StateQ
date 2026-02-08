package org.stateq.intermediate.decomposed

import org.stateq.compiler.CodeGenerator
import org.stateq.intermediate.DecomposedInstruction
import org.stateq.util.Location

class DecomposedWithBlock(
    val withExprBlock: DecomposedBasicBlock,
    val withBody: DecomposedBasicBlock,
    override val location: Location,
) : DecomposedInstruction {
    override fun emit(codegen: CodeGenerator) {
        codegen.withStatement({ withExprBlock.emit(this) }) {
            withBody.emit(this)
        }
    }
}
