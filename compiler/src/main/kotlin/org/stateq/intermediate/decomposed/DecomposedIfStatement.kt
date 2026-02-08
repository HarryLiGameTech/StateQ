package org.stateq.intermediate.decomposed

import org.stateq.expression.BoolExpr
import org.stateq.intermediate.DecomposedInstruction
import org.stateq.compiler.CodeGenerator
import org.stateq.util.Location

class DecomposedIfStatement(
    val condition: BoolExpr,
    val ifBranch: DecomposedBasicBlock,
    val elseBranch: DecomposedBasicBlock?,
    override val location: Location?,
) : DecomposedInstruction {
    override fun emit(codegen: CodeGenerator) {
        codegen.ifStatement(condition) {
            ifBranch.emit(this)
        }.elseBranch {
            elseBranch?.emit(codegen)
        }.transpile()
    }
}
